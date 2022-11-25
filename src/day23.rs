#![allow(dead_code)]
#![allow(unused_variables)]

//#![warn(clippy::as_conversions)]

use regex::Regex;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(12521, 0);

/* Impl */

type Amphipod = char;

#[derive(Debug)]
struct Problem {
    depth: usize,
    rooms: [Vec<Amphipod>; 4],
    hallway_size: usize,
}

fn read_problem(lines: Vec<String>) -> Problem {
    let mut it = lines.iter();
    let hallway_size: usize = it.next().unwrap().len() - 2;
    let depth = lines.len() - 3;
    let mut_rooms: [&mut Vec<Amphipod>; 4] = [
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
    ];
    it.next();
    let re = Regex::new(r"([ABCD])").unwrap();
    for _ in 0..lines.len() - 2 {
        let line = it.next().unwrap();
        for (idx, m) in re.find_iter(line).enumerate() {
            let a: Amphipod = m.as_str().chars().next().unwrap();
            mut_rooms[idx].push(a);
        }
    }
    let rooms: [Vec<Amphipod>; 4] = mut_rooms.map(|m| m.to_vec());
    Problem {
        hallway_size,
        depth,
        rooms,
    }
}

fn resolve_a(problem: Problem) -> u64 {
    println!("{:?}", problem);
    0
}

pub fn process_a(lines: Vec<String>) -> u64 {
    resolve_a(read_problem(lines))
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_a(lines)
}
