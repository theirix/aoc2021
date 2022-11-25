use regex::Regex;
use std::collections::HashSet;

use crate::{answer, common::Answer};

// a: 39
// sample2. answer was 623748
// b: run22b
pub const ANSWER: Answer = answer!(39, 2758514936282235);
//pub const ANSWER : Answer = answer!(590784, 2758514936282235);

/* Impl */

type Pos = (isize, isize, isize);

type Range = (isize, isize);

type Cube = (Range, Range, Range);

#[derive(Debug, PartialEq)]
struct Instruction {
    on: bool,
    xrange: Range,
    yrange: Range,
    zrange: Range,
}

fn parse_insruction(line: &str) -> Instruction {
    let re = Regex::new(
        r"(on|off) x=([0-9-]+)\.\.([0-9-]+),y=([0-9-]+)\.\.([0-9-]+),z=([0-9-]+)\.\.([0-9-]+)",
    )
    .unwrap();
    let cap = re.captures(line).unwrap();
    Instruction {
        on: cap.get(1).unwrap().as_str() == "on",
        xrange: (
            cap.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<isize>().unwrap(),
        ),
        yrange: (
            cap.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(5).unwrap().as_str().parse::<isize>().unwrap(),
        ),
        zrange: (
            cap.get(6).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(7).unwrap().as_str().parse::<isize>().unwrap(),
        ),
    }
}

fn read_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|s| parse_insruction(s)).collect()
}

fn resolve_a(instructions: Vec<Instruction>) -> u64 {
    let pmin: isize = -50;
    let pmax: isize = 50;

    let mut reactor: HashSet<Pos> = HashSet::new();
    let mut counter: usize = 0;
    for instruction in instructions {
        for x in instruction.xrange.0.max(pmin)..=instruction.xrange.1.min(pmax) {
            for y in instruction.yrange.0.max(pmin)..=instruction.yrange.1.min(pmax) {
                for z in instruction.zrange.0.max(pmin)..=instruction.zrange.1.min(pmax) {
                    if instruction.on {
                        reactor.insert((x, y, z));
                    } else {
                        reactor.remove(&(x, y, z));
                    }
                    counter += 1;
                }
            }
        }
    }
    println!("Iterated {} steps", counter);
    println!("Filled reactor hash with {} items", reactor.len());

    let mut counter = 0u64;
    for x in pmin..=pmax {
        for y in pmin..=pmax {
            for z in pmin..=pmax {
                if reactor.contains(&(x, y, z)) {
                    counter += 1;
                }
            }
        }
    }
    counter as u64
}

fn does_intersect(range1: &Range, range2: &Range) -> bool {
    // (x1min < x2max), (x1max > x2min)
    range1.0 <= range2.1 && range1.1 >= range2.0
}

#[allow(dead_code)]
fn intersection_size(range1: &Range, range2: &Range) -> usize {
    let l = range1.0.max(range2.0);
    let r = range1.1.min(range2.1);
    usize::try_from(r - l).unwrap() + 1
}

fn intersection(range1: &Range, range2: &Range) -> Range {
    let l = range1.0.max(range2.0);
    let r = range1.1.min(range2.1);
    (l, r)
}

fn _cube_intersection_volume(cube1: &Cube, cube2: &Cube) -> Option<usize> {
    let intersected = does_intersect(&cube1.0, &cube2.0)
        && does_intersect(&cube1.1, &cube2.1)
        && does_intersect(&cube1.2, &cube2.2);
    if !intersected {
        return None;
    }
    let volume = intersection_size(&cube1.0, &cube2.0)
        * intersection_size(&cube1.1, &cube2.1)
        * intersection_size(&cube1.2, &cube2.2);
    Some(volume)
}

fn cube_intersection(cube1: &Cube, cube2: &Cube) -> Option<Cube> {
    let intersected = does_intersect(&cube1.0, &cube2.0)
        && does_intersect(&cube1.1, &cube2.1)
        && does_intersect(&cube1.2, &cube2.2);
    if !intersected {
        return None;
    }
    Some((
        intersection(&cube1.0, &cube2.0),
        intersection(&cube1.1, &cube2.1),
        intersection(&cube1.2, &cube2.2),
    ))
}

fn cube_volume(cube: &Cube) -> usize {
    usize::try_from(
        (1 + cube.0 .1 - cube.0 .0) * (1 + cube.1 .1 - cube.1 .0) * (1 + cube.2 .1 - cube.2 .0),
    )
    .unwrap()
}

fn _resolve_b_volume(instructions: &mut Vec<Instruction>) -> u64 {
    //let cube1 : Cube = ( (0, 5),(0, 5),(0, 5) );
    //let cube2 : Cube = ( (3, 6),(4, 6),(4, 6) );
    //if let Some(vol) = cube_intersection_volume(&cube1, &cube2) {
    //println!("{}", vol);
    //}
    //

    let count = instructions.len();
    let mut volume: isize = 0;
    for idx in 0..count {
        let instruction = &instructions[idx];
        if instruction.on {
            let local_volume =
                cube_volume(&(instruction.xrange, instruction.yrange, instruction.zrange)) as isize;
            let (res, overflow) = volume.overflowing_add(local_volume);
            assert!(!overflow);
            volume = res;
            println!(
                "Instruction {} {:?} on has volume {} -> {}",
                idx + 1,
                instruction,
                local_volume,
                volume
            );
        } else {
            println!("Instruction {} {:?} off and ignored", idx + 1, instruction);
        }

        let mut intersecteds: Vec<Cube> = vec![];
        let mut ons: Vec<bool> = vec![];
        for idx2 in 0..idx {
            let instruction1 = &instructions[idx];
            let instruction2 = &instructions[idx2];
            if let Some(intersected) = cube_intersection(
                &(
                    instruction1.xrange,
                    instruction1.yrange,
                    instruction1.zrange,
                ),
                &(
                    instruction2.xrange,
                    instruction2.yrange,
                    instruction2.zrange,
                ),
            ) {
                intersecteds.push(intersected);
                ons.push(instruction2.on);
            }
        }

        println!("Got intersections: {:?}", &intersecteds);

        for intersected in &intersecteds {
            let intersection_volume = cube_volume(intersected);
            let (res, overflow) = volume.overflowing_sub(intersection_volume as isize);
            assert!(!overflow);
            volume = res;
            println!(
                "Substract intersection of {} -> {}",
                intersection_volume, volume
            );
        }

        if true {
            for i in 0..intersecteds.len() {
                for j in 0..i {
                    if let Some(intersected) = cube_intersection(&intersecteds[i], &intersecteds[j])
                    {
                        if ons[i] {
                            let intersection_volume = cube_volume(&intersected);
                            let (res, overflow) =
                                volume.overflowing_add(intersection_volume as isize);
                            assert!(!overflow);
                            volume = res;
                            println!(
                                "Add 2nd level intersection of {} -> {}",
                                intersection_volume, volume
                            );
                        }
                    }
                }
            }
        }

        println!("After instruction {} volume -> {}", idx + 1, volume);
    }
    u64::try_from(volume).unwrap()
}

fn resolve_b_iterative(instructions: &mut Vec<Instruction>) -> u64 {
    // Works. Iterative solution.
    // Inspired by
    // https://www.reddit.com/r/adventofcode/comments/rlxhmg/comment/hqxczc4
    let count = instructions.len();

    let mut cubes_add: Vec<Cube> = vec![];
    let mut cubes_sub: Vec<Cube> = vec![];

    for idx in 0..count {
        let instruction = &instructions[idx];
        let cur_cube = (instruction.xrange, instruction.yrange, instruction.zrange);

        let mut new_cubes_add: Vec<Cube> = vec![];
        let mut new_cubes_sub: Vec<Cube> = vec![];
        for cube in &cubes_add {
            if let Some(intersected) = cube_intersection(&cur_cube, &cube) {
                new_cubes_sub.push(intersected);
            }
        }
        for cube in &cubes_sub {
            if let Some(intersected) = cube_intersection(&cur_cube, cube) {
                new_cubes_add.push(intersected);
            }
        }

        // do not add 'off' cube to `cubes_add` but treat its intersections (two loops below)
        if instruction.on {
            cubes_add.push(cur_cube);
        }

        cubes_add.extend(new_cubes_add);
        cubes_sub.extend(new_cubes_sub);

        if false {
            println!(
                "After instruction {} cubes to add {}, to sub {}",
                idx + 1,
                cubes_add.len(),
                cubes_sub.len()
            );
            for cube in &cubes_add {
                println!(" add: {:?}", cube);
            }
            for cube in &cubes_sub {
                println!(" sub: {:?}", cube);
            }
        }
    }
    let volume: isize = cubes_add
        .iter()
        .map(|cube| cube_volume(cube) as isize)
        .sum::<isize>()
        - cubes_sub
            .iter()
            .map(|cube| cube_volume(cube) as isize)
            .sum::<isize>();
    u64::try_from(volume).unwrap()
}

fn _all_intersections(cubes: &Vec<Cube>) -> Vec<Cube> {
    let count = cubes.len();
    println!("Input {:?}", &cubes);
    let mut intersecteds: Vec<Cube> = vec![];
    for idx1 in 0..count {
        for idx2 in idx1 + 1..count {
            if let Some(intersected) = cube_intersection(&cubes[idx1], &cubes[idx2]) {
                intersecteds.push(intersected);
            }
            println!(
                "{} vs {} so far {}: {:?}",
                idx1,
                idx2,
                intersecteds.len(),
                &intersecteds
            );
        }
    }
    intersecteds
}

fn _resolve_b_hier(instructions: &mut Vec<Instruction>) -> u64 {
    // Hierarchical approach. Does not work...
    let count = instructions.len();

    let mut levels: Vec<Vec<Cube>> = vec![];

    // form level 0 from instructions
    levels.push(vec![]);
    for idx in 0..count {
        let instruction = &instructions[idx];
        let cur_cube = (instruction.xrange, instruction.yrange, instruction.zrange);
        levels[0].push(cur_cube);
    }
    println!("At level 0 there are {} items", &levels[0].len());

    for level in 1..count {
        levels.push(_all_intersections(&levels[level - 1]));
        println!(
            "At level {} there are {} items:",
            level,
            &levels[level].len()
        );
        for cube in &levels[level] {
            println!(" {:?}", cube);
        }
    }

    let mut volume: isize = 0;
    for level in 0..count {
        let eligible_cubes: Vec<Cube> = if level == 0 {
            levels[0]
                .iter()
                .zip(instructions.iter())
                .filter(|(_cube, instr)| instr.on)
                .map(|(cube, _instr)| *cube)
                .collect()
        } else {
            levels[level].clone()
        };
        let local_sum = eligible_cubes
            .iter()
            .map(|cube| cube_volume(cube) as isize)
            .sum::<isize>();
        if (level % 2) == 0 {
            volume += local_sum;
        } else {
            volume -= local_sum;
        }
    }
    u64::try_from(volume).unwrap()
}

fn _resolve_b(instructions: Vec<Instruction>) -> u64 {
    let pmin: isize = -50;
    let pmax: isize = 50;

    let xmin: isize = instructions.iter().map(|i| i.xrange.0).min().unwrap();
    let xmax: isize = instructions.iter().map(|i| i.xrange.1).max().unwrap();
    let ymin: isize = instructions.iter().map(|i| i.yrange.0).min().unwrap();
    let ymax: isize = instructions.iter().map(|i| i.yrange.1).max().unwrap();
    let zmin: isize = instructions.iter().map(|i| i.zrange.0).min().unwrap();
    let zmax: isize = instructions.iter().map(|i| i.zrange.1).max().unwrap();

    //let xcount : usize = (xmax-xmin+1) as usize;
    //let ycount : usize = (ymax-ymin+1) as usize;
    //let zcount : usize = (zmax-zmin+1) as usize;

    //let mut reactor : Vec<bool> = vec![false; xcount*ycount*zcount];
    let reactor: HashSet<Pos> = HashSet::new();
    let mut counter: usize = 0;
    println!("x {}, y {}, z {}", xmax - xmin, ymax - ymin, zmax - zmin);
    for instruction in instructions {
        //for x in instruction.xrange.0.max(pmin)..=instruction.xrange.1.min(pmax) {
        //for y in instruction.yrange.0.max(pmin)..=instruction.yrange.1.min(pmax) {
        //for z in instruction.zrange.0.max(pmin)..=instruction.zrange.1.min(pmax) {
        for _x in instruction.xrange.0..=instruction.xrange.1 {
            for _y in instruction.yrange.0..=instruction.yrange.1 {
                for _z in instruction.zrange.0..=instruction.zrange.1 {
                    //let pos =
                    //((x - xmin) as usize) * ycount * zcount
                    //+ ((y - ymin) as usize) * zcount
                    //+ (z - zmin) as usize;
                    //if x < pmin || y < pmin || z < pmin || x > pmax || y > pmax || z > pmax {
                    //continue;
                    //}
                    //if instruction.on {
                    //reactor.insert((x,y,z));
                    //} else {
                    //reactor.remove(&(x,y,z));
                    //}
                    counter += 1;
                }
            }
        }
    }
    println!("Iterated {} steps", counter);
    println!("Filled reactor hash with {} items", reactor.len());

    let mut counter = 0u64;
    for x in pmin..=pmax {
        for y in pmin..=pmax {
            for z in pmin..=pmax {
                //if x < pmin || y < pmin || z < pmin || x > pmax || y > pmax || z > pmax {
                //continue;
                //}
                if reactor.contains(&(x, y, z)) {
                    counter += 1;
                }
            }
        }
    }
    counter as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    resolve_a(read_instructions(lines))
}

pub fn process_b(lines: Vec<String>) -> u64 {
    resolve_b_iterative(&mut read_instructions(lines))
}
