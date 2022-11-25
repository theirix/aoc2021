use std::collections::HashMap;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(26397, 288957);

/* Impl */

fn pairs() -> HashMap<char, char> {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])
}

//fn revpairs() -> HashMap<char, char> {
//let r: Vec<(char, char)> = pairs().iter().map(|(k,v)| (*v,*k)).collect();
//HashMap::from_iter(r)
//}

fn score_a(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("bad symbol for score {}", c),
    }
}

fn score_b(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("bad symbol for score {}", c),
    }
}

fn handle_open(stack: &mut Vec<char>, c: char) -> Result<(), char> {
    stack.push(c);
    Ok(())
}

fn handle_close(stack: &mut Vec<char>, c: char) -> Result<(), char> {
    //println!("  looking to {}, current stack {:?}", c, stack);
    // top stack contains opening char
    let last: char = *stack.last().unwrap();
    // expected closing char
    let expected: char = *pairs().get(&last).expect("No such pair");
    if expected == c {
        stack.pop();
        Ok(())
    } else {
        Err(expected)
    }
}

enum BraceResult {
    Ok,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn process_line(line: &str) -> BraceResult {
    // Return () if okay or u32 with score if damaged
    let mut stack: Vec<char> = Vec::new();
    //println!("Processing line {}", line);
    for c in line.chars() {
        //println!(" processing symbol {}", c);
        let res = match c {
            '(' | '[' | '{' | '<' => handle_open(&mut stack, c),
            ')' | ']' | '}' | '>' => handle_close(&mut stack, c),
            _ => panic!("unknown symbol {}", c),
        };
        if let Err(expected) = res {
            println!("Expected {}, but found {} instead", expected, c);
            return BraceResult::Corrupted(c);
        }
    }
    if stack.is_empty() {
        BraceResult::Ok
    } else {
        let mut remaining: Vec<char> = Vec::new();
        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            remaining.push(*pairs().get(&c).unwrap());
        }
        BraceResult::Incomplete(remaining)
    }
}

pub fn process_a(lines: Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| match process_line(line) {
            BraceResult::Ok => 0,
            BraceResult::Corrupted(c) => {
                let score = score_a(c);
                println!("Registered score {} for line {}\n\n", score, line);
                score as u64
            }
            BraceResult::Incomplete(_remaining) => 0,
        })
        .sum()
}

fn score_b_for_remaining(remaining: &[char]) -> u64 {
    remaining
        .iter()
        .map(|c| score_b(*c) as u64)
        .reduce(|accum, item| accum * 5 + item)
        .unwrap()
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let mut scores: Vec<u64> = lines
        .iter()
        .map(|line| match process_line(line) {
            BraceResult::Ok => 0,
            BraceResult::Corrupted(_c) => 0,
            BraceResult::Incomplete(remaining) => {
                println!("For line {}, remaining {:?}", line, remaining);
                score_b_for_remaining(&remaining)
            }
        })
        // drop corrupted and normal scores
        .filter(|score| *score > 0)
        .collect();

    scores.sort();
    println!("scores {:?}", scores);
    let ind: usize = scores.len() / 2;
    scores[ind]
}
