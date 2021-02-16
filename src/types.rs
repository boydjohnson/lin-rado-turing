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
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
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
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "H" => Ok(Self::H),
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
        Self::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::A, Self::B].into_iter())
    }

    fn halt() -> Self {
        Self::H
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
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "H" => Ok(Self::H),
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
        Self::A
    }

    fn iter_states() -> Box<dyn Iterator<Item = Self>> {
        Box::new(vec![Self::A, Self::B, Self::C].into_iter())
    }

    fn halt() -> Self {
        Self::H
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
        Box::new(vec![Self::A, Self::B, Self::C, Self::D].into_iter())
    }

    fn halt() -> Self {
        Self::H
    }
}

impl FromStr for FourState {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "H" => Ok(Self::H),
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
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
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
        Box::new(vec![Self::Zero, Self::One].into_iter())
    }

    fn zero() -> Self {
        Self::Zero
    }
}

impl ToString for TwoSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::One => "#",
            Self::Zero => "_",
        }
        .to_owned()
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
        Self::Zero
    }
}

impl ToString for ThreeSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
        }
        .to_owned()
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
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
        }
        .to_owned()
    }
}
