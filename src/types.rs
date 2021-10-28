use crate::program::ProgramParseError;
use std::{convert::TryFrom, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ProgramParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            a => Err(ProgramParseError(format!("Expected 'L' or 'R' got {}", a))),
        }
    }
}

pub trait State: Ord + Eq + TryFrom<char> + Copy + Debug {
    fn states() -> Vec<Self>;

    fn initial_state() -> Self;

    fn halt() -> Self;
}

pub trait Symbol: Ord + Eq + TryFrom<char> + Copy + Debug + ToString {
    fn symbols() -> Vec<Self>;

    fn zero() -> Self;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum StateT<const S: char> {
    Val(char),
    Halt,
}

impl<const S: char> ToString for StateT<S> {
    fn to_string(&self) -> String {
        match self {
            Self::Val(t) => t.to_string(),
            Self::Halt => "H".to_string(),
        }
    }
}

impl<const S: char> State for StateT<S> {
    fn states() -> Vec<Self> {
        ('A'..=S).map(StateT::Val).collect()
    }

    fn initial_state() -> Self {
        StateT::Val('A')
    }

    fn halt() -> Self {
        StateT::Halt
    }
}

impl<const S: char> TryFrom<char> for StateT<S> {
    type Error = ProgramParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'H' {
            Ok(StateT::Halt)
        } else if value >= 'A' && value <= S {
            Ok(StateT::Val(value))
        } else {
            Err(ProgramParseError(format!(
                "Expected State instruction 'A' to {} or 'H' got {}",
                S, value
            )))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SymbolT<const S: char>(pub char);

impl<const S: char> Symbol for SymbolT<S> {
    fn symbols() -> Vec<Self> {
        ('0'..=S).map(SymbolT).collect()
    }

    fn zero() -> Self {
        SymbolT('0')
    }
}

impl<const S: char> ToString for SymbolT<S> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<const S: char> TryFrom<char> for SymbolT<S> {
    type Error = ProgramParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value >= '0' && value <= S {
            Ok(SymbolT(value))
        } else {
            Err(ProgramParseError(format!(
                "Expected Symbol Instruction '0' to '{}' found {}",
                S, value
            )))
        }
    }
}
