use std::io::Write;

use crate::{
    program::Program,
    tape::Tape,
    types::{State, Symbol},
};

pub struct Machine<State, Symbol> {
    prog: Program<State, Symbol>,
    state: State,
    pos: i64,
    tape: Tape<Symbol>,

    halt: Option<Halt>,
}

impl<S: State, Sym: Symbol> Machine<S, Sym> {
    pub const fn num(&self) -> (usize, usize) {
        self.prog.num()
    }

    pub fn new(prog: Program<S, Sym>) -> Self {
        Machine {
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
        self.tape
            .iter()
            .filter(|item| Sym::zero() != **item)
            .count()
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

    pub fn halt(&mut self) -> Option<Halt> {
        self.halt
    }

    fn input_to_tape(&mut self, input: Vec<Sym>) {
        for (i, s) in input.into_iter().enumerate() {
            self.tape.write(i as i64, s);
        }
    }

    pub fn run_until_halt<B: Write>(
        &mut self,
        input: Vec<Sym>,
        limit: usize,
        output: &mut Option<B>,
    ) {
        self.input_to_tape(input);

        for step in 1..=limit {
            if self.state == S::halt() {
                self.halt = Some(Halt::new(step - 1, HaltReason::Halt));
                break;
            }

            let symbol = self.read().copied().unwrap_or_else(Sym::zero);
            let state = self.state;

            let &(new_state, symbol, direction) = self.prog.instruction(state, symbol);
            if let Some(buffer) = output {
                writeln!(
                    buffer,
                    "step: {}: state={:?}, symbol: {:?}",
                    step, new_state, symbol
                )
                .expect("Failed to write to stdout");
            }
            self.state = new_state;

            self.write(symbol);

            match direction {
                crate::types::Direction::Left => self.move_left(),
                crate::types::Direction::Right => self.move_right(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Halt {
    pub steps: usize,
    pub reason: HaltReason,
}

impl Halt {
    pub fn new(steps: usize, reason: HaltReason) -> Self {
        Halt { steps, reason }
    }

    pub fn is_halted(&self) -> bool {
        self.reason == HaltReason::Halt
    }

    pub fn is_lr_recurrence(&self) -> bool {
        self.reason == HaltReason::Recurr
    }

    pub fn is_limit(&self) -> bool {
        self.reason == HaltReason::XLimit
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HaltReason {
    Halt,
    Recurr,
    XLimit,
}

pub fn run_machine<S: State, Sym: Symbol>(
    program: Program<S, Sym>,
    prog_str: &str,
    limit: usize,
    mut output: Option<Box<dyn Write>>,
    _check_recurrence: bool,
) {
    let mut machine = Machine::new(program);

    machine.run_until_halt(vec![], limit, &mut output);

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
}
