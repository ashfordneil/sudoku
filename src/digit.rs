use std::fmt::{Debug, Display, Formatter};

/// A single digit that can be placed in a cell of a Sudoku.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Digit {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

impl Digit {
    /// Iterate through all possible digits.
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Digit::_1,
            Digit::_2,
            Digit::_3,
            Digit::_4,
            Digit::_5,
            Digit::_6,
            Digit::_7,
            Digit::_8,
            Digit::_9,
        ]
        .into_iter()
    }

    /// Parse the input character as a digit. Returns None if the character was invalid.
    pub fn parse(ch: char) -> Option<Self> {
        let output = match ch {
            '1' => Digit::_1,
            '2' => Digit::_2,
            '3' => Digit::_3,
            '4' => Digit::_4,
            '5' => Digit::_5,
            '6' => Digit::_6,
            '7' => Digit::_7,
            '8' => Digit::_8,
            '9' => Digit::_9,
            _ => return None
        };
        Some(output)
    }
}

impl Into<usize> for Digit {
    fn into(self) -> usize {
        match self {
            Digit::_1 => 1,
            Digit::_2 => 2,
            Digit::_3 => 3,
            Digit::_4 => 4,
            Digit::_5 => 5,
            Digit::_6 => 6,
            Digit::_7 => 7,
            Digit::_8 => 8,
            Digit::_9 => 9,
        }
    }
}

impl Debug for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number: usize = self.clone().into();
        <usize as Debug>::fmt(&number, f)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number: usize = self.clone().into();
        <usize as Display>::fmt(&number, f)
    }
}
