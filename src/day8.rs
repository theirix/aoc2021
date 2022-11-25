use std::collections::HashMap;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(26, 61229);

/* Impl */

fn is_unique(s: &str) -> bool {
    s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7
}

fn find_digit(digits: &[&str], count: usize) -> Vec<char> {
    if let Some(v) = digits.iter().find(|d| d.len() == count) {
        v.chars().collect::<Vec<char>>()
    } else {
        panic!()
    }
}

fn subset(larger: &[char], smaller: &[char]) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for l in larger {
        if !smaller.contains(l) {
            result.push(*l);
        }
    }
    result
}

fn find_mapping(input: &[&str]) -> HashMap<char, char> {
    println!("In {:?}", input);
    // wire to real digit
    let mut mapping: HashMap<char, char> = HashMap::new();
    let dig1 = find_digit(input, 2);
    let dig7 = find_digit(input, 3);
    let dig4 = find_digit(input, 4);
    let dig8 = find_digit(input, 7);
    println!("1={:?}, 7={:?} 4={:?} 8={:?}", dig1, dig7, dig4, dig8);

    // a must be 8 times
    // b must be 6 times
    // c must be 8 times
    // d must be 7 times
    // e must be 4 times
    // f must be 9 times
    // g must be 7 times
    let frequences: HashMap<char, usize> = HashMap::from_iter(
        "abcdefg"
            .chars()
            .map(|c| (c, input.iter().filter(|one| one.contains(c)).count())),
    );
    println!("Frequences: {:?}", frequences);

    // Handle digit 1
    let remaining1 = &dig1;
    println!("for 1 {:?}", remaining1);

    let count_c = *frequences.get(&remaining1[0]).unwrap();
    let count_f = *frequences.get(&remaining1[1]).unwrap();
    // c must be 8 times
    // f must be 9 times
    if count_c == 8 && count_f == 9 {
        mapping.insert(remaining1[0], 'c');
        mapping.insert(remaining1[1], 'f');
    } else if count_f == 8 && count_c == 9 {
        mapping.insert(remaining1[1], 'c');
        mapping.insert(remaining1[0], 'f');
    } else {
        panic!();
    }

    // Handle digit 7
    let remaining7 = subset(&dig7, &dig1);
    println!("for 7 {:?}", remaining7);

    mapping.insert(remaining7[0], 'a');

    println!("Mapping so far {:?}", mapping);
    let remaining4 = subset(&dig4, &dig1);
    println!("for 4 {:?}", remaining4);

    let count_b = *frequences.get(&remaining4[0]).unwrap();
    let count_d = *frequences.get(&remaining4[1]).unwrap();
    println!("countb {}, countd {}", count_b, count_d);
    if count_b == 6 && count_d == 7 {
        // b must be 6 times
        // d must be 7 times
        mapping.insert(remaining4[0], 'b');
        mapping.insert(remaining4[1], 'd');
    } else if count_d == 6 && count_b == 7 {
        mapping.insert(remaining4[1], 'b');
        mapping.insert(remaining4[0], 'd');
    } else {
        panic!();
    }

    // Handle digit 9
    let existing: Vec<char> = mapping.keys().copied().collect();
    let remaining9 = subset(
        "abcdefg".chars().collect::<Vec<char>>().as_slice(),
        &existing,
    );
    println!("For 9 {:?}", remaining9);
    let count_g = *frequences.get(&remaining9[0]).unwrap();
    let count_e = *frequences.get(&remaining9[1]).unwrap();
    println!("count_g {}, count_e {}", count_g, count_e);
    if count_g == 7 && count_e == 4 {
        // g must be 7 times
        // e must be 4 times
        mapping.insert(remaining9[0], 'g');
        mapping.insert(remaining9[1], 'e');
    } else if count_e == 7 && count_g == 4 {
        mapping.insert(remaining9[1], 'g');
        mapping.insert(remaining9[0], 'e');
    } else {
        panic!();
    }

    println!("Mapping {:?}", mapping);
    assert_eq!(mapping.len(), 7);
    mapping
}

fn decode(mapping: &HashMap<char, char>, digits: &[&str]) -> u64 {
    let patterns: HashMap<Vec<char>, u64> = HashMap::from([
        ("abcefg".chars().collect(), 0),
        ("cf".chars().collect(), 1),
        ("acdeg".chars().collect(), 2),
        ("acdfg".chars().collect(), 3),
        ("bcdf".chars().collect(), 4),
        ("abdfg".chars().collect(), 5),
        ("abdefg".chars().collect(), 6),
        ("acf".chars().collect(), 7),
        ("abcdefg".chars().collect(), 8),
        ("abcdfg".chars().collect(), 9),
    ]);

    println!(" digits {:?}", digits);
    let decoded_digits: Vec<u64> = digits
        .iter()
        .map(|digit| {
            let mut mapped: Vec<char> = digit
                .chars()
                .map(|c| mapping.get(&c).unwrap())
                .copied()
                .collect();
            mapped.sort_unstable();
            *patterns.get(&mapped).unwrap()
        })
        .collect();
    let decoded_number: u64 = decoded_digits
        .iter()
        .enumerate()
        .map(|(i, v)| v * 10u64.pow(3u32 - (i as u32)))
        .sum::<u64>();
    decoded_number
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let outs: Vec<Vec<&str>> = lines
        .iter()
        .map(|s| {
            s.split_once('|')
                .unwrap()
                .1
                .split_whitespace()
                .collect::<Vec<&str>>()
        })
        .collect();
    println!("Outs {:?}", outs);
    outs.iter()
        .map(|digits| digits.iter().filter(|d| is_unique(d)).count())
        .sum::<usize>() as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let ins: Vec<Vec<&str>> = lines
        .iter()
        .map(|s| {
            s.split_once('|')
                .unwrap()
                .0
                .split_whitespace()
                .collect::<Vec<&str>>()
        })
        .collect();
    let outs: Vec<Vec<&str>> = lines
        .iter()
        .map(|s| {
            s.split_once('|')
                .unwrap()
                .1
                .split_whitespace()
                .collect::<Vec<&str>>()
        })
        .collect();
    println!("Ins {:?}", ins);
    println!("Outs {:?}", outs);

    let decoded_numbers: Vec<u64> = ins
        .iter()
        .zip(outs.iter())
        .map(|(inp, digits)| decode(&find_mapping(inp), digits))
        .collect();
    println!("{:?}", decoded_numbers);
    decoded_numbers.iter().sum()
}
