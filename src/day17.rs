use itertools::Itertools;
use regex::Regex;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(45, 112);

/* Impl */

#[derive(Debug)]
struct Problem {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

fn parse_problem(line: &str) -> Problem {
    let re =
        Regex::new(r"target area: x=([0-9-]+)\.\.([0-9-]+), y=([0-9-]+)\.\.([0-9-]+)").unwrap();
    let cap = re.captures(line).unwrap();
    Problem {
        x1: cap.get(1).unwrap().as_str().parse().unwrap(),
        x2: cap.get(2).unwrap().as_str().parse().unwrap(),
        y1: cap.get(3).unwrap().as_str().parse().unwrap(),
        y2: cap.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn does_hit(problem: &Problem, x: i64, y: i64) -> bool {
    x >= problem.x1 && x <= problem.x2 && y >= problem.y1 && y <= problem.y2
}

fn simulate(problem: &Problem, mut vx: i64, mut vy: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut maxy = 0;
    let mut hit = false;
    for _step in 0..10000 {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
        maxy = maxy.max(y);
        if does_hit(problem, x, y) {
            hit = true;
            break;
        }
        if x > problem.x2 {
            break;
        }
    }
    if hit {
        Some(maxy)
    } else {
        None
    }
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let problem = parse_problem(lines.first().unwrap());
    (1..500)
        .cartesian_product(-500..500)
        .filter_map(|(vx, vy)| simulate(&problem, vx, vy))
        .max()
        .unwrap() as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let problem = parse_problem(lines.first().unwrap());
    (1..500)
        .cartesian_product(-500..500)
        .filter(|(vx, vy)| simulate(&problem, *vx, *vy).is_some())
        .count() as u64
}

