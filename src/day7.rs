use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(37, 168);

type Pos = i32;

fn read_numbers(lines: Vec<String>) -> Vec<Pos> {
    let line = lines.first().unwrap();
    let numbers: Vec<Pos> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    numbers
}

type PathScore = fn(u32) -> u32;

fn calc_score(initial_pos: &[Pos], pos: &[Pos], path_score: PathScore) -> u32 {
    initial_pos
        .iter()
        .zip(pos)
        .map(|(a, b)| path_score((a - b).abs() as u32))
        .sum::<u32>()
}

fn find_best_aligned_pos(initial_pos: Vec<Pos>, path_score: PathScore) -> u64 {
    let mut best_align_pos = 0;
    let mut best_score: u32 = u32::max_value();
    for align_pos in 0..initial_pos.len() {
        let target_pos = vec![align_pos as Pos; initial_pos.len()];
        let score = calc_score(&initial_pos, &target_pos, path_score);
        if score < best_score {
            best_align_pos = align_pos;
            best_score = score;
        }
        println!("For pos {:?} score={}", target_pos, score);
    }
    println!("Best align pos is {}", best_align_pos);
    best_score as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let pos = read_numbers(lines);

    find_best_aligned_pos(pos, |x| x)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let pos = read_numbers(lines);
    let path_score_b = |n| (1..n + 1).sum::<u32>();

    find_best_aligned_pos(pos, path_score_b)
}
