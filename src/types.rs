use crate::program::ProgramParseError;
use std::{fmt::Debug, str::FromStr};

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

pub trait Symbol: Ord + Eq + FromStr + Copy + Debug + ToString {
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

impl ToString for TwoSymbol {
    fn to_string(&self) -> String {
        match self {
            TwoSymbol::One => "#".to_owned(),
            TwoSymbol::Zero => "_".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum ThreeSymbol {
    Zero,
    One,
    Two,
}

impl Symbol for ThreeSymbol {
    const NUM: usize = 3;

    fn iter_symbols() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::Zero, Self::One, Self::Two].into_iter())
    }

    fn zero() -> Self {
        ThreeSymbol::Zero
    }
}

impl ToString for ThreeSymbol {
    fn to_string(&self) -> String {
        match self {
            ThreeSymbol::Zero => "0".to_owned(),
            ThreeSymbol::One => "1".to_owned(),
            ThreeSymbol::Two => "2".to_owned(),
        }
    }
}

impl FromStr for ThreeSymbol {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            a => Err(ProgramParseError::Error(format!(
                "Expected '0', '1', or '2', not {}",
                a
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum FiveState {
    A,
    B,
    C,
    D,
    E,
    H,
}

impl FromStr for FiveState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "H" => Ok(Self::H),
            a => Err(ProgramParseError::Error(format!(
                "Expecting 'A', 'B', 'C', 'D', 'E', or 'H', not {}",
                a
            ))),
        }
    }
}

impl State for FiveState {
    const NUM_STATES: usize = 5;

    fn initial_state() -> Self {
        Self::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::A, Self::B, Self::C, Self::D, Self::E].into_iter())
    }

    fn halt() -> Self {
        Self::H
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum SixState {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
}

impl FromStr for SixState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "H" => Ok(Self::H),
            a => Err(ProgramParseError::Error(format!(
                "Expecting 'A', 'B', 'C', 'D', 'E', or 'H', not {}",
                a
            ))),
        }
    }
}

impl State for SixState {
    const NUM_STATES: usize = 5;

    fn initial_state() -> Self {
        Self::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::A, Self::B, Self::C, Self::D, Self::E, Self::F].into_iter())
    }

    fn halt() -> Self {
        Self::H
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum FourSymbol {
    Zero,
    One,
    Two,
    Three,
}

impl Symbol for FourSymbol {
    const NUM: usize = 3;

    fn iter_symbols() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::Zero, Self::One, Self::Two, Self::Three].into_iter())
    }

    fn zero() -> Self {
        Self::Zero
    }
}

impl FromStr for FourSymbol {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            a => Err(ProgramParseError::Error(format!(
                "Expected '0', '1', or '2', not {}",
                a
            ))),
        }
    }
}

impl ToString for FourSymbol {
    fn to_string(&self) -> String {
        match self {
            FourSymbol::Zero => "0".to_owned(),
            FourSymbol::One => "1".to_owned(),
            FourSymbol::Two => "2".to_owned(),
            FourSymbol::Three => "3".to_owned(),
        }
    }
}
