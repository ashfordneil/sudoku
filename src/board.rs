use crate::{Bitfield, Digit};
use std::{
    fmt::{Display, Formatter, Write},
    ops::{Index, IndexMut},
};

/// A representation of a Sudoku. Rather than letting you look up what digit is located at a
/// position, we optimise to make it easiest to look up which positions are filled by a certain
/// digit.
///
/// Use the `Index` and `IndexMut` traits, with `Digit` enums as lookups, to find the positions
/// filled by any given digit.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    placements: [Bitfield; 9],
}

impl Board {
    /// Create an empty Sudoku board.
    fn empty() -> Self {
        Board {
            placements: [Default::default(); 9],
        }
    }

    /// Assert the internal validity of the board structure. This says nothing about whether the
    /// puzzle follows the **rules** of Sudoku, and is instead just a simple check that we haven't
    /// accidentally put two different digits into the same square.
    fn valid(&self) -> bool {
        let mut total = Bitfield::default();
        for digit in Digit::iter() {
            let current = self[digit];
            if current.len() > 9 {
                return false;
            }
            if !(current & total).is_empty() {
                return false;
            }

            total |= current;
        }

        true
    }

    /// Parse a Sudoku from what appears to be the standard text representation. The cells of each
    /// row are listed in order as a single (81 char long) string. Digits are represented as
    /// themselves in ASCII, and "." represents empty cells. Any input that does not match the
    /// format is returned as None.
    pub fn parse(input: &str) -> Option<Self> {
        if input.len() != 81 {
            return None;
        }

        let board = input
            .chars()
            .enumerate()
            .filter_map(|(idx, ch)| {
                if ch == '.' {
                    None
                } else {
                    let row = idx / 9;
                    let col = idx % 9;
                    Some((Bitfield::new(row, col), ch))
                }
            })
            .try_fold(Self::empty(), |mut board, (bit, ch)| {
                let digit = Digit::parse(ch)?;
                board[digit] |= bit;
                Some(board)
            })?;

        if board.valid() {
            Some(board)
        } else {
            None
        }
    }
}

impl Index<Digit> for Board {
    type Output = Bitfield;

    fn index(&self, index: Digit) -> &Self::Output {
        let idx: usize = index.into();
        &self.placements[idx - 1]
    }
}

impl IndexMut<Digit> for Board {
    fn index_mut(&mut self, index: Digit) -> &mut Self::Output {
        let idx: usize = index.into();
        &mut self.placements[idx - 1]
    }
}

impl Display for Board {
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

                let cell = Bitfield::new(row, col);
                let digit = Digit::iter().find(|&digit| self[digit].contains(cell));
                match digit {
                    Some(digit) => <Digit as Display>::fmt(&digit, f)?,
                    None => f.write_char(' ')?,
                }
            }
            f.write_char('|')?;
            writeln!(f)?;
        }

        f.write_str(row_sep)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use crate::digit::Digit;
    use crate::Bitfield;

    #[test]
    fn valid_function_works() {
        // does not test for correctness, just internal consistency
        let mut board = Board::empty();
        assert!(board.valid());

        board[Digit::_1] |= Bitfield::new(5, 5);
        board[Digit::_2] |= Bitfield::new(4, 5);
        board[Digit::_3] |= Bitfield::new(3, 5);
        assert!(board.valid());

        board[Digit::_5] |= Bitfield::new(5, 5);
        assert!(!board.valid());
    }

    #[test]
    fn notices_incorrect_txt() {
        assert_eq!(None, Board::parse(""));
        // Correct length, contains invalid character
        assert_eq!(
            None,
            Board::parse(
                "........Q........................................................................"
            )
        );
        // 90 character string, too long
        assert_eq!(
            None,
            Board::parse(
                ".........................................................................................."
            )
        );
    }

    #[test]
    fn parses_txt_correctly() {
        let input =
            "........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3";
        let board = Board::parse(input).unwrap();

        assert_eq!(board[Digit::_1], Bitfield::new(7, 1) | Bitfield::new(4, 5));
        assert_eq!(
            board[Digit::_2],
            Bitfield::new(2, 4) | Bitfield::new(4, 6) | Bitfield::new(5, 5) | Bitfield::new(7, 7)
        );
        assert_eq!(board[Digit::_3], Bitfield::new(1, 2) | Bitfield::new(8, 8));
        assert_eq!(board[Digit::_4], Bitfield::new(1, 6) | Bitfield::new(8, 0));
        assert_eq!(
            board[Digit::_5],
            Bitfield::new(5, 3) | Bitfield::new(6, 6) | Bitfield::new(8, 2)
        );
        assert_eq!(
            board[Digit::_6],
            Bitfield::new(2, 7) | Bitfield::new(4, 4) | Bitfield::new(5, 1)
        );
        assert_eq!(board[Digit::_7], Bitfield::new(3, 4) | Bitfield::new(5, 7));
        assert_eq!(board[Digit::_8], Bitfield::new(0, 8) | Bitfield::new(6, 2));
        assert_eq!(board[Digit::_9], Bitfield::new(2, 1) | Bitfield::new(3, 5));
    }

    #[test]
    fn displays_correctly() {
        // Use previous input just to make construction simpler
        let input =
            "........8..3...4...9..2..6.....79.......612...6.5.2.7...8...5...1.....2.4.5.....3";
        let board = Board::parse(input).unwrap();
        let string = board.to_string();
        let lines = string.lines().map(|line| line.trim()).collect::<Vec<_>>();

        assert_eq!(
            &lines[..],
            &[
                "+-----+-+-----+-+-----+",
                "|     | |     | |    8|",
                "|    3| |     | |4    |",
                "|  9  | |  2  | |  6  |",
                "+-----+-+-----+-+-----+",
                "|     | |  7 9| |     |",
                "|     | |  6 1| |2    |",
                "|  6  | |5   2| |  7  |",
                "+-----+-+-----+-+-----+",
                "|    8| |     | |5    |",
                "|  1  | |     | |  2  |",
                "|4   5| |     | |    3|",
                "+-----+-+-----+-+-----+",
            ]
        );
    }
}
