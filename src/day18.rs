use std::fmt;
use std::rc::Rc;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(4140, 3993);

/* Impl */

#[derive(Debug)]
//#[derive(Debug, PartialEq, Eq, Hash)]
enum Number {
    Value(u64),
    Pair(Rc<Number>, Rc<Number>),
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Value(v) => write!(fmt, "{}", v)?,
            Number::Pair(l, r) => write!(fmt, "[{},{}]", l, r)?,
        };
        Ok(())
    }
}

fn parse_impl(line: &str, ind: &mut usize, depth: usize) -> Option<Rc<Number>> {
    println!(
        "Input: {}, ind {}, depth {}, peek {}",
        line,
        *ind,
        depth,
        line.chars().nth(*ind).unwrap()
    );
    while *ind < line.len()
        && (line.chars().nth(*ind).unwrap() == ']' || line.chars().nth(*ind).unwrap() == ',')
    {
        *ind += 1;
    }
    if *ind == line.len() - 1 {
        panic!("at end");
    }
    if line.chars().nth(*ind).unwrap().is_ascii_digit() {
        let mut dig_ind = *ind;
        while line.chars().nth(dig_ind).unwrap().is_ascii_digit() {
            dig_ind += 1;
        }
        let cx = line.chars().nth(dig_ind).unwrap();
        assert!(cx == ']' || cx == ',');
        let val = line[*ind..dig_ind].parse::<u64>().unwrap();
        println!("Parsed value {}", val);
        *ind = dig_ind;
        return Some(Rc::new(Number::Value(val)));
    }
    if line.chars().nth(*ind).unwrap() == '[' {
        *ind += 1;
        let l = parse_impl(line, ind, depth + 1).unwrap();
        *ind += 1;
        let r = parse_impl(line, ind, depth + 1).unwrap();
        return Some(Rc::new(Number::Pair(l, r)));
    }
    panic!("at end: {}, ind {}, depth {}", line, *ind, depth);
}

fn parse_number(line: &str) -> Option<Rc<Number>> {
    let mut ind = 0;
    parse_impl(line, &mut ind, 0)
}

fn magnitude(node: &Rc<Number>) -> u64 {
    match node.as_ref() {
        Number::Value(v) => *v,
        Number::Pair(l, r) => magnitude(l) * 3 + magnitude(r) * 2,
    }
}

fn enumerate_numbers(node: &Rc<Number>, leafs: &mut Vec<Rc<Number>>) {
    match node.as_ref() {
        Number::Value(_) => leafs.push(node.clone()),
        Number::Pair(l, r) => {
            enumerate_numbers(l, leafs);
            enumerate_numbers(r, leafs);
        }
    }
}

fn find_exploded(node: &Rc<Number>, depth: usize) -> Option<(Rc<Number>, u64, u64)> {
    if let Number::Pair(l, r) = node.as_ref() {
        if depth >= 4 {
            if let Number::Value(l1) = l as &Number {
                if let Number::Value(r1) = r as &Number {
                    println!("explode {}", node);
                    return Some((node.clone(), *l1, *r1));
                }
            }
        }
        if let Some(res) = find_exploded(l, depth + 1) {
            return Some(res);
        }
        if let Some(res) = find_exploded(r, depth + 1) {
            return Some(res);
        }
    }
    None
}

fn rewrite_exploded(
    node: &Rc<Number>,
    exploded: &Rc<Number>,
    left: &Option<Rc<Number>>,
    right: &Option<Rc<Number>>,
    leftadd: u64,
    rightadd: u64,
) -> Rc<Number> {
    if Rc::ptr_eq(exploded, node) {
        return Rc::new(Number::Value(0));
    }
    match node.as_ref() {
        Number::Value(v) => {
            if let Some(lleft) = left {
                if Rc::ptr_eq(lleft, node) {
                    return Rc::new(Number::Value(*v + leftadd));
                }
            }
            if let Some(rright) = right {
                if Rc::ptr_eq(rright, node) {
                    return Rc::new(Number::Value(*v + rightadd));
                }
            }
            Rc::new(Number::Value(*v))
        }
        Number::Pair(l, r) => Rc::new(Number::Pair(
            rewrite_exploded(l, exploded, left, right, leftadd, rightadd),
            rewrite_exploded(r, exploded, left, right, leftadd, rightadd),
        )),
    }
}

fn explode(root: Rc<Number>) -> Option<Rc<Number>> {
    if let Some((exp, leftadd, rightadd)) = find_exploded(&root, 0) {
        if let Number::Pair(l, r) = exp.as_ref() as &Number {
            println!("Exploded {:?}", exp);
            let mut leafs: Vec<Rc<Number>> = vec![];
            enumerate_numbers(&root, &mut leafs);
            // Find left/right
            let mut left = None;
            let mut right = None;
            for ind in 0..leafs.len() {
                if Rc::ptr_eq(&leafs[ind], l) && ind > 0 {
                    left = Some(leafs[ind - 1].clone());
                }
                if Rc::ptr_eq(&leafs[ind], r) && ind < leafs.len() - 1 {
                    right = Some(leafs[ind + 1].clone());
                }
            }
            println!("Got left={:?} right={:?}", left, right);
            return Some(rewrite_exploded(
                &root, &exp, &left, &right, leftadd, rightadd,
            ));
        }
    }
    None
}

fn split_one(node: &Rc<Number>, hit: &mut bool) -> Rc<Number> {
    match node.as_ref() {
        Number::Value(v) => {
            if *v >= 10 && !*hit {
                let l = (*v as f64 / 2.0).floor() as u64;
                let r = (*v as f64 / 2.0).ceil() as u64;
                *hit = true;
                Rc::new(Number::Pair(
                    Rc::new(Number::Value(l)),
                    Rc::new(Number::Value(r)),
                ))
            } else {
                Rc::new(Number::Value(*v))
            }
        }
        Number::Pair(l, r) => Rc::new(Number::Pair(split_one(l, hit), split_one(r, hit))),
    }
}

fn split(root: Rc<Number>) -> Option<Rc<Number>> {
    let mut hit = false;
    let newroot = split_one(&root, &mut hit);
    if hit {
        Some(newroot)
    } else {
        None
    }
}

fn reduce(left: Rc<Number>, right: Rc<Number>) -> Rc<Number> {
    let mut newroot = Rc::new(Number::Pair(left, right));
    println!("after addition: {}", newroot);
    loop {
        let mut hit = false;
        let rewritten = explode(newroot.clone());
        if let Some(n) = rewritten {
            println!("after explode: {}", n);
            hit = true;
            newroot = n;
        };
        if !hit {
            let splitted = split(newroot.clone());
            if let Some(n) = splitted {
                println!("after split: {}", n);
                hit = true;
                newroot = n;
            }
        };
        if !hit {
            break;
        }
    }
    newroot
}

fn sum_all(numbers: &[Rc<Number>]) -> Rc<Number> {
    let mut root: Rc<Number> = numbers[0].clone();
    println!("Root number {}", root);
    for number in &numbers[1..] {
        println!("Add number {}", number);
        root = reduce(root, number.clone());
    }
    root
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let numbers: Vec<Rc<Number>> = lines.iter().map(|s| parse_number(s).unwrap()).collect();
    let root = sum_all(&numbers);
    magnitude(&root)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let numbers: Vec<Rc<Number>> = lines.iter().map(|s| parse_number(s).unwrap()).collect();
    let mut max_mag = 0;
    for a in &numbers {
        for b in &numbers {
            if !Rc::ptr_eq(a, b) {
                let reduced: Rc<Number> = reduce(a.clone(), b.clone());
                let mag = magnitude(&reduced);
                println!("TRY {} + {} = {} ({})", a, b, reduced, mag);
                max_mag = max_mag.max(mag);
            }
        }
    }
    max_mag
}

#[cfg(test)]
mod tests {
    use super::Number::{Pair, Value};
    use super::*;

    fn parse_numbers(s: &str) -> Vec<Rc<Number>> {
        s.lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| parse_number(s).unwrap())
            .collect()
    }

    #[test]
    fn test_parse1() {
        let input = r#"[1,2]"#;
        let n = parse_number(input).unwrap();
        let nexp = Rc::new(Pair(Rc::new(Value(1)), Rc::new(Value(2))));
        assert_eq!(format!("{}", n), format!("{}", nexp));
    }

    #[test]
    fn test_parse2() {
        let input = r#"[9,[8,7]]"#;
        let n = parse_number(input).unwrap();
        let nexp = Rc::new(Pair(
            Rc::new(Value(9)),
            Rc::new(Pair(Rc::new(Value(8)), Rc::new(Value(7)))),
        ));
        assert_eq!(format!("{}", n), format!("{}", nexp));
    }

    #[test]
    fn test_parse3() {
        let input = r#"[[1,2],3]"#;
        let n = parse_number(input).unwrap();
        let nexp = Rc::new(Pair(
            Rc::new(Pair(Rc::new(Value(1)), Rc::new(Value(2)))),
            Rc::new(Value(3)),
        ));
        assert_eq!(format!("{}", n), format!("{}", nexp));
    }

    #[test]
    fn test_parse4() {
        let input = r#"[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"#;
        parse_number(input).unwrap();
    }

    #[test]
    fn test_sample1() {
        let input = r#"
[1,1]
[2,2]
[3,3]
[4,4]
"#;
        let sum_value = sum_all(&parse_numbers(input));
        assert_eq!(format!("{}", sum_value), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_sample2() {
        let input = r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
"#;
        let sum_value = sum_all(&parse_numbers(input));
        assert_eq!(format!("{}", sum_value), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_sample3() {
        let input = r#"
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
"#;
        let sum_value = sum_all(&parse_numbers(input));
        assert_eq!(format!("{}", sum_value), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_explode1() {
        let n1 = parse_number("[[[[[9,8],1],2],3],4]").unwrap();
        let n2 = parse_number("[[[[0,9],2],3],4]").unwrap();
        assert_eq!(format!("{}", explode(n1).unwrap()), format!("{}", n2));
    }

    #[test]
    fn test_explode2() {
        let n1 = parse_number("[7,[6,[5,[4,[3,2]]]]]").unwrap();
        let n2 = parse_number("[7,[6,[5,[7,0]]]]").unwrap();
        assert_eq!(format!("{}", explode(n1).unwrap()), format!("{}", n2));
    }

    #[test]
    fn test_explode3() {
        let n1 = parse_number("[[6,[5,[4,[3,2]]]],1]").unwrap();
        let n2 = parse_number("[[6,[5,[7,0]]],3]").unwrap();
        assert_eq!(format!("{}", explode(n1).unwrap()), format!("{}", n2));
    }

    #[test]
    fn test_explode4() {
        let n1 = parse_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let n2 = parse_number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
        assert_eq!(format!("{}", explode(n1).unwrap()), format!("{}", n2));
    }

    #[test]
    fn test_split1() {
        let n1 = parse_number("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        let n2 = parse_number("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
        assert_eq!(format!("{}", split(n1).unwrap()), format!("{}", n2));
    }

    #[test]
    fn test_reduce() {
        let n1 = parse_number("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let n2 = parse_number("[1,1]").unwrap();
        let n3 = parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        let r3 = reduce(n1, n2);
        assert_eq!(format!("{}", n3), format!("{}", r3));
    }
}
