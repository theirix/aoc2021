use crate::{answer, common::Answer};
use std::fmt;

pub const ANSWER: Answer = answer!(58, 0);

#[derive(Clone)]
enum Cucumber {
    E,
    S,
}

#[derive(Clone)]
struct Board {
    // top-level are rows so it can be indexed as [row][col] (aka [y][x])
    points: Vec<Vec<Option<Cucumber>>>,
    rows: usize,
    cols: usize,
}

impl fmt::Debug for Board {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if let Some(Cucumber::E) = self.points[r][c] {
                    write!(f, ">")?
                } else if let Some(Cucumber::S) = self.points[r][c] {
                    write!(f, "v")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn make_board(lines: Vec<String>) -> Board {
    let cols = lines[0].len();
    let rows = lines.len();
    let mut points: Vec<Vec<_>> = vec![vec![None; cols as usize]; rows as usize];
    for col in 0..cols {
        for row in 0..rows {
            points[row][col] = match lines[row].chars().nth(col).unwrap() {
                '>' => Some(Cucumber::E),
                'v' => Some(Cucumber::S),
                _ => None,
            };
        }
    }
    Board { points, rows, cols }
}

fn step_east(board: &Board) -> (Board, bool) {
    let mut nboard = board.clone();
    let mut moved = false;
    for row in 0..board.rows {
        for col in 0..board.cols {
            if let Some(Cucumber::E) = board.points[row][col] {
                let ncol = (col + 1) % board.cols;
                if board.points[row][ncol].is_none() {
                    nboard.points[row][col] = None;
                    nboard.points[row][ncol] = Some(Cucumber::E);
                    moved = true;
                }
            }
        }
    }
    (nboard, moved)
}

fn step_south(board: &Board) -> (Board, bool) {
    let mut nboard = board.clone();
    let mut moved = false;
    for col in 0..board.cols {
        for row in 0..board.rows {
            if let Some(Cucumber::S) = board.points[row][col] {
                let nrow = (row + 1) % board.rows;
                if board.points[nrow][col].is_none() {
                    nboard.points[row][col] = None;
                    nboard.points[nrow][col] = Some(Cucumber::S);
                    moved = true;
                }
            }
        }
    }
    (nboard, moved)
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let mut board = make_board(lines);
    println!("Initial state\n{:?}", board);
    for iter in 1.. {
        let (next_board, moved_east) = step_east(&board);
        let (next_board, moved_south) = step_south(&next_board);
        board = next_board;
        if !moved_east && !moved_south {
            return iter;
        }
    }
    return 0
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_a(lines)
}

