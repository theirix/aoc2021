use std::collections::HashSet;
use std::fmt;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(4512, 1924);

struct Board {
    size: usize,
    data: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>,
}

//fn print_data(data: Vec<Vec<i32>>) -> String {
//"foo".to_string()
//}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Board: {{size={}, data=", self.size)?;
        for r in 0..self.size {
            write!(f, "   ")?;
            for c in 0..self.size {
                let mark = if self.marked[r][c] { "*" } else { " " };
                write!(f, "{:>2}{}  ", self.data[r][c], mark)?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")

        //f.debug_struct("Board")
        //.field("size", &self.size)
        //.field("data", print_data(self.data))
        //.finish()
    }
}

fn read_numbers(lines: &[String]) -> Vec<i32> {
    let line = lines.first().unwrap();
    let numbers: Vec<i32> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    numbers
}

fn read_board(lines: &Vec<String>) -> Board {
    let mut board = Board {
        size: lines.len() as usize,
        data: Vec::new(),
        marked: Vec::new(),
    };
    for line in lines {
        let row = line
            .split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        board.data.push(row);
        let marked = vec![false; board.size];
        board.marked.push(marked);
    }
    board
}

fn read_boards(in_lines: &[String]) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let mut it = in_lines.iter();
    it.next();
    it.next();
    for line in it {
        if !line.is_empty() {
            lines.push(line.to_string());
        } else if !lines.is_empty() {
            boards.push(read_board(&lines));
            lines.clear();
        }
    }
    if !lines.is_empty() {
        boards.push(read_board(&lines));
    }
    boards
}

fn draw_number_a(number: i32, boards: &mut Vec<Board>) -> Option<&mut Board> {
    for board in boards {
        // mark number
        for col in 0..board.size {
            for row in 0..board.size {
                if board.data[col][row] == number {
                    board.marked[col][row] = true;
                }
            }
        }
        if does_win(board) {
            return Some(board);
        }
    }
    None
}

fn does_win(board: &Board) -> bool {
    // determine is it a winner by rows
    for col in 0..board.size {
        let mut all = true;
        for row in 0..board.size {
            if !board.marked[col][row] {
                all = false;
            }
        }
        if all {
            return true;
        }
    }
    // determine is it a winner by cols
    for row in 0..board.size {
        let mut all = true;
        for col in 0..board.size {
            if !board.marked[col][row] {
                all = false;
            }
        }
        if all {
            return true;
        }
    }
    false
}

fn draw_number_b(number: i32, boards: &mut [Board]) -> Option<&mut Board> {
    let winned: Vec<bool> = boards.iter().map(does_win).collect();
    for (ind, board) in &mut boards.iter_mut().enumerate() {
        // mark number
        for col in 0..board.size {
            for row in 0..board.size {
                if board.data[col][row] == number {
                    board.marked[col][row] = true;
                }
            }
        }
        // if all other wins
        let mut all_other_win = !winned[ind];
        for (win_ind, win_item) in winned.iter().enumerate() {
            if ind != win_ind && !win_item {
                all_other_win = false;
            }
        }
        println!(
            "For num {} winned {:?} ind {} all other win={}",
            number, winned, ind, all_other_win
        );
        if all_other_win && does_win(board) {
            return Some(board);
        }
    }
    None
}

fn determine_score(number: i32, board: &Board) -> u64 {
    let mut sum = 0;
    for col in 0..board.size {
        for row in 0..board.size {
            if !board.marked[col][row] {
                sum += board.data[col][row];
            }
        }
    }
    (number * sum) as u64
}

fn find_winning_board(numbers: Vec<i32>, boards: &mut Vec<Board>) -> u64 {
    for number in numbers {
        if let Some(winning_board) = draw_number_a(number, boards) {
            println!("Winning number {}, board {:?}", number, winning_board);
            return determine_score(number, winning_board);
        }
    }
    panic!("No result")
}

fn find_losing_board(numbers: Vec<i32>, boards: &mut Vec<Board>) -> u64 {
    for number in numbers {
        if let Some(winning_board) = draw_number_b(number, boards) {
            println!("Winning number {}, board {:?}", number, winning_board);
            return determine_score(number, winning_board);
        }
    }
    panic!("No result")
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(&lines);
    println!("Numbers: {:?}", numbers);
    let mut boards = read_boards(&lines);
    let sizes: HashSet<usize> = HashSet::from_iter(boards.iter().map(|b| b.size));
    println!("Board count {} of sizes {:?}", boards.len(), sizes);
    if sizes.len() != 1 || *sizes.iter().next().unwrap() == 0 {
        panic!("Wrong sizes");
    }
    for b in &boards {
        println!("{:?}", b);
    }
    find_winning_board(numbers, &mut boards)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(&lines);
    println!("Numbers: {:?}", numbers);
    let mut boards = read_boards(&lines);
    let sizes: HashSet<usize> = HashSet::from_iter(boards.iter().map(|b| b.size));
    println!("Board count {} of sizes {:?}", boards.len(), sizes);
    if sizes.len() != 1 || *sizes.iter().next().unwrap() == 0 {
        panic!("Wrong sizes");
    }
    for b in &boards {
        println!("{:?}", b);
    }
    find_losing_board(numbers, &mut boards)
}
