use crate::{answer, common::Answer};
use std::collections::HashMap;

pub const ANSWER: Answer = answer!(0, 0);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl TryFrom<&str> for Register {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Err(format!("cannot convert {}", s)),
        }
    }
}

#[derive(Debug)]
enum Source {
    Register(Register),
    Value(isize),
}

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        if let Ok(register) = s.try_into() {
            Source::Register(register)
        } else {
            Source::Value(s.parse().unwrap())
        }
    }
}

#[derive(Debug)]
enum Op {
    Inp(Register),
    Add(Register, Source),
    Mul(Register, Source),
    Div(Register, Source),
    Mod(Register, Source),
    Eql(Register, Source),
}

fn read_op(line: &String) -> Op {
    let mut tokens = line.split(" ");
    let s_op = tokens.next().unwrap();
    if s_op == "inp" {
        let left: Register = tokens.next().unwrap().try_into().unwrap();
        Op::Inp(left)
    } else {
        let left: Register = tokens.next().unwrap().try_into().unwrap();
        let right: Source = tokens.next().unwrap().into();
        match s_op {
            "add" => Op::Add(left, right),
            "mul" => Op::Mul(left, right),
            "div" => Op::Div(left, right),
            "mod" => Op::Mod(left, right),
            "eql" => Op::Eql(left, right),
            _ => panic!("bad op {}", &line),
        }
    }
}

fn read_ops(lines: Vec<String>) -> Vec<Op> {
    lines.iter().map(read_op).collect()
}

fn print_state(state: &HashMap<Register, isize>) {
    println!("X={} Y={} Z={} W={}", state[&Register::X], state[&Register::Y], state[&Register::Z], state[&Register::W]);
}

fn execute(program: &Vec<Op>, input: String) -> [isize; 4] {
    let verbose = false;
    let mut input_it = input.chars();
    let mut state: HashMap<Register, isize> = HashMap::from([
        (Register::X, 0),
        (Register::Y, 0),
        (Register::Z, 0),
        (Register::W, 0),
    ]);
    let mut idx_input = 0;
    let mut last_input : Option<isize> = None;
    let mut last_c = 0;
    for op in program {
        match op {
            Op::Inp(target) => {
                if last_input.is_some() {
                    if verbose {
                        print!("C={} ", last_c);
                        println!("X became {}", state[&Register::X]);
                        print!("input {}. output: ", last_input.unwrap()); print_state(&state);
                    }
                    idx_input += 1;
                }
                let val : isize = input_it.next().unwrap().to_string().parse().unwrap();
                assert!(val >= 1 && val <= 9);
                last_input = Some(val);
                state.insert(*target, val);
            },
            Op::Add(target, Source::Register(source)) => {
                *state.get_mut(&target).unwrap() += state[&source];
            }
            Op::Add(target, Source::Value(val)) => {
                if *target == Register::X && verbose {
                    print!("B={} ", val);
                }
                if *target == Register::Y {
                    last_c = *val;
                }
                *state.get_mut(&target).unwrap() += val;
            },
            Op::Mul(target, Source::Register(source)) => {
                *state.get_mut(&target).unwrap() *= state[&source];
            },
            Op::Mul(target, Source::Value(val)) => {
                *state.get_mut(&target).unwrap() *= val;
            },
            Op::Div(target, Source::Register(source)) => {
                *state.get_mut(&target).unwrap() /= state[&source];
            },
            Op::Div(target, Source::Value(val)) => {
                if verbose {
                    print!("--- i={}\n A={} ", idx_input, val);
                }
                *state.get_mut(&target).unwrap() /= val;
            },
            Op::Mod(target, Source::Register(source)) => {
                *state.get_mut(&target).unwrap() %= state[&source];
            },
            Op::Mod(target, Source::Value(val)) => {
                *state.get_mut(&target).unwrap() %= val;
            },
            Op::Eql(target, Source::Register(source)) => {
                let result = if state[&target] == state[&source] { 1 } else { 0 };
                state.insert(*target, result);
            },
            Op::Eql(target, Source::Value(val)) => {
                let result = if state[&target] == *val { 1 } else { 0 };
                state.insert(*target, result);
            },
        };
        //println!("after {op:?}"); print_state(&state);
    }
    if verbose {
        println!("X became {}", state[&Register::X]);
        print!("input {}. output ", last_input.unwrap()); print_state(&state);
    }
    [
        state[&Register::W],
        state[&Register::X],
        state[&Register::Y],
        state[&Register::Z],
    ]
}

fn is_valid(state: [isize; 4]) -> bool {
    state[3] == 0
}

pub fn process_generic(lines: Vec<String>) -> (u64, u64) {
    let ops = read_ops(lines);

    const DCOUNT : usize = 14;
    
    assert!(ops.len()/18 == 14);
    let mut va = Vec::<isize>::new();
    let mut vb = Vec::<isize>::new();
    let mut vc = Vec::<isize>::new();
    for i in 0..DCOUNT {
        if let Op::Div(Register::Z, Source::Value(val)) = &ops[i*18+4] {
            va.push(*val);
        }
        if let Op::Add(Register::X, Source::Value(val)) = &ops[i*18+5] {
            vb.push(*val);
        }
        if let Op::Add(Register::Y, Source::Value(val)) = &ops[i*18+15] {
            vc.push(*val);
        }
    }


    println!("{:?}", &va);
    println!("{:?}", &vb);
    println!("{:?}", &vc);

    let mut digits = [-1; DCOUNT];
    let mut max_val = 0;
    let mut min_val : usize = usize::MAX;
    for counter in 1111111..=9999999 {
        let mut x = counter;
        let mut any_zero = false;
        for i in 0..DCOUNT {
            if va[i] == 1 {
                digits[i] = x % 10;
                if digits[i] == 0 {
                    any_zero = true;
                }
                x = x / 10;
            } else {
                digits[i] = -1;
            }
        }
        if any_zero { 
            continue; 
        }
        
        // now fill empty values
        let mut zval = 0;
        let mut valid = true;
        for j in 0..DCOUNT {
            // digit must be filled based on previous values of z
            if digits[j] == -1 {
                // evaluate it based on current zval
                digits[j] = zval % 26 + vb[j];
                if digits[j] <= 0 || digits[j] > 9 {
                    valid = false;
                    break;
                }
            }
            if va[j] == 1 {
                zval = 26*zval + digits[j] + vc[j];
            }
            if va[j] == 26 {
                let boolx = (zval % 26 + vb[j]) != digits[j];
                if boolx {
                    zval = zval + digits[j] + vc[j] 
                } else {
                    zval = zval / 26;
                }
            }
        }
        if !valid {
            continue;
        }
        if zval == 0 {
            let input : String = digits.iter().map(|c| c.to_string() ).collect();
            let cur_val : usize = input.parse().unwrap();
            max_val = max_val.max(cur_val);
            min_val = min_val.min(cur_val);
        }
    }

    // recheck
    let is_valid_max = is_valid(execute(&ops, max_val.to_string()));
    let is_valid_min = is_valid(execute(&ops, max_val.to_string()));

    println!("max {} is valid: {}", max_val, is_valid_max);
    println!("min {} is valid: {}", min_val, is_valid_min);

    (max_val as u64, min_val as u64)
}

pub fn process_a(lines: Vec<String>) -> u64 {
    process_generic(lines).0
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_generic(lines).1
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = r"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";

    #[test]
    fn test_4digits() {
        let ops = read_ops(String::from(SAMPLE).split("\n").map(|s| s.into()).collect());
        // stores the lowest (1's) bit in z, the second-lowest (2's) bit in y,
        // the third-lowest (4's) bit in x, and the fourth-lowest (8's) bit in w:
        assert_eq!(execute(&ops, "9".into()), [1, 0, 0, 1]); // b1001
        assert_eq!(execute(&ops, "8".into()), [1, 0, 0, 0]); // b1000
    }
    
    #[test]
    fn test_valid() {
        let ops = read_ops(String::from(SAMPLE).split("\n").map(|s| s.into()).collect());
        assert!(!is_valid(execute(&ops, "9".into())));
        assert!(is_valid(execute(&ops, "8".into())));
    }
    
    // requires input
    #[ignore]
    #[test]
    fn test_real_random() {
        let ops = read_ops(include_str!("../data/day24.dat").split("\n").filter( |s| !s.is_empty() ).map(|s| s.into()).collect());
        assert!(!is_valid(execute(&ops, "28765432198765".into())));
    }
    
    // requires input
    #[ignore]
    #[test]
    fn test_real_one() {
        let ops = read_ops(include_str!("../data/day24.dat").split("\n").filter( |s| !s.is_empty() ).map(|s| s.into()).collect());
        assert!(is_valid(execute(&ops, "74391738991352".into())));
    }
}
