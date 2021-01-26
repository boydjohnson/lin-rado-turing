use std::io::{BufRead, BufWriter, Write};

use crate::{
    program::Program,
    tape::Tape,
    types::{State, Symbol},
};

pub struct Machine<State, Symbol> {
    prog: Program<State, Symbol>,
    state: State,
    pos: i64,

    halt: Option<Halt>,
}

impl<S: State, Sym: Symbol> Machine<S, Sym> {
    pub fn new(prog: Program<S, Sym>) -> Self {
        Machine {
            prog,
            state: S::initial_state(),
            pos: 0,

            halt: None,
        }
    }

    fn read<'a>(&self, tape: &'a Tape<Sym>) -> Option<&'a Sym> {
        tape.read(self.pos)
    }

    fn write(&self, tape: &mut Tape<Sym>, symbol: Sym) {
        tape.write(self.pos, symbol)
    }

    fn move_left(&mut self) {
        self.pos -= 1;
    }

    fn move_right(&mut self) {
        self.pos += 1;
    }

    fn halt(&mut self) -> Option<Halt> {
        self.halt
    }

    fn input_to_tape(input: Vec<Sym>) -> Tape<Sym> {
        let mut tape = Tape::default();
        for (i, s) in input.into_iter().enumerate() {
            tape.write(i as i64, s);
        }
        tape
    }

    pub fn run_until_halt<B: Write>(
        &mut self,
        input: Vec<Sym>,
        limit: usize,
        mut output: Option<B>,
    ) {
        let mut tape = Self::input_to_tape(input);
        for step in 1..=limit {
            if self.state == S::halt() {
                self.halt = Some(Halt::new(step - 1, HaltReason::Halt));
                break;
            }

            let symbol = self.read(&tape).copied().unwrap_or_else(|| Sym::zero());
            let state = self.state;

            let (new_state, symbol, direction) = self.prog.instruction(state, symbol);
            if let Some(buffer) = &mut output {
                writeln!(
                    buffer,
                    "step: {}: state={:?}, symbol: {:?}",
                    step, new_state, symbol
                )
                .expect("Failed to write to stdout");
            }
            self.state = *new_state;

            self.write(&mut tape, *symbol);

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
