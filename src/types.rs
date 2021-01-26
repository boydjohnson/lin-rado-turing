use crate::program::ProgramParseError;
use std::{fmt::Debug, hash::Hash, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            s => Err(ProgramParseError::Error(format!(
                "Expected 'L' or 'R', found {}",
                s
            ))),
        }
    }
}

pub trait State: Ord + Eq + FromStr + Copy + Debug {
    const NUM_STATES: usize;

    fn initial_state() -> Self;

    fn iter_states() -> Box<dyn Iterator<Item = Self>>;

    fn halt() -> Self;
}

pub trait Symbol: Ord + Eq + FromStr + Copy + Debug {
    const NUM: usize;

    fn iter_symbols() -> Box<dyn Iterator<Item = Self>>;

    fn zero() -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum TwoState {
    A,
    B,
    H,
}

impl FromStr for TwoState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(TwoState::A),
            "B" => Ok(TwoState::B),
            "H" => Ok(TwoState::H),
            a => Err(ProgramParseError::Error(format!(
                "Expected 'A', 'B', 'H' not {}",
                a
            ))),
        }
    }
}

impl State for TwoState {
    const NUM_STATES: usize = 2;

    fn initial_state() -> Self {
        TwoState::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![TwoState::A, TwoState::B].into_iter())
    }

    fn halt() -> Self {
        TwoState::H
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ThreeState {
    A,
    B,
    C,
    H,
}

impl FromStr for ThreeState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(ThreeState::A),
            "B" => Ok(ThreeState::B),
            "C" => Ok(ThreeState::C),
            "H" => Ok(ThreeState::H),
            a => Err(ProgramParseError::Error(format!(
                "Expected 'A', 'B', 'C', or 'H', found {}",
                a
            ))),
        }
    }
}

impl State for ThreeState {
    const NUM_STATES: usize = 3;

    fn initial_state() -> Self {
        ThreeState::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![ThreeState::A, ThreeState::B, ThreeState::C].into_iter())
    }

    fn halt() -> Self {
        ThreeState::H
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum FourState {
    A,
    B,
    C,
    D,
    H,
}

impl State for FourState {
    const NUM_STATES: usize = 4;

    fn initial_state() -> Self {
        Self::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![FourState::A, FourState::B, FourState::C, FourState::D].into_iter())
    }

    fn halt() -> Self {
        Self::H
    }
}

impl FromStr for FourState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(FourState::A),
            "B" => Ok(FourState::B),
            "C" => Ok(FourState::C),
            "D" => Ok(FourState::D),
            "H" => Ok(FourState::H),
            a => Err(ProgramParseError::Error(format!(
                "Expected 'A', 'B', 'C', 'D', 'H', found {}",
                a
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum TwoSymbol {
    Zero,
    One,
}

impl FromStr for TwoSymbol {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(TwoSymbol::Zero),
            "1" => Ok(TwoSymbol::One),
            a => Err(ProgramParseError::Error(format!(
                "Expected '0' or '1', found {}",
                a
            ))),
        }
    }
}

impl Symbol for TwoSymbol {
    const NUM: usize = 2;

    fn iter_symbols() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![TwoSymbol::Zero, TwoSymbol::One].into_iter())
    }

    fn zero() -> Self {
        TwoSymbol::Zero
    }
}
