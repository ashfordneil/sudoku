use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

/// A boolean field defined over the 9x9 grid of a Sudoku. This stores a yes/no value for each cell
/// on the board, and defines some useful operators.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Bitfield(u128);

const MASK: Bitfield = Bitfield(
    0b111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111,
);

impl Bitfield {
    /// Create a new bitfield, with exactly one bit set, corresponding to the cell at position
    /// `row`, `col`.
    pub fn new(row: usize, col: usize) -> Self {
        assert!(row < 9 && col < 9);

        let bit = 9 * row + col;

        Bitfield(1 << bit)
    }

    /// Is this bitfield completely empty? A bitfield which satisfies this can be created with
    /// `Default::default()`.
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Is this bitfield a complete superset of the other bitfield?
    pub fn contains(self, other: Self) -> bool {
        other & self == other
    }

    /// How many true values are there within this bitfield?
    pub fn len(self) -> u32 {
        self.0.count_ones()
    }
}

// Print the thing as an ascii-art board, using "!" to show where the bitfield is set.
// This is 90% just so we can have readable tests for the more complex stuff
impl Display for Bitfield {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let row_sep = "+-----+-+-----+-+-----+";
        for row in 0..9 {
            if row % 3 == 0 {
                f.write_str(row_sep)?;
                writeln!(f)?;
            }

            f.write_char('|')?;
            for col in 0..9 {
                if col != 0 {
                    if col % 3 == 0 {
                        f.write_str("| |")?;
                    } else {
                        f.write_char(' ')?;
                    }
                }

                let ch = if self.contains(Bitfield::new(row, col)) {
                    '!'
                } else {
                    ' '
                };

                f.write_char(ch)?;
            }
            f.write_char('|')?;
            writeln!(f)?;
        }

        f.write_str(row_sep)?;

        Ok(())
    }
}

// Print the thing in binary, with exactly 81 bits because that's all we need for this
impl Debug for Bitfield {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:081b}", self.0)
    }
}

// Only implement the bitwise arithmetic traits, and only implement the specific bitwise arithmetic
// traits that can't be used to accidentally create a bitfield where any bit after bit 81 is set.
impl BitOr for Bitfield {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitfield(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitfield {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for Bitfield {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitfield(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitfield {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Not for Bitfield {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitfield(!self.0) & MASK
    }
}

#[cfg(test)]
mod test {
    use super::Bitfield;

    #[test]
    fn storage_mechanism_works() {
        assert_eq!(Bitfield::new(5, 4), Bitfield::new(5, 4));
        assert_ne!(Bitfield::new(5, 4), Bitfield::new(4, 5));
    }

    #[test]
    fn is_empty_check() {
        assert!(Bitfield::default().is_empty());
        assert!(!Bitfield::new(3, 6).is_empty());
    }

    #[test]
    fn contains_check() {
        let small = Bitfield::new(5, 4);
        let big = small | Bitfield::new(3, 7);
        let biggest = big | Bitfield::new(1, 1);

        assert!(biggest.contains(big));
        assert!(biggest.contains(small));
        assert!(big.contains(small));

        assert!(!small.contains(big));
        assert!(!small.contains(biggest));
        assert!(!big.contains(biggest));
    }

    #[test]
    fn len_check() {
        let small = Bitfield::new(5, 4);
        let big = small | Bitfield::new(3, 7);
        let biggest = big | Bitfield::new(1, 1);

        assert_eq!(small.len(), 1);
        assert_eq!(big.len(), 2);
        assert_eq!(biggest.len(), 3);
    }

    #[test]
    fn debug_format() {
        let bitfield = Bitfield::new(3, 6) | Bitfield::new(4, 5);
        let string = format!("{:?}", bitfield);
        assert_eq!(string.len(), 81);
        assert_eq!(string.chars().filter(|&ch| ch == '1').count(), 2);
        assert_eq!(string.chars().filter(|&ch| ch == '0').count(), 79);
    }

    #[test]
    fn display_format() {
        let bitfield = Bitfield::new(3, 6) | Bitfield::new(1, 2) | Bitfield::new(8, 8);
        let string = bitfield.to_string();
        let lines = string.lines().map(|line| line.trim()).collect::<Vec<_>>();

        assert_eq!(
            &lines[..],
            &[
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|    !| |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |     | |!    |",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "|     | |     | |    !|",
                "+-----+-+-----+-+-----+",
            ]
        );
    }
}
