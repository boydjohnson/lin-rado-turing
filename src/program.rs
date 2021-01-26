use crate::types::{Direction, State, Symbol};
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug)]
pub struct Program<State, Symbol>(BTreeMap<(State, Symbol), (State, Symbol, Direction)>);

impl<S, Sym> Program<S, Sym>
where
    S: State,
    Sym: Symbol,
{
    pub const fn num(&self) -> (usize, usize) {
        (S::NUM_STATES, Sym::NUM)
    }

    pub fn instruction(&self, state: S, symbol: Sym) -> &(S, Sym, Direction) {
        self.0.get(&(state, symbol)).expect(
            "An unexpected state, symbol pair was asked for: possibly Halt state was entered",
        )
    }
}

impl<S, Sym> FromStr for Program<S, Sym>
where
    S: State,
    Sym: Symbol,
    ProgramParseError: From<<S as FromStr>::Err>,
    ProgramParseError: From<<Sym as FromStr>::Err>,
{
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, ProgramParseError> {
        let mut program_string_split = s.trim().split(" ");

        let mut inner = BTreeMap::new();

        for state in State::iter_states() {
            for symbol in Symbol::iter_symbols() {
                match program_string_split.next() {
                    Some(item) => {
                        let (sym, state_dir) = item.split_at(1);
                        let (direction, write_state) = state_dir.split_at(1);

                        inner.insert(
                            (state, symbol),
                            (
                                write_state.parse::<S>()?,
                                sym.parse::<Sym>()?,
                                direction.parse()?,
                            ),
                        );
                    }
                    None => {
                        return Err(ProgramParseError::Error(format!(
                            "Number of State, Symbols did not match up with program."
                        )))
                    }
                }
            }
        }
        Ok(Program(inner))
    }
}

#[derive(Debug, PartialEq)]
pub enum ProgramParseError {
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{TwoState, TwoSymbol};

    #[test]
    fn test_program_parse_2_2_info() {
        let program = "1RB 1LB 1LA 1RH".parse::<Program<TwoState, TwoSymbol>>();

        assert!(program.is_ok());

        let prog = program.unwrap();

        assert_eq!(
            *prog.instruction(TwoState::A, TwoSymbol::Zero),
            (TwoState::B, TwoSymbol::One, Direction::Right)
        );
    }
}
