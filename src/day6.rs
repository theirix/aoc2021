use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(5934, 26984457539);

type Fish = u8;

fn read_numbers(lines: Vec<String>) -> Vec<i32> {
    let line = lines.first().unwrap();
    let numbers: Vec<i32> = line.trim().split(',').map(|s| s.parse().unwrap()).collect();
    numbers
}

fn make_fish(numbers: Vec<i32>) -> Vec<Fish> {
    let mut fish = Vec::new();
    for number in numbers {
        fish.push(number as Fish)
    }
    fish
}

fn grow_classic(days: usize, fish: &mut Vec<Fish>) {
    for day in 0..days {
        //let mut new_fish = Vec::new();
        let day_size = fish.len();
        //for afish in fish.iter_mut() {
        for ind in 0..day_size {
            //let &mut afish = &mut fish[ind];
            //println!("A fish {:?}", afish);
            if fish[ind] == 0 {
                fish[ind] = 6;
                fish.push(8);
            } else {
                fish[ind] -= 1;
                //*afish = *afish - 1;
            }
        }
        //fish.append(&mut new_fish);
        //println!("Fish after day {:>2}: {:?}", day+1, fish);
        println!("Fish after day {:>2} count={}", day + 1, fish.len());
    }
}

fn make_counter(fish: &[Fish]) -> Vec<usize> {
    let mut counter = vec![0; 9];
    for d in 0..9 {
        for afish in fish.iter() {
            if *afish == d {
                counter[d as usize] += 1;
            }
        }
    }
    counter
}

fn grow_exp(days: usize, fish: &mut Vec<Fish>) -> usize {
    let mut counter = make_counter(fish);
    for day in 0..days {
        let c0 = counter[0];
        for d in 0..6 {
            counter[d] = counter[d + 1];
        }
        counter[6] = c0 + counter[7];
        counter[7] = counter[8];
        counter[8] = c0;

        if false {
            // verify
            let counter_sum: usize = counter.iter().sum();
            let indices: Vec<usize> = (0..9).collect();
            println!("counter indices        {:?}", indices);
            println!("counter after  day {:>2}: {:?}", day + 1, counter);
            println!("counter sum {}", counter_sum);

            println!(
                "Fish before day {:>2} count={}, fish={:?}, \tcounter={:?}",
                day + 1,
                fish.len(),
                fish,
                make_counter(fish)
            );
            let day_size = fish.len();
            for ind in 0..day_size {
                if fish[ind] == 0 {
                    fish[ind] = 6;
                    fish.push(8);
                } else {
                    fish[ind] -= 1;
                }
            }
            println!(
                "Fish after  day {:>2} count={}, fish={:?}, \tcounter {:?}",
                day + 1,
                fish.len(),
                fish,
                make_counter(fish)
            );
            println!("\n");
            assert!(counter == make_counter(fish));
        }
    }
    counter.iter().sum()
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(lines);
    println!("Numbers: {:?}", numbers);
    let mut fish = make_fish(numbers);
    println!("Initial: {:?}", fish);
    grow_classic(80, &mut fish);
    fish.len() as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let numbers = read_numbers(lines);
    println!("Numbers: {:?}", numbers);
    let mut fish = make_fish(numbers);
    println!("Initial: {:?}", fish);
    grow_exp(256, &mut fish) as u64
}
