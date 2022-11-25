use std::fmt;

use itertools::Itertools;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(17, 0);

/* Impl */

struct Board {
    // top-level are rows so it can be indexed as [row][col] (aka [y][x])
    points: Vec<Vec<bool>>,
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
                write!(f, "{}", if self.points[r][c] { "#" } else { "." }).unwrap();
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        let points: Vec<Vec<bool>> = vec![vec![false; cols as usize]; rows as usize];
        Board { points, rows, cols }
    }
}

fn read_board(lines: &Vec<String>, folds: &[Fold]) -> Board {
    let pairs: Vec<(usize, usize)> = lines
        .iter()
        .filter(|s| s.contains(','))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    // data lies, there can be empty row or column
    //let rows = pairs.iter().map(|(_,r)| r ).max().unwrap() + 1;
    //let cols = pairs.iter().map(|(c,_)| c ).max().unwrap() + 1;
    let rows = (folds
        .iter()
        .map(|f| if let Fold::Row(value) = *f { value } else { 0 })
        .find(|x| *x > 0)
        .unwrap() as usize
        * 2)
        + 1;
    let cols = (folds
        .iter()
        .map(|f| if let Fold::Col(value) = *f { value } else { 0 })
        .find(|x| *x > 0)
        .unwrap() as usize
        * 2)
        + 1;
    println!("rows {}, cols {}", rows, cols);
    assert!((rows % 2) == 1);
    assert!((cols % 2) == 1);
    let mut board = Board::new(rows, cols);
    for (c, r) in pairs {
        board.points[r][c] = true;
    }
    board
}

#[derive(Debug)]
enum Fold {
    Row(usize),
    Col(usize),
}

fn read_folds(lines: &Vec<String>) -> Vec<Fold> {
    lines
        .iter()
        .filter(|s| s.contains("fold"))
        .map(|line| {
            if line.starts_with("fold along y=") {
                Fold::Row(line.split('=').nth(1).unwrap().parse().unwrap())
            } else if line.starts_with("fold along x=") {
                Fold::Col(line.split('=').nth(1).unwrap().parse().unwrap())
            } else {
                panic!("wrong instruction {}", line);
            }
        })
        .collect()
}

fn process_fold(board: Board, fold: &Fold) -> Board {
    println!(
        "Begin folding at {:?} board of {} rows and {} cols",
        fold, board.rows, board.cols
    );
    match fold {
        Fold::Row(row) => {
            assert!((board.rows % 2) == 1);
            let mut new_board = Board::new((board.rows - 1) / 2, board.cols);
            for r in 0..new_board.rows {
                for c in 0..new_board.cols {
                    new_board.points[r][c] =
                        board.points[r][c] || board.points[board.rows - r - 1][c];
                }
            }
            println!("After folding at row {} board is {:?}", row, new_board);
            new_board
        }
        Fold::Col(col) => {
            assert!((board.cols % 2) == 1);
            let mut new_board = Board::new(board.rows, (board.cols - 1) / 2);
            for r in 0..new_board.rows {
                for c in 0..new_board.cols {
                    new_board.points[r][c] =
                        board.points[r][c] || board.points[r][board.cols - c - 1];
                }
            }
            println!("After folding at col {} board is {:?}", col, new_board);
            new_board
        }
    }
}

fn score(board: Board) -> u64 {
    board
        .points
        .iter()
        .map(|row| row.iter().map(|v| if *v { 1 } else { 0 }).sum::<u64>())
        .sum::<u64>()
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let folds = read_folds(&lines);
    let board = read_board(&lines, &folds);
    println!("{:?}", board);
    println!("{:?}", folds);
    let final_board = folds[0..1].iter().fold(board, process_fold);
    println!("Final board {:?}", final_board);
    score(final_board) as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let folds = read_folds(&lines);
    let board = read_board(&lines, &folds);
    println!("{:?}", board);
    println!("{:?}", folds);
    let final_board = folds.iter().fold(board, process_fold);
    println!("Final board {:?}", final_board);
    0
}
