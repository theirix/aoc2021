use std::fmt;

use std::collections::{HashMap, HashSet};

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(40, 315);

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
                write!(f, "{}", self.points[r][c]).unwrap();
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

impl Board {
    fn new(rows: usize, cols: usize) -> Board {
        let points: Vec<Vec<u32>> = vec![vec![0; cols as usize]; rows as usize];
        Board { points, rows, cols }
    }
}

// r,c
type Point = (usize, usize);

fn read_board(lines: &[String]) -> Board {
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

fn adjacents(board: &Board, point: Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if point.0 > 0 {
        points.push((point.0 - 1, point.1))
    }
    if point.0 < board.rows - 1 {
        points.push((point.0 + 1, point.1))
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1))
    }
    if point.1 < board.cols - 1 {
        points.push((point.0, point.1 + 1))
    }
    points
}

fn find_best(distances: &HashMap<Point, f32>, unvisited: &HashSet<Point>) -> Point {
    //where T: IntoIterator<Item = Point> {
    let mut point: Option<Point> = None;
    let mut best_f = f32::INFINITY;
    for p in unvisited {
        if distances[p] < best_f {
            best_f = distances[p];
            point = Some(*p);
        }
    }
    point.unwrap()
}

fn priority_queue_min(pq: &mut HashMap<Point, f32>) -> Point {
    let mut opoint: Option<Point> = None;
    let mut best_f = f32::INFINITY;
    for (p, prio) in pq.iter() {
        if *prio < best_f {
            best_f = *prio;
            opoint = Some(*p);
        }
    }
    let point = opoint.unwrap();
    pq.remove(&point);
    point
}

#[allow(dead_code)]
fn dijkstra_path(board: &Board, start: Point, end: Point) -> (Vec<Point>, u64) {
    let mut distances: HashMap<Point, f32> = HashMap::new();
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();

    for r in 0..board.rows {
        for c in 0..board.cols {
            unvisited.insert((r, c));
            distances.insert((r, c), f32::INFINITY);
        }
    }
    distances.insert(start, 0.0);

    println!("From {:?} to {:?}", start, end);

    while !unvisited.is_empty() {
        // find best
        let point = find_best(&distances, &unvisited);
        //println!("Choose {:?}", point);

        unvisited.remove(&point);
        //println!("Unvisited now: {}", unvisited.len());

        if point == end {
            let mut path: Vec<Point> = Vec::new();
            let mut back = end;
            while back != start {
                path.insert(0, back);
                back = prev[&back];
            }
            path.insert(0, start);
            let score = distances[&end] - distances[&start];
            return (path, score as u64);
        }

        // mark
        for out in adjacents(board, point) {
            //println!(" adj {:?}", out);
            if unvisited.contains(&out) {
                //println!(" Checking {:?}", out);
                let new_dist = distances[&point] + board.points[out.0][out.1] as f32;
                if new_dist < distances[&out] {
                    distances.insert(out, new_dist);
                    prev.insert(out, point);
                }
            }
        }
    }

    panic!("No path")
}

fn dikstra_path_uniform_cost_search(board: &Board, start: Point, end: Point) -> (Vec<Point>, u64) {
    let mut distances: HashMap<Point, f32> = HashMap::new();
    let mut explored: HashSet<Point> = HashSet::new();
    let mut priority_queue: HashMap<Point, f32> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();

    distances.insert(start, 0.0);
    priority_queue.insert(start, 0.0);

    println!("From {:?} to {:?}", start, end);

    while !priority_queue.is_empty() {
        // find best
        let point: Point = priority_queue_min(&mut priority_queue);
        //println!("Choose {:?}", point);

        explored.insert(point);
        //println!("Explored now: {}", explored.len());

        if point == end {
            let mut path: Vec<Point> = Vec::new();
            let mut back = end;
            while back != start {
                path.insert(0, back);
                back = prev[&back];
            }
            path.insert(0, start);
            let score = distances[&end] - distances[&start];
            return (path, score as u64);
        }

        // mark
        for out in adjacents(board, point) {
            let out_in_frontier = priority_queue.contains_key(&out);
            if !out_in_frontier && !explored.contains(&out) {
                let new_dist = distances[&point] + board.points[out.0][out.1] as f32;
                distances.insert(out, new_dist);
                priority_queue.insert(out, new_dist);
                prev.insert(out, point);
            } else if out_in_frontier {
                let new_dist = distances[&point] + board.points[out.0][out.1] as f32;
                if new_dist < distances[&out] {
                    distances.insert(out, new_dist);
                    priority_queue.insert(out, new_dist);
                    prev.insert(out, point);
                }
            }
        }
    }

    panic!("No path")
}

fn process_board(board: Board) -> u64 {
    let start: Point = (0, 0);
    let end: Point = (board.rows - 1, board.cols - 1);
    //let (path0, score0) = dijkstra_path(&board, start, end);
    let (path, score) = dikstra_path_uniform_cost_search(&board, start, end);
    //assert!(score == score0);
    //assert!(path == path0);
    println!("Path: {:?}, lenght {}", path, path.len());
    //if false {
    //for r in 0..board.rows {
    //for c in 0..board.cols {
    //if path.contains(&(r, c)) {
    //print!("{}", board.points[r][c]);
    //} else {
    //print!(" ");
    //}
    //}
    //print!("\n");
    //}
    //}
    score as u64
}

fn make_large_board(board: Board) -> Board {
    let mut large_board = Board::new(board.rows * 5, board.cols * 5);
    for lr in 0..5 {
        for lc in 0..5 {
            for r in 0..board.rows {
                for c in 0..board.cols {
                    let mut value = board.points[r][c] + (lc + lr);
                    while value > 9 {
                        value -= 9;
                    }
                    let newr: usize = (lr as usize) * board.rows + r as usize;
                    let newc: usize = (lc as usize) * board.cols + c as usize;
                    large_board.points[newr][newc] = value;
                }
            }
        }
    }
    large_board
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let board = read_board(&lines);
    println!("{:?}", board);
    let score = process_board(board);
    score as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let board = read_board(&lines);
    let large_board = make_large_board(board);
    println!("{:?}", large_board);
    let score = process_board(large_board);
    score as u64
}
