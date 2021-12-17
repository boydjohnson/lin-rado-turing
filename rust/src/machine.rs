use crate::{
    program::{Action, Program},
    tape::{ITape, SSTape, Tape},
    types::{Direction, State, Symbol},
};
use itertools::{Either, EitherOrBoth::*, Itertools};
use rayon::prelude::*;
use std::{cmp::Ordering, collections::BTreeMap, io::Write};

type Beeps<S> = BTreeMap<S, usize>;
type Snapshots<S, Sym> = BTreeMap<Action<S, Sym>, Vec<(usize, usize, i64, Tape<Sym>, Beeps<S>)>>;

pub struct Machine<State, Symbol, T> {
    prog: Program<State, Symbol>,
    state: State,
    tape: T,

    halt: Option<Halt>,
}

impl<S: State + Send + Sync + ToString, Sym: Symbol + Send + Sync + ToString, T> Machine<S, Sym, T>
where
    T: ITape<Sym>,
{
    pub fn new(prog: Program<S, Sym>) -> Self {
        Self {
            prog,
            state: S::initial_state(),
            tape: T::default(),
            halt: None,
        }
    }

    fn read(&self) -> Sym {
        self.tape.read()
    }

    pub fn marks(&self) -> usize {
        self.tape.marks()
    }

    fn write(&mut self, direction: Direction, symbol: Sym) {
        self.tape.write_symbol(direction, symbol);
    }

    pub fn halt(&self) -> Option<&Halt> {
        self.halt.as_ref()
    }

    fn input_to_tape(&mut self, input: Vec<Sym>) {
        for s in input.into_iter() {
            self.write(Direction::Right, s);
        }
    }

    fn run_turing_step(&mut self, marks: &mut usize) -> bool {
        let read_symbol = self.read();
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

            self.tape.write_symbol(direction, symbol);
            true
        } else {
            false
        }
    }

    fn display_tape<B: Write>(&self, output: &mut Option<B>) {
        if let Some(b) = output {
            write!(b, "{}", self.tape).expect("Unable to write to output");
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

        self.display_tape(output);

        let mut marks = 0;

        let mut beeps: Beeps<S> = BTreeMap::new();

        for step in 0..=limit {
            if self.halt.is_some() {
                break;
            }

            beeps.insert(self.state, step);
            let notundefined = self.run_turing_step(&mut marks);

            self.display_tape(output);

            // Checks for stopping

            if !notundefined {
                let mut undfnd_str = self.state.to_string();

                undfnd_str.push_str(self.read().to_string().as_str());

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
) -> Machine<S, Sym, SSTape<Sym>> {
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
