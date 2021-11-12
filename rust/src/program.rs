use crate::{
    parse::parse_instructions,
    types::{Direction, State, StateT, Symbol, SymbolT},
};
use std::{collections::BTreeMap, convert::TryFrom};

pub type Action<S, Sym> = (S, Sym);
pub type Instruction<S, Sym> = (S, Sym, Direction);

#[derive(Debug)]
pub struct Program<State, Symbol>(
    BTreeMap<Action<State, Symbol>, Option<Instruction<State, Symbol>>>,
);

impl<S: State, Sym: Symbol> Program<S, Sym> {
    pub fn instruction(&self, state: S, symbol: Sym) -> Option<&Instruction<S, Sym>> {
        self.0
            .get(&(state, symbol))
            .expect(
                "An unexpected state, symbol pair was asked for: possibly Halt state was entered",
            )
            .as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgramParseError(pub String);

#[derive(Debug)]
pub enum ProgramT {
    TwoTwo(Program<StateT<'B'>, SymbolT<'1'>>),
    TwoThree(Program<StateT<'B'>, SymbolT<'2'>>),
    TwoFour(Program<StateT<'B'>, SymbolT<'3'>>),
    ThreeTwo(Program<StateT<'C'>, SymbolT<'1'>>),
    ThreeThree(Program<StateT<'C'>, SymbolT<'2'>>),
    ThreeFour(Program<StateT<'C'>, SymbolT<'3'>>),
    FourTwo(Program<StateT<'D'>, SymbolT<'1'>>),
    FourThree(Program<StateT<'D'>, SymbolT<'2'>>),
    FourFour(Program<StateT<'D'>, SymbolT<'3'>>),
    FiveTwo(Program<StateT<'E'>, SymbolT<'1'>>),
    FiveThree(Program<StateT<'E'>, SymbolT<'2'>>),
    FiveFour(Program<StateT<'E'>, SymbolT<'3'>>),
    SixTwo(Program<StateT<'F'>, SymbolT<'1'>>),
    SixThree(Program<StateT<'F'>, SymbolT<'2'>>),
    SixFour(Program<StateT<'F'>, SymbolT<'3'>>),
}

pub fn parse_program(prog_str: &str) -> Result<ProgramT, ProgramParseError> {
    let (rest, instructions) = match parse_instructions(prog_str) {
        Ok(inner) => inner,
        Err(e) => {
            return Err(ProgramParseError(format!(
                "Error parsing program string({}): {}",
                prog_str, e
            )));
        }
    };
    if !rest.is_empty() {
        return Err(ProgramParseError(format!(
            "Unexpected end of string: {}",
            rest
        )));
    }
    let color = if let Some(first) = instructions.first() {
        first.len()
    } else {
        return Err(ProgramParseError("Unexpected missing instructions".into()));
    };

    if !instructions.iter().all(|inst| inst.len() == color) {
        return Err(ProgramParseError(format!(
            "All states must have {} color instructions, which the first state did.",
            color
        )));
    }

    let state = instructions.len();

    match (state, color) {
        (2, 2) => construct_program(instructions, ProgramT::TwoTwo),
        (2, 3) => construct_program(instructions, ProgramT::TwoThree),
        (2, 4) => construct_program(instructions, ProgramT::TwoFour),
        (3, 2) => construct_program(instructions, ProgramT::ThreeTwo),
        (3, 3) => construct_program(instructions, ProgramT::ThreeThree),
        (3, 4) => construct_program(instructions, ProgramT::ThreeFour),
        (4, 2) => construct_program(instructions, ProgramT::FourTwo),
        (4, 3) => construct_program(instructions, ProgramT::FourThree),
        (4, 4) => construct_program(instructions, ProgramT::FourFour),
        (5, 2) => construct_program(instructions, ProgramT::FiveTwo),
        (5, 3) => construct_program(instructions, ProgramT::FiveThree),
        (5, 4) => construct_program(instructions, ProgramT::FiveFour),
        (6, 2) => construct_program(instructions, ProgramT::SixTwo),
        (6, 3) => construct_program(instructions, ProgramT::SixThree),
        (6, 4) => construct_program(instructions, ProgramT::SixFour),
        (s, c) => Err(ProgramParseError(format!(
            "State must be (2-6) inclusive, color must be (2-4) inclusive not state={}, color={}",
            s, c
        ))),
    }
}

fn construct_program<
    F: Fn(Program<StateT<S>, SymbolT<C>>) -> ProgramT,
    const S: char,
    const C: char,
>(
    instructions: Vec<Vec<(char, char, char)>>,
    func: F,
) -> Result<ProgramT, ProgramParseError> {
    let mut inner = BTreeMap::default();
    for ((state, symbol), (sym, dir, st)) in instructions
        .into_iter()
        .zip(<StateT<S> as State>::states().into_iter())
        .flat_map(|(instr, state)| {
            instr
                .into_iter()
                .zip(<SymbolT<C> as Symbol>::symbols().into_iter())
                .map(move |(i, sym)| ((state, sym), i))
        })
    {
        if sym == '.' && st == '.' && dir == '.' {
            inner.insert((state, symbol), None);
        } else {
            let sym = SymbolT::<C>::try_from(sym)?;
            let st = StateT::<S>::try_from(st)?;
            let dir = Direction::try_from(dir)?;

            inner.insert((state, symbol), Some((st, sym, dir)));
        }
    }
    Ok(func(Program(inner)))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::types::{StateT, SymbolT};

    #[test]
    fn test_program_parse_2_2_info() {
        let program = parse_program("1RB 1LB  1LA 1RH");

        assert!(program.is_ok());

        let prog = program.unwrap();

        assert!(matches!(prog, ProgramT::TwoTwo(_)));

        if let ProgramT::TwoTwo(prog) = prog {
            assert_eq!(
                prog.instruction(StateT::<'B'>::Val('A'), SymbolT::<'1'>('0')),
                Some(&(
                    StateT::<'B'>::Val('B'),
                    SymbolT::<'1'>('1'),
                    Direction::Right
                ))
            );
        }
    }

    #[test]
    fn test_program_parse_too_many() {
        let program = parse_program("1RB 0LA  1RB 0LA  1LB");

        assert!(program.is_err());
    }

    #[test]
    fn test_program_undefined() {
        let program = parse_program("1RB ...  1LA 1RH");

        assert!(program.is_ok());

        let prog = program.unwrap();

        assert!(matches!(prog, ProgramT::TwoTwo(_)));
        if let ProgramT::TwoTwo(prog) = prog {
            assert_eq!(
                prog.instruction(StateT::<'B'>::Val('A'), SymbolT::<'1'>('1')),
                None
            );
        }
    }
}
