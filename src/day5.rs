use std::fmt;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(5, 12);

struct Line {
    from: (u32, u32),
    to: (u32, u32),
}

impl fmt::Debug for Line {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Line")
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

fn read_line(line: &str) -> Line {
    let comp: Vec<&str> = line.split(" -> ").collect();
    if comp.len() != 2 {
        panic!("Bad input");
    }
    let first_comp: Vec<&str> = comp[0].split(',').collect();
    let second_comp: Vec<&str> = comp[1].split(',').collect();
    Line {
        from: (
            first_comp[0].parse().unwrap(),
            first_comp[1].parse().unwrap(),
        ),
        to: (
            second_comp[0].parse().unwrap(),
            second_comp[1].parse().unwrap(),
        ),
    }
}

struct Board {
    // top-level are cols so it can be indexed as [x][y]
    points: Vec<Vec<u32>>,
}

impl Board {
    fn print(&self) {
        let mut max_val: u32 = 0;
        for col in &self.points {
            for v in col {
                max_val = std::cmp::max(max_val, *v);
            }
        }
        let width: usize = (max_val as f32).log10().round() as usize;

        let height = self.points[0].len();
        for y in 0..height {
            print!("   ");
            for x in 0..self.points.len() {
                if self.points[x][y] == 0 {
                    print!(".");
                } else {
                    print!("{:>width$}", self.points[x][y], width = width);
                }
            }
            println!();
        }
    }
}

fn read_lines(lines: Vec<String>) -> Vec<Line> {
    lines.iter().map(|s| read_line(s)).collect()
}

fn make_map(lines: Vec<Line>, allow_diag: bool) -> Board {
    let mut max_x = 0;
    let mut max_y = 0;
    // line is (x,y)
    for line in &lines {
        max_x = std::cmp::max(max_x, line.from.0);
        max_x = std::cmp::max(max_x, line.to.0);
        max_y = std::cmp::max(max_y, line.from.1);
        max_y = std::cmp::max(max_y, line.to.1);
    }
    max_x += 1;
    max_y += 1;
    let mut board = Board {
        points: vec![vec![0; max_y as usize]; max_x as usize],
    };

    for line in lines {
        let from_x = if line.from.0 < line.to.0 {
            line.from.0
        } else {
            line.to.0
        };
        let to_x = if line.from.0 < line.to.0 {
            line.to.0
        } else {
            line.from.0
        };
        let from_y = if line.from.1 < line.to.1 {
            line.from.1
        } else {
            line.to.1
        };
        let to_y = if line.from.1 < line.to.1 {
            line.to.1
        } else {
            line.from.1
        };
        assert!(from_x <= to_x);
        assert!(from_y <= to_y);
        if line.from.1 == line.to.1 {
            // x1, y1 -> x2, y1
            let y = line.from.1;
            for x in from_x..(to_x + 1) {
                board.points[x as usize][y as usize] += 1;
            }
        } else if line.from.0 == line.to.0 {
            // x1, y1 -> x1, y2
            let x = line.from.0;
            for y in from_y..(to_y + 1) {
                board.points[x as usize][y as usize] += 1;
            }
        } else if allow_diag {
            for ind in 0..(to_x - from_x + 1) {
                //println!("{}", ind;
                let xstep: i32 = if line.from.0 < line.to.0 { 1 } else { -1 };
                let ystep: i32 = if line.from.1 < line.to.1 { 1 } else { -1 };
                //println!("{} {}", xstep, ystep);
                let x = line.from.0 as i32 + xstep * (ind as i32);
                let y = line.from.1 as i32 + ystep * (ind as i32);
                //println!("{} {}", x, y);
                board.points[x as usize][y as usize] += 1;
            }
        }
        //println!("\nLine {:?}", line);
        //board.print();
    }
    board
}

fn determine_score(board: Board) -> u64 {
    let mut count = 0;
    for col in board.points {
        for v in col {
            if v >= 2 {
                count += 1;
            }
        }
    }
    count
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let board = make_map(read_lines(lines), false);
    board.print();
    determine_score(board)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let board = make_map(read_lines(lines), true);
    board.print();
    determine_score(board)
}
