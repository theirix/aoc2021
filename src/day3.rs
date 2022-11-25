use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(198, 230);

pub fn process_a(lines: Vec<String>) -> u64 {
    let mut zeroes = Vec::new();
    let mut ones = Vec::new();
    for line in lines {
        if zeroes.is_empty() {
            zeroes.resize(line.len(), 0);
            ones.resize(line.len(), 0);
        }
        for (ind, c) in line.trim().chars().enumerate() {
            match c {
                '0' => zeroes[ind] += 1,
                '1' => ones[ind] += 1,
                _ => panic!(),
            }
        }
        println!("{}", line);
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for ind in 0..zeroes.len() {
        if ones[ind] > zeroes[ind] {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_num = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = isize::from_str_radix(&epsilon, 2).unwrap();
    println!(
        "\ngamma: {}, {}, {}, {}",
        gamma, gamma_num, epsilon, epsilon_num
    );
    (gamma_num * epsilon_num) as u64
}

fn detect_sensor(input: Vec<String>, prefer_one: bool) -> Result<u32, &'static str> {
    let mut data = input;
    let n = data[0].len();
    for i in 0..n {
        let ones = data
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '1')
            .count();
        let zeroes = data
            .iter()
            .filter(|x| x.chars().nth(i).unwrap() == '0')
            .count();
        let mut prefered;
        if prefer_one {
            if ones > zeroes {
                prefered = '1'
            } else if ones < zeroes {
                prefered = '0'
            } else {
                prefered = '1'
            }
            if i == n - 1 {
                prefered = '1'
            }
        } else {
            if ones > zeroes {
                prefered = '0'
            } else if ones < zeroes {
                prefered = '1'
            } else {
                prefered = '0'
            }
            if i == n - 1 {
                prefered = '0'
            }
        }

        data = data
            .iter()
            .filter(|datum| datum.chars().nth(i).unwrap() == prefered)
            .map(|x| x.to_string())
            .collect();
        println!(
            "at iter {} 1={} 0={} remaining {}",
            i,
            ones,
            zeroes,
            data.len()
        );
        for x in &data {
            println!(" {}", x);
        }
        if data.len() == 1 {
            let sensor = &data[0];
            let num = isize::from_str_radix(sensor, 2).unwrap();
            println!("found {} {}", sensor, num);
            return Ok(num as u32);
        }
    }
    Err("No data")
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let oxygen_value = detect_sensor(lines.clone(), true).unwrap();
    let co2_value = detect_sensor(lines, false).unwrap();
    (oxygen_value * co2_value) as u64
}
