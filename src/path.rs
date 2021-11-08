use crate::Bitfield;
use std::ops::BitOr;

/// Find all permutations of the numbers between 0 and 8, with a lazy iterator.
fn permutations() -> impl Iterator<Item = [usize; 9]> {
    (0..9).flat_map(|a| {
        (0..9).filter(move |&b| a != b).flat_map(move |b| {
            let bitfield = 1 << a | 1 << b;
            (0..9)
                .filter(move |&c| bitfield & (1 << c) == 0)
                .flat_map(move |c| {
                    let bitfield = bitfield | 1 << c;
                    (0..9)
                        .filter(move |&d| bitfield & (1 << d) == 0)
                        .flat_map(move |d| {
                            let bitfield = bitfield | 1 << d;
                            (0..9)
                                .filter(move |&e| bitfield & (1 << e) == 0)
                                .flat_map(move |e| {
                                    let bitfield = bitfield | 1 << e;
                                    (0..9).filter(move |&f| bitfield & (1 << f) == 0).flat_map(
                                        move |f| {
                                            let bitfield = bitfield | 1 << f;
                                            (0..9)
                                                .filter(move |&g| bitfield & (1 << g) == 0)
                                                .flat_map(move |g| {
                                                    let bitfield = bitfield | 1 << g;
                                                    (0..9)
                                                        .filter(move |&h| bitfield & (1 << h) == 0)
                                                        .flat_map(move |h| {
                                                            let bitfield = bitfield | 1 << h;
                                                            (0..9)
                                                                .filter(move |&i| {
                                                                    bitfield & (1 << i) == 0
                                                                })
                                                                .map(move |i| {
                                                                    [a, b, c, d, e, f, g, h, i]
                                                                })
                                                        })
                                                })
                                        },
                                    )
                                })
                        })
                })
        })
    })
}

/// Create a bitfield that's true for every cell inside a box, and false elsewhere.
fn new_box(row: usize, col: usize) -> Bitfield {
    (0..3)
        .flat_map(|sub_row| {
            (0..3).map(move |sub_col| Bitfield::new(3 * row + sub_row, 3 * col + sub_col))
        })
        .fold(Bitfield::default(), BitOr::bitor)
}

/// Generate all possible "Paths" that are valid within a Sudoku.
pub fn generate_paths() -> impl Iterator<Item = Bitfield> {
    let boxes = [
        new_box(0, 0),
        new_box(0, 1),
        new_box(0, 2),
        new_box(1, 0),
        new_box(1, 1),
        new_box(1, 2),
        new_box(2, 0),
        new_box(2, 1),
        new_box(2, 2),
    ];

    permutations()
        .map(|cols| {
            cols.into_iter()
                .enumerate()
                .map(|(row, col)| Bitfield::new(row, col))
                .fold(Bitfield::default(), BitOr::bitor)
        })
        .filter(move |&potential_path| {
            boxes
                .iter()
                .cloned()
                .all(|square| (square & potential_path).len() == 1)
        })
}

#[cfg(test)]
mod permutations_test {
    use super::permutations;

    #[test]
    fn all_unique() {
        let duplicate = permutations().find(|xs| {
            for (i, x) in xs.iter().enumerate() {
                for (j, x2) in xs.iter().enumerate() {
                    if i != j && x == x2 {
                        return true;
                    }
                }
            }
            false
        });
        assert_eq!(None, duplicate)
    }

    #[test]
    fn count() {
        // 9 factorial
        assert_eq!(362_880, permutations().count());
    }
}

#[cfg(test)]
mod box_test {
    use super::new_box;

    #[test]
    fn top_left() {
        let bitfield = new_box(0, 0);
        let string = bitfield.to_string();
        let lines = string.lines().map(|line| line.trim()).collect::<Vec<_>>();

        assert_eq!(
            &lines[..],
            &[
                "+-----+-+-----+-+-----+",
                "|! ! !| |     | |     |",
                "|! ! !| |     | |     |",
                "|! ! !| |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
            ]
        );
    }

    #[test]
    fn bottom_middle() {
        let bitfield = new_box(2, 1);
        let string = bitfield.to_string();
        let lines = string.lines().map(|line| line.trim()).collect::<Vec<_>>();

        assert_eq!(
            &lines[..],
            &[
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "|     | |     | |     |",
                "+-----+-+-----+-+-----+",
                "|     | |! ! !| |     |",
                "|     | |! ! !| |     |",
                "|     | |! ! !| |     |",
                "+-----+-+-----+-+-----+",
            ]
        );
    }
}

#[cfg(test)]
mod test {
    use super::generate_paths;
    use crate::bitfield::Bitfield;

    #[test]
    fn total_count() {
        // Pre-calculated to be the right number
        assert_eq!(generate_paths().count(), 46_656);
    }

    #[test]
    fn includes_one_known_value() {
        let example_path = Bitfield::new(0, 1)
            | Bitfield::new(1, 4)
            | Bitfield::new(2, 8)
            | Bitfield::new(3, 0)
            | Bitfield::new(4, 3)
            | Bitfield::new(5, 7)
            | Bitfield::new(6, 5)
            | Bitfield::new(7, 2)
            | Bitfield::new(8, 6);

        assert!(generate_paths().any(|path| path == example_path));
    }

    #[test]
    fn does_not_include_known_bad_value() {
        let example_bad_path = Bitfield::new(0, 1)
            | Bitfield::new(1, 2)
            | Bitfield::new(2, 8)
            | Bitfield::new(3, 0)
            | Bitfield::new(4, 3)
            | Bitfield::new(5, 7)
            | Bitfield::new(6, 5)
            | Bitfield::new(7, 4)
            | Bitfield::new(8, 6);

        assert!(generate_paths().all(|path| path != example_bad_path));
    }
}
