use anyhow::{anyhow, Context};
use atty::Stream;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;

mod common;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

/* Common functions */

fn read_lines_stdin() -> anyhow::Result<Vec<String>> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    Ok(lines)
}

fn read_lines_from_file(filename: String) -> anyhow::Result<Vec<String>> {
    let file = std::fs::File::open(filename)?;
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    Ok(lines)
}

fn check_answer(answer: u64, correct_answer: u64) -> anyhow::Result<()> {
    if answer == correct_answer {
        println!("{}", format!("CORRECT: {}", answer).green());
        Ok(())
    } else {
        println!(
            "{}",
            format!("incorrect: {}, expected {}", answer, correct_answer).red()
        );
        Err(anyhow!("incorrect"))
    }
}

enum TaskType {
    A,
    B,
}

type FnProcess = Box<dyn Fn(Vec<String>) -> u64>;
struct Task {
    answer: common::Answer,
    process_a: FnProcess,
    process_b: FnProcess,
}
type Registry = std::collections::HashMap<usize, Task>;

/// Macro to register one day from module `$module`
macro_rules! day {
    ($registry: ident, $module: ident) => {
        //ANSWER::path is like aoc2021::dayX
        // Extract day number from module name
        let path = $module::ANSWER.path;
        let re: Regex = Regex::new(r".*::day(?P<num>[[:digit:]]+)$")?;
        let day: usize = re
            .captures(path)
            .context("No match")?
            .name("num")
            .context("No match")?
            .as_str()
            .parse()?;
        //println!("Registered module {} for day num '{}'", path, day);
        $registry.insert(
            day,
            Task {
                answer: $module::ANSWER,
                process_a: Box::new($module::process_a),
                process_b: Box::new($module::process_b),
            },
        );
    };
}

/// Macro to register all days
macro_rules! days {
    ($registry: ident, ($( $daymod: ident ), +)) => {
        $(
            day!($registry, $daymod);
        )+
    };
}

/// Collect all days from crate
fn init_registry() -> anyhow::Result<Registry> {
    let mut registry: Registry = HashMap::new();
    days!(
        registry,
        (
            day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
            day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
        )
    );
    Ok(registry)
}

fn main() -> anyhow::Result<()> {
    let day: usize = env::args().nth(1).expect("provide day").parse()?;
    println!("{}", format!("Run day {}", day).blue());
    let lines: Vec<String> = match env::args().nth(3) {
        Some(filename) => read_lines_from_file(filename)?,
        None => {
            if !atty::is(Stream::Stdin) {
                read_lines_stdin()?
            } else {
                read_lines_from_file(format!("data/day{}.dat", day))?
            }
        }
    };
    let task_type: TaskType = match env::args()
        .nth(2)
        .context("Expected a or b as second argument")?
        .as_str()
    {
        "a" => Ok(TaskType::A),
        "b" => Ok(TaskType::B),
        _ => Err(anyhow!("wrong task type")),
    }?;
    let registry = init_registry()?;
    let task: &Task = registry.get(&day).context("No such day")?;
    let answer: u64 = match task_type {
        TaskType::A => (task.process_a)(lines),
        TaskType::B => (task.process_b)(lines),
    };
    let expected_answer: u64 = match task_type {
        TaskType::A => task.answer.a,
        TaskType::B => task.answer.b,
    };
    check_answer(answer, expected_answer)
}
