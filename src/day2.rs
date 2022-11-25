use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(150, 900);

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn read_command(line: &String) -> Command {
    let (a, b) = line.split_once(' ').unwrap();
    let amount: isize = b.parse().unwrap();
    match a {
        "forward" => Command::Forward(amount),
        "down" => Command::Down(amount),
        "up" => Command::Up(amount),
        _ => panic!("bad command"),
    }
}

fn read_commands(lines: Vec<String>) -> Vec<Command> {
    lines.iter().map(read_command).collect()
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let commands = read_commands(lines);
    let horizontal: isize = commands
        .iter()
        .map(|c| match c {
            Command::Forward(amount) => *amount as isize,
            _ => 0,
        })
        .sum();
    let vertical: isize = commands
        .iter()
        .map(|c| match c {
            Command::Up(amount) => -*amount,
            Command::Down(amount) => *amount,
            _ => 0,
        })
        .sum();
    println!("h={} v={}", horizontal, vertical);
    (horizontal * vertical).try_into().unwrap()
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let commands = read_commands(lines);
    let mut horizontal: isize = 0;
    let mut vertical: isize = 0;
    let mut aim: isize = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => {
                horizontal += amount;
                vertical += aim * amount;
            }
            Command::Up(amount) => aim -= amount,
            Command::Down(amount) => aim += amount,
        }
    }
    (horizontal * vertical).try_into().unwrap()
}
