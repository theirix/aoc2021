use std::collections::HashSet;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(15, 1134);

struct Board {
    // top-level are rows so it can be indexed as [row][col] (aka [y][x])
    points: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

type Point = (usize, usize);

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

/*
fn find_adjacents_exp(board: &Board, row: usize, col: usize) -> Vec<u32> {
    let ir = row as i32;
    let ic = col as i32;
    let coords = vec![
        (ir-1,ic),
        (ir+1,ic),
        (ir,ic-1),
        (ir,ic+1)
    ];
    vec![]
}*/

fn find_adjacent_points(board: &Board, row: usize, col: usize) -> Vec<Point> {
    assert!(board.cols > 1 && board.rows > 1);
    let mut res: Vec<Point> = Vec::new();
    if row > 0 {
        res.push((row - 1, col));
    }
    if col > 0 {
        res.push((row, col - 1));
    }
    if row < board.rows - 1 {
        res.push((row + 1, col));
    }
    if col < board.cols - 1 {
        res.push((row, col + 1));
    }
    res
}

fn is_low(board: &Board, row: usize, col: usize) -> bool {
    let adjacents = find_adjacent_points(board, row, col);
    let lowest = adjacents
        .iter()
        .map(|p| &board.points[p.0][p.1])
        .all(|v| v > &board.points[row][col]);
    lowest
}

fn process_board(board: Board) -> u64 {
    let mut risk = 0u64;
    for row in 0..board.rows {
        for col in 0..board.cols {
            if is_low(&board, row, col) {
                let local_risk = board.points[row][col] + 1;
                risk += local_risk as u64;
            }
        }
    }
    risk
}

// Breadth-first search
struct Search<'a> {
    board: &'a Board,
    visited: HashSet<Point>,
}

impl<'a> Search<'a> {
    fn new(board: &'a Board) -> Search<'a> {
        Search {
            board,
            visited: HashSet::new(),
        }
    }

    fn grow(&mut self, point: Point) {
        self.visited.insert(point);
        // check neighbours
        //let adjacents: Vec<Point> = find_adjacent_points(&self.board, point.0, point.1);
        let adjacents: Vec<Point> = find_adjacent_points(&self.board, point.0, point.1)
            .into_iter()
            .filter(|p| !self.visited.contains(p))
            .filter(|p| self.board.points[p.0][p.1] < 9)
            .collect();
        if !adjacents.is_empty() {
            println!("Found {} adjacents for {:?}", adjacents.len(), point);
        }
        for adj in adjacents {
            self.grow(adj);
        }
    }

    fn locate(&mut self, base: Point) -> Vec<Point> {
        self.grow(base);
        self.visited.drain().collect()
    }
}

fn locate_basin(board: &Board, row: usize, col: usize) -> u32 {
    let low: Point = (row, col);
    let mut search = Search::new(board);
    let points = search.locate(low);
    points.len() as u32
}

fn process_basins(board: Board) -> u64 {
    let mut basins: Vec<u64> = Vec::new();
    for row in 0..board.rows {
        for col in 0..board.cols {
            if is_low(&board, row, col) {
                // find basin around point, it is a basin size
                let basin = locate_basin(&board, row, col);
                println!("Located basin {} around {}:{}", basin, row, col);
                basins.push(basin as u64);
            }
        }
    }
    println!("Basins {:?}", basins);
    basins.sort_by(|a, b| a.cmp(b).reverse());
    assert!(basins.len() >= 3);
    basins[0..3].iter().product()
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let board = make_board(lines);
    process_board(board)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let board = make_board(lines);
    process_basins(board)
}
