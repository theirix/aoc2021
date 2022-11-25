use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(7, 5);

fn read_numbers(lines: Vec<String>) -> Vec<u64> {
    lines.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(lines);
    let len = numbers.len();
    let mut count = 1;
    for i in 2..len {
        if numbers[i] > numbers[i - 1] {
            count += 1
        }
    }
    count
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(lines);
    let len = numbers.len();
    let mut count = 0;
    let mut prev_window = u64::max_value();
    for i in 0..len - 2 {
        let window = numbers[i] + numbers[i + 1] + numbers[i + 2];
        println!("at {} win {}", i, window);
        if window > prev_window {
            count += 1
        }
        prev_window = window;
    }
    count
}
