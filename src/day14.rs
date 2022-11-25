use std::collections::HashMap;
use std::fmt;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(1588, 2188189693529);

/* Impl */

/// Trait for polymer type
trait PolymerTrait {
    fn new(s: &str) -> Self;
    fn len(&self) -> usize;
    fn score(&self) -> u64;
    fn iterate(polymer: Self, rules: &[Rule]) -> Self;
}

/* Slow variant */

struct Polymer {
    chars: Vec<char>,
}

type Rule = (char, char, char);

impl fmt::Debug for Polymer {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(
            f,
            "{}",
            String::from_utf8(self.chars.iter().map(|c| *c as u8).collect()).unwrap()
        )?;
        write!(f, "}}")
    }
}

impl PolymerTrait for Polymer {
    fn new(s: &str) -> Polymer {
        let chars = s.chars().collect();
        Polymer { chars }
    }

    fn len(&self) -> usize {
        self.chars.len()
    }

    fn score(&self) -> u64 {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for c in self.chars.iter() {
            counter.insert(*c, counter.get(c).unwrap_or(&0) + 1);
        }
        println!("{:?}", counter);
        let value_max = counter.values().max().unwrap();
        let value_min = counter.values().min().unwrap();
        (value_max - value_min) as u64
    }

    fn iterate(polymer: Polymer, rules: &[Rule]) -> Polymer {
        //let new_polymer = Polymer { chars: polymer.chars.clone() };
        let chars = polymer.chars;
        let mut new_chars: Vec<char> = Vec::new();

        let iter1 = chars.iter();
        let mut iter2 = chars.iter();
        iter2.next();
        for (c1, c2) in iter1.zip(iter2) {
            //println!("Looking to {} {}", c1, c2);
            new_chars.push(*c1);
            if let Some(applied_rule) = rules.iter().find(|(a, b, _r)| *a == *c1 && *b == *c2) {
                //println!("Matched rule {:?}", applied_rule);
                new_chars.push(applied_rule.2);
            }
        }
        new_chars.push(chars[chars.len() - 1]);
        Polymer { chars: new_chars }
    }
}

/* Fast variant */

type Pair = (char, char);

struct FastPolymer {
    pairs: HashMap<Pair, usize>,
    counter: HashMap<char, usize>,
}

impl fmt::Debug for FastPolymer {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(
            f,
            "pairs count={:?} {:?} counter={:?}",
            self.pairs.len(),
            self.pairs,
            self.counter
        )?;
        write!(f, "}}")
    }
}

impl PolymerTrait for FastPolymer {
    fn new(s: &str) -> FastPolymer {
        let chars: Vec<char> = s.chars().collect();
        let iter1 = chars.iter();
        let mut iter2 = chars.iter();
        iter2.next();
        let raw_pairs: Vec<Pair> = iter1.zip(iter2).map(|(c1, c2)| (*c1, *c2)).collect();

        let mut pairs: HashMap<Pair, usize> = HashMap::new();
        for pair in raw_pairs {
            pairs.insert(pair, pairs.get(&pair).unwrap_or(&0) + 1);
        }

        let mut counter: HashMap<char, usize> = HashMap::new();
        for c in chars.iter() {
            counter.insert(*c, counter.get(c).unwrap_or(&0) + 1);
        }
        FastPolymer { pairs, counter }
    }

    fn len(&self) -> usize {
        self.counter.values().sum()
    }

    fn score(&self) -> u64 {
        let value_max = self.counter.values().max().unwrap();
        let value_min = self.counter.values().min().unwrap();
        (value_max - value_min) as u64
    }

    fn iterate(polymer: FastPolymer, rules: &[Rule]) -> FastPolymer {
        let mut new_counter = polymer.counter;
        let pairs = polymer.pairs;
        let mut new_pairs: HashMap<Pair, usize> = HashMap::new();

        for pair in pairs.keys() {
            println!("Looking to {} {}", pair.0, pair.1);
            if let Some(applied_rule) = rules.iter().find(|(a, b, _r)| *a == pair.0 && *b == pair.1)
            {
                //println!("Matched rule {:?}", applied_rule);
                let new_char = applied_rule.2;

                let orig_pair: Pair = (pair.0, pair.1);
                // how many times pair was found in original polymer
                let encountered: usize = *pairs.get(&orig_pair).unwrap_or(&0);

                let pair1: Pair = (pair.0, new_char);
                let pair2: Pair = (new_char, pair.1);

                *new_pairs.entry(pair1).or_default() += encountered;
                *new_pairs.entry(pair2).or_default() += encountered;

                //println!(" increasing {} by {}", new_char, encountered);
                // increase new character counter by times of original pair was found
                //new_counter.insert(new_char, new_counter.get(&new_char).unwrap_or(&0) + encountered);
                *new_counter.entry(new_char).or_default() += encountered;
            }
        }
        FastPolymer {
            pairs: new_pairs,
            counter: new_counter,
        }
    }
}

fn process_uni<T: PolymerTrait + fmt::Debug>(lines: Vec<String>, iters: usize) -> u64 {
    let mut liter = lines.iter();
    let mut polymer = T::new(liter.next().unwrap());
    liter.next(); // empty line
    let rules: Vec<Rule> = liter
        .map(|s| {
            let (l, r) = s.split_once("->").unwrap();
            let a = l.trim().chars().next().unwrap();
            let b = l.trim().chars().nth(1).unwrap();
            let c = r.trim().chars().next().unwrap();
            (a, b, c) as Rule
        })
        .collect();
    println!("Polymer: {:?}", polymer);
    println!("Rules: {:?}", rules);
    for iter in 0..iters {
        println!("After step {}: {:?}", iter, polymer);
        polymer = T::iterate(polymer, &rules);
        //println!("After step {} len {}", iter, polymer.len());
    }
    polymer.score() as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    process_uni::<FastPolymer>(lines, 10)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_uni::<FastPolymer>(lines, 40)
}
