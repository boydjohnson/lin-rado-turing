use crate::types::{
    Direction, FiveState, FourState, FourSymbol, SixState, State, Symbol, ThreeState, ThreeSymbol,
    TwoState, TwoSymbol,
};
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug)]
pub struct Program<State, Symbol>(BTreeMap<(State, Symbol), (State, Symbol, Direction)>);

impl<S: State, Sym: Symbol> Program<S, Sym> {
    pub fn instruction(&self, state: S, symbol: Sym) -> &(S, Sym, Direction) {
        self.0.get(&(state, symbol)).expect(
            "An unexpected state, symbol pair was asked for: possibly Halt state was entered",
        )
    }
}

impl<S: State, Sym: Symbol> FromStr for Program<S, Sym>
where
    ProgramParseError: From<<S as FromStr>::Err> + From<<Sym as FromStr>::Err>,
{
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, ProgramParseError> {
        let mut program_string_split = s.trim().split(' ');

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
                        return Err(ProgramParseError::Error(
                            "Number of State, Symbols did not match up with program.".to_string(),
                        ))
                    }
                }
            }
        }
        if program_string_split.count() > 0 {
            return Err(ProgramParseError::Error(
                "Too many program instructions".into(),
            ));
        }

        Ok(Self(inner))
    }
}

#[derive(Debug, PartialEq)]
pub enum ProgramParseError {
    Error(String),
}

pub enum ProgramT {
    TwoTwo(Program<TwoState, TwoSymbol>),
    TwoThree(Program<TwoState, ThreeSymbol>),
    TwoFour(Program<TwoState, FourSymbol>),
    ThreeTwo(Program<ThreeState, TwoSymbol>),
    ThreeThree(Program<ThreeState, ThreeSymbol>),
    ThreeFour(Program<ThreeState, FourSymbol>),
    FourTwo(Program<FourState, TwoSymbol>),
    FourThree(Program<FourState, ThreeSymbol>),
    FourFour(Program<FourState, FourSymbol>),
    FiveTwo(Program<FiveState, TwoSymbol>),
    FiveThree(Program<FiveState, ThreeSymbol>),
    FiveFour(Program<FiveState, FourSymbol>),
    SixTwo(Program<SixState, TwoSymbol>),
    SixThree(Program<SixState, ThreeSymbol>),
    SixFour(Program<SixState, FourSymbol>),
}

pub fn parse_program(prog_str: &str, complexity: &str) -> Result<ProgramT, ProgramParseError> {
    match complexity {
        "2-2" => prog_str.parse().map(ProgramT::TwoTwo),
        "2-3" => prog_str.parse().map(ProgramT::TwoThree),
        "2-4" => prog_str.parse().map(ProgramT::TwoFour),
        "3-2" => prog_str.parse().map(ProgramT::ThreeTwo),
        "3-3" => prog_str.parse().map(ProgramT::ThreeThree),
        "3-4" => prog_str.parse().map(ProgramT::ThreeFour),
        "4-2" => prog_str.parse().map(ProgramT::FourTwo),
        "4-3" => prog_str.parse().map(ProgramT::FourThree),
        "4-4" => prog_str.parse().map(ProgramT::FourFour),
        "5-2" => prog_str.parse().map(ProgramT::FiveTwo),
        "5-3" => prog_str.parse().map(ProgramT::FiveThree),
        "5-4" => prog_str.parse().map(ProgramT::FiveFour),
        "6-2" => prog_str.parse().map(ProgramT::SixTwo),
        "6-3" => prog_str.parse().map(ProgramT::SixThree),
        "6-4" => prog_str.parse().map(ProgramT::SixFour),
        a => Err(ProgramParseError::Error(format!(
            "Unable to parse program of type {}",
            a
        ))),
    }
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

    #[test]
    fn test_program_parse_too_many() {
        let program = "1RB 0LA 1RB 0LA 1LB".parse::<Program<TwoState, TwoSymbol>>();

        assert!(program.is_err());
    }
}
