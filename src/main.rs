use std::{env, ops::BitOr, process, time::Instant};
use sudoku::{Bitfield, Board, Digit};

fn solve_helper(possible_paths: &[Vec<Bitfield>], taken_spaces: Bitfield) -> Option<Vec<Bitfield>> {
    if let Some((first, rest)) = possible_paths.split_first() {
        first
            .iter()
            .cloned()
            .filter(|&path| (path & taken_spaces).is_empty())
            .find_map(|path| {
                let mut output = solve_helper(rest, path | taken_spaces)?;
                output.insert(0, path);
                Some(output)
            })
    } else {
        Some(Vec::new())
    }
}

fn solve(board: &mut Board, path_db: &[Bitfield]) -> bool {
    let total_clues = Digit::iter()
        .map(|digit| board[digit])
        .fold(Bitfield::default(), BitOr::bitor);

    let possible_paths = Digit::iter()
        .map(|digit| {
            let clues = board[digit];
            let opposing_clues = total_clues & !clues;

            path_db
                .iter()
                .cloned()
                .filter(|&path| path.contains(clues))
                .filter(|&path| (path & opposing_clues).is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if let Some(assigned_paths) = solve_helper(&possible_paths, Bitfield::default()) {
        for (digit, path) in Digit::iter().zip(assigned_paths) {
            board[digit] = path;
        }
        true
    } else {
        false
    }
}

fn main() {
    let args = env::args().skip(1);
    if args.len() == 0 {
        eprintln!("Usage: sudoku <puzzle> [puzzle2] [puzzle3] [puzzle4] ...");
        process::exit(1);
    }

    let all_paths = sudoku::generate_paths().collect::<Vec<_>>();

    for puzzle in args {
        if let Some(mut board) = Board::parse(&puzzle) {
            let time = Instant::now();
            let was_solved = solve(&mut board, &all_paths);
            let solve_time = time.elapsed();
            if was_solved {
                println!("{}", board);
            } else {
                println!("No solution found");
            }
            println!("Solution took {:?}", solve_time);
        } else {
            eprintln!("Invalid board format")
        }
    }
}
