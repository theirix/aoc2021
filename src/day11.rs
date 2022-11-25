use std::collections::HashSet;
use std::fmt;

use colored::*;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(1656, 195);

/* Impl */

struct Board {
    // top-level are rows so it can be indexed as [row][col] (aka [y][x])
    points: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl fmt::Debug for Board {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board: {{\n")?;
        for r in 0..self.rows {
            write!(f, "   ")?;
            for c in 0..self.cols {
                if self.points[r][c] == 0 {
                    write!(f, "{}", self.points[r][c].to_string().bold())?;
                } else {
                    write!(f, "{}", self.points[r][c])?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

fn make_board(lines: Vec<String>) -> Board {
    let cols = lines[0].len();
    let rows = lines.len();
    let mut points: Vec<Vec<u32>> = vec![vec![0; cols as usize]; rows as usize];
    for col in 0..cols {
        for row in 0..rows {
            points[row][col] = lines[row][col..col + 1].parse().unwrap(); //.chars().nth(col).unwrap().parse().unwrap();
        }
    }
    Board { points, rows, cols }
}

type Point = (usize, usize);

fn find_adjacent_points(board: &Board, row: usize, col: usize) -> Vec<Point> {
    assert!(board.cols > 1 && board.rows > 1);
    let mut res: Vec<Point> = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            let nrow = row as isize + dr;
            let ncol = col as isize + dc;
            if nrow >= 0
                && nrow < board.rows as isize
                && ncol >= 0
                && ncol < board.cols as isize
                && !(nrow == row as isize && ncol == col as isize)
            {
                res.push((nrow as usize, ncol as usize));
            }
        }
    }
    res
}

fn flash(board: &mut Board, flashed: &mut HashSet<Point>, row: usize, col: usize) {
    let p: Point = (row, col);
    if flashed.contains(&p) {
        return;
    }

    flashed.insert((row, col));

    let adjacents = find_adjacent_points(board, row, col);
    //println!("Flashes {}:{}, adjacents: {:?}", row, col, adjacents);
    for p in adjacents.iter() {
        board.points[p.0][p.1] += 1;
        if board.points[p.0][p.1] > 9 {
            flash(board, flashed, p.0, p.1);
        }
    }
}

fn run_iteration(board: &mut Board) -> u64 {
    // step 1 - increase energy
    for row in 0..board.rows {
        for col in 0..board.cols {
            board.points[row][col] += 1;
        }
    }
    // step 2 - flash
    let mut flashed: HashSet<Point> = HashSet::new();
    for row in 0..board.rows {
        for col in 0..board.cols {
            if board.points[row][col] > 9 {
                // flash
                flash(board, &mut flashed, row, col);
            }
        }
    }
    // step 3 - reset
    for row in 0..board.rows {
        for col in 0..board.cols {
            if board.points[row][col] > 9 {
                board.points[row][col] = 0;
            }
        }
    }
    flashed.len() as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let mut board = make_board(lines);
    let iterations = 100;
    (0..iterations)
        .map(|_iter| run_iteration(&mut board))
        .sum::<u64>()
}

fn is_blink(board: &Board) -> bool {
    board.points.iter().all(|row| row.iter().all(|x| *x == 0))
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let mut board = make_board(lines);
    let mut iter = 0;
    loop {
        iter += 1;
        run_iteration(&mut board);
        if is_blink(&board) {
            break iter;
        }
    }
}
