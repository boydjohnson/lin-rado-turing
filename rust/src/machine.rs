use crate::{
    program::{Action, Program},
    tape::Tape,
    types::{State, Symbol},
};
use itertools::{Either, EitherOrBoth::*, Itertools};
use rayon::prelude::*;
use std::{cmp::Ordering, collections::BTreeMap, io::Write};

type Beeps<S> = BTreeMap<S, usize>;
type Snapshots<S, Sym> = BTreeMap<Action<S, Sym>, Vec<(usize, usize, i64, Tape<Sym>, Beeps<S>)>>;

pub struct Machine<State, Symbol> {
    prog: Program<State, Symbol>,
    state: State,
    pos: usize,
    tape: Tape<Symbol>,

    halt: Option<Halt>,
}

impl<S: State + Send + Sync + ToString, Sym: Symbol + Send + Sync + ToString> Machine<S, Sym> {
    pub fn new(prog: Program<S, Sym>) -> Self {
        Self {
            prog,
            state: S::initial_state(),
            pos: 0,
            tape: Tape::default(),
            halt: None,
        }
    }

    fn read(&self) -> Option<&Sym> {
        self.tape.read(self.pos)
    }

    pub fn marks(&self) -> usize {
        self.tape.iter().filter(|item| Sym::zero() != *item).count()
    }

    fn write(&mut self, symbol: Sym) {
        self.tape.write(self.pos, symbol)
    }

    fn move_left(&mut self) {
        self.pos -= 1;
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }

    pub fn halt(&self) -> Option<&Halt> {
        self.halt.as_ref()
    }

    fn input_to_tape(&mut self, input: Vec<Sym>) {
        for (i, s) in input.into_iter().enumerate() {
            self.tape.write(i, s);
        }
    }

    fn recurr_check_init() -> (Snapshots<S, Sym>, Vec<i64>) {
        (BTreeMap::new(), vec![])
    }

    fn par_min_deviations(deviations: &[i64], dev: i64, pstep: usize) -> i64 {
        deviations[pstep..].par_iter().min().copied().unwrap_or(dev)
    }

    fn par_max_deviations(deviations: &[i64], dev: i64, pstep: usize) -> i64 {
        deviations[pstep..].par_iter().max().copied().unwrap_or(dev) + 1
    }

    fn par_recurr_check(
        &mut self,
        step: usize,
        snaps: &mut Snapshots<S, Sym>,
        deviations: &[i64],
        init: usize,
        beeps: &Beeps<S>,
        dev: i64,
    ) -> Option<Halt> {
        let action = (self.state, self.read().copied().unwrap_or_else(Sym::zero));

        if let Some(items) = snaps.get(&action).cloned() {
            let iter = items.par_iter();

            if let Some((pstep, step, pbeeps, ptape)) = iter
                .filter_map(|(pstep, pinit, pdev, ptape, pbeeps)| {
                    let (prev, curr) = match dev.cmp(pdev) {
                        std::cmp::Ordering::Less => {
                            let dmax = Self::par_max_deviations(deviations, dev, *pstep);

                            let mut prev = ptape
                                .iter_to((*pinit as i64 + dmax) as usize)
                                .collect::<Vec<_>>();

                            let mut curr = self
                                .tape
                                .iter_to((init as i64 + dmax + dev - *pdev) as usize)
                                .collect::<Vec<_>>();

                            match curr.len().cmp(&prev.len()) {
                                Ordering::Greater => {
                                    let mut prep = (0..(curr.len() - prev.len()))
                                        .map(|_| Sym::zero())
                                        .collect::<Vec<_>>();
                                    prep.append(&mut prev);
                                    prev = prep;
                                }
                                Ordering::Less => {
                                    let mut prep = (0..(prev.len() - curr.len()))
                                        .map(|_| Sym::zero())
                                        .collect::<Vec<_>>();
                                    prep.append(&mut curr);
                                    curr = prep;
                                }
                                Ordering::Equal => (),
                            }

                            (prev, curr)
                        }
                        Ordering::Greater => {
                            let dmin = Self::par_min_deviations(deviations, dev, *pstep);

                            let from_prev = *pinit as i64 + dmin;

                            let mut prev = ptape.iter_from(from_prev).collect::<Vec<_>>();

                            let from_curr = init as i64 + dmin + dev - pdev;

                            let mut curr = self.tape.iter_from(from_curr).collect::<Vec<_>>();

                            match curr.len().cmp(&prev.len()) {
                                Ordering::Greater => {
                                    let mut app = (0..(curr.len() - prev.len()))
                                        .map(|_| Sym::zero())
                                        .collect::<Vec<_>>();
                                    prev.append(&mut app);
                                }
                                Ordering::Less => {
                                    let mut app = (0..(prev.len() - curr.len()))
                                        .map(|_| Sym::zero())
                                        .collect::<Vec<_>>();
                                    curr.append(&mut app);
                                }
                                Ordering::Equal => (),
                            }

                            (prev, curr)
                        }
                        Ordering::Equal => {
                            let dmax = Self::par_max_deviations(deviations, dev, *pstep);
                            let dmin = Self::par_min_deviations(deviations, dev, *pstep);

                            let from_prev = *pinit as i64 + dmin;

                            let prev = ptape
                                .iter_between(from_prev, *pinit as i64 + dmax)
                                .collect::<Vec<_>>();

                            let from_curr = init as i64 + dmin;

                            let curr = self
                                .tape
                                .iter_between(from_curr, init as i64 + dmax)
                                .collect::<Vec<_>>();

                            (prev, curr)
                        }
                    };

                    if prev == curr {
                        Some((pstep, step, pbeeps, ptape))
                    } else {
                        None
                    }
                })
                .min_by_key(|&(pstep, _, _, _)| pstep)
            {
                self.tape = ptape.clone();

                let reason = if pbeeps
                    .keys()
                    .all(|state| beeps.get(state) > pbeeps.get(state))
                {
                    HaltReason::Recurr
                } else {
                    HaltReason::Quasihalt
                };

                return Some(Halt::new(*pstep, reason(step - pstep)));
            }
        }

        snaps
            .entry(action)
            .and_modify(|v| v.push((step, init, dev, self.tape.clone(), beeps.clone())))
            .or_insert_with(|| vec![(step, init, dev, self.tape.clone(), beeps.clone())]);
        None
    }

    fn recurr_check(
        &mut self,
        step: usize,
        snaps: &mut Snapshots<S, Sym>,
        deviations: &[i64],
        init: usize,
        beeps: &Beeps<S>,
        dev: i64,
    ) -> Option<Halt> {
        let action = (self.state, self.read().copied().unwrap_or_else(Sym::zero));

        let mut iter = if let Some(items) = snaps.get(&action) {
            Either::Right(items.iter())
        } else {
            Either::Left(std::iter::empty())
        };

        if let Some((pstep, step, pbeeps, ptape)) = loop {
            if let Some((pstep, pinit, pdev, ptape, pbeeps)) = iter.next() {
                let (prev, curr) = match dev.cmp(pdev) {
                    std::cmp::Ordering::Less => {
                        let dmax = deviations[*pstep..].iter().max().copied().unwrap_or(dev) + 1;

                        let to_prev = (*pinit as i64 + dmax) as usize;

                        let mut prev =
                            Box::new(ptape.iter_to(to_prev)) as Box<dyn Iterator<Item = Sym>>;

                        let to_curr = (init as i64 + dmax + dev - *pdev) as usize;

                        let mut curr =
                            Box::new(self.tape.iter_to(to_curr)) as Box<dyn Iterator<Item = Sym>>;

                        match to_curr.cmp(&to_prev) {
                            Ordering::Greater => {
                                let prep = (0..(to_curr - to_prev)).map(|_| Sym::zero());

                                prev = Box::new(prep.chain(prev)) as Box<dyn Iterator<Item = Sym>>;
                            }
                            Ordering::Less => {
                                let prep = (0..(to_prev - to_curr)).map(|_| Sym::zero());

                                curr = Box::new(prep.chain(curr)) as Box<dyn Iterator<Item = Sym>>;
                            }
                            Ordering::Equal => (),
                        }

                        (OfThree::One(prev), OfThree::One(curr))
                    }
                    Ordering::Greater => {
                        let dmin = deviations[*pstep..].iter().min().copied().unwrap_or(dev);

                        let from_prev = *pinit as i64 + dmin;

                        let prev = ptape.iter_from(from_prev);

                        let from_curr = init as i64 + dmin + dev - pdev;

                        let curr = self.tape.iter_from(from_curr);

                        (OfThree::Two(prev), OfThree::Two(curr))
                    }
                    Ordering::Equal => {
                        let dmax = deviations[*pstep..].iter().max().copied().unwrap_or(dev) + 1;
                        let dmin = deviations[*pstep..].iter().min().copied().unwrap_or(dev);

                        let from_prev = *pinit as i64 + dmin;

                        let prev = ptape.iter_between(from_prev, *pinit as i64 + dmax);

                        let from_curr = init as i64 + dmin;

                        let curr = self.tape.iter_between(from_curr, init as i64 + dmax);

                        (OfThree::Three(prev), OfThree::Three(curr))
                    }
                };

                if prev.zip_longest(curr).all(|both| match both {
                    Both(p, c) => p == c,
                    Left(l) => l == Sym::zero(),
                    Right(r) => r == Sym::zero(),
                }) {
                    break Some((*pstep, step, pbeeps, ptape));
                }
            } else {
                break None;
            }
        } {
            self.tape = ptape.clone();

            let reason = if pbeeps
                .keys()
                .all(|state| beeps.get(state) > pbeeps.get(state))
            {
                HaltReason::Recurr
            } else {
                HaltReason::Quasihalt
            };

            return Some(Halt::new(pstep, reason(step - pstep)));
        }

        snaps
            .entry(action)
            .and_modify(|v| v.push((step, init, dev, self.tape.clone(), beeps.clone())))
            .or_insert_with(|| vec![(step, init, dev, self.tape.clone(), beeps.clone())]);
        None
    }

    fn write_tape<B: Write>(&self, output: &mut Option<B>, step: usize) {
        if let Some(b) = output {
            let mut buffer = format!("{:8} {:?}  ", step, self.state);

            let tape_iter = self.tape.iter();

            for (idx, s) in tape_iter.enumerate() {
                if idx == self.pos {
                    buffer.push('[');
                }
                buffer.push_str(&s.to_string());
                if idx == self.pos {
                    buffer.push(']')
                }
            }

            writeln!(b, "{}", buffer).unwrap();
        }
    }

    fn run_turing_step(&mut self, init: &mut usize, marks: &mut usize) -> bool {
        let read_symbol = self.read().copied().unwrap_or_else(Sym::zero);
        let state = self.state;

        if let Some(&(new_state, symbol, direction)) = self.prog.instruction(state, read_symbol) {
            self.state = new_state;

            if (Sym::zero(), Sym::zero()) == (read_symbol, symbol) {
            } else if Sym::zero() == read_symbol && Sym::zero() != symbol {
                *marks += 1;
            } else if Sym::zero() != read_symbol && Sym::zero() == symbol {
                *marks -= 1;
            } else {
            }

            self.write(symbol);

            match direction {
                crate::types::Direction::Left => {
                    if self.pos == 0 {
                        *init += 1;
                        self.tape.insert();
                    } else {
                        self.move_left();
                    }
                }
                crate::types::Direction::Right => self.move_right(),
            }
            true
        } else {
            false
        }
    }

    pub fn run_until_halt<B: Write>(
        &mut self,
        input: Vec<Sym>,
        limit: usize,
        output: &mut Option<B>,
        check_recurrence: Option<usize>,
        check_blank: Option<usize>,
        parallel: bool,
    ) {
        self.input_to_tape(input);
        let mut init = self.tape.size() / 2;

        self.pos = init;

        let mut marks = 0;

        let mut beeps: Beeps<S> = BTreeMap::new();

        let (mut snapshots, mut deviations) = if check_recurrence.is_some() {
            let f = Self::recurr_check_init();
            (Some(f.0), Some(f.1))
        } else {
            (None, None)
        };

        for step in 0..=limit {
            self.write_tape(output, step);

            let dev = self.pos as i64 - init as i64;

            if let Some(ref mut devs) = deviations {
                devs.push(dev);
            }

            if let (Some(start), Some(snaps), Some(deviations)) =
                (check_recurrence, &mut snapshots, &deviations)
            {
                if step >= start {
                    if parallel {
                        self.halt =
                            self.par_recurr_check(step, snaps, deviations, init, &beeps, dev);
                    } else {
                        self.halt = self.recurr_check(step, snaps, deviations, init, &beeps, dev);
                    }
                }
            }
            if self.halt.is_some() {
                break;
            }

            beeps.insert(self.state, step);
            let notundefined = self.run_turing_step(&mut init, &mut marks);

            // Checks for stopping

            if !notundefined {
                let mut undfnd_str = self.state.to_string();

                undfnd_str.push_str(
                    self.read()
                        .copied()
                        .unwrap_or_else(Sym::zero)
                        .to_string()
                        .as_str(),
                );

                self.halt = Some(Halt::new(step + 1, HaltReason::Undefined(undfnd_str)));
                break;
            }

            if let Some(s) = check_blank {
                if s <= step && marks == 0 {
                    self.halt = Some(Halt::new(step + 1, HaltReason::Blanking));
                    break;
                }
            }

            if self.state == S::halt() {
                self.halt = Some(Halt::new(step + 1, HaltReason::Halt));
                break;
            }
        }

        if self.halt.is_none() {
            self.halt = Some(Halt::new(limit, HaltReason::XLimit));
        }
    }
}

pub enum OfThree<I1, I2, I3> {
    One(I1),
    Two(I2),
    Three(I3),
}

impl<T, I1: Iterator<Item = T>, I2: Iterator<Item = T>, I3: Iterator<Item = T>> Iterator
    for OfThree<I1, I2, I3>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use OfThree::*;
        match self {
            One(i) => i.next(),
            Two(i) => i.next(),
            Three(i) => i.next(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Halt {
    pub steps: usize,
    pub reason: HaltReason,
}

impl Halt {
    pub const fn new(steps: usize, reason: HaltReason) -> Self {
        Self { steps, reason }
    }

    pub const fn is_halted(&self) -> bool {
        matches!(self.reason, HaltReason::Halt)
    }

    pub const fn is_lr_recurrence(&self) -> bool {
        matches!(self.reason, HaltReason::Recurr(_))
    }

    pub const fn is_limit(&self) -> bool {
        matches!(self.reason, HaltReason::XLimit)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HaltReason {
    Halt,
    Recurr(usize),
    XLimit,
    Quasihalt(usize),
    Blanking,
    Undefined(String),
}

#[allow(clippy::too_many_arguments)]
pub fn run_machine<S: State + Send + Sync + ToString, Sym: Symbol + Send + Sync + ToString>(
    program: Program<S, Sym>,
    prog_str: &str,
    limit: usize,
    mut output: Option<Box<dyn Write>>,
    verbose: bool,
    check_recurrence: Option<usize>,
    check_blank: Option<usize>,
    parallel: bool,
) -> Machine<S, Sym> {
    let mut machine = Machine::new(program);

    if verbose {
        machine.run_until_halt(
            vec![],
            limit,
            &mut output,
            check_recurrence,
            check_blank,
            parallel,
        );
    } else {
        machine.run_until_halt::<std::io::Stdout>(
            vec![],
            limit,
            &mut None,
            check_recurrence,
            check_blank,
            parallel,
        );
    }

    if let Some(halt) = machine.halt() {
        if let Some(w) = &mut output {
            if let Err(e) = writeln!(
                w,
                "{}: marks {} steps {} reason {:?}",
                prog_str,
                machine.marks(),
                halt.steps,
                halt.reason
            ) {
                writeln!(std::io::stderr(), "Error writing: {}", e)
                    .expect("Unable to write to stderr");
            }
        }
    }
    machine
}
