use std::collections::HashMap;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(739785, 444_356_092_776_315);

/* Impl */

fn parse_init(s: &str) -> u64 {
    s.split(':')
        .next_back()
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap()
}

fn roll(dice: &mut u64) -> u64 {
    let result = *dice;
    if *dice < 100 {
        *dice += &1;
    } else {
        *dice = 1;
    }
    result
}

fn movement(_first: bool, dice: &mut u64, pos: &mut u64, score: &mut u64) {
    //let orig_dice = *dice;
    let value = roll(dice) + roll(dice) + roll(dice);
    *pos = 1 + (*pos + value - 1).wrapping_rem(10);
    *score += *pos;
    //println!(
    //"player {} rolls {} total {} and moves to {} total {}",
    //if first { 1 } else { 2 },
    //orig_dice,
    //value,
    //*pos,
    //*score
    //);
}

#[allow(unreachable_code)]
pub fn process_a(lines: Vec<String>) -> u64 {
    let init1: u64 = parse_init(&lines[0]);
    let init2: u64 = parse_init(&lines[1]);
    let mut pos1 = init1;
    let mut pos2 = init2;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut dice = 1u64;
    let mut rolls = 0;
    loop {
        movement(true, &mut dice, &mut pos1, &mut score1);
        rolls += 3;
        if score1 >= 1000 {
            println!(
                "dice {}, rolls {}, scores {} vs {}",
                dice, rolls, score1, score2
            );
            return score2 * rolls;
        }
        movement(false, &mut dice, &mut pos2, &mut score2);
        rolls += 3;
        if score2 >= 1000 {
            println!(
                "dice {}, rolls {}, scores {} vs {}",
                dice, rolls, score1, score2
            );
            return score1 * rolls;
        }
    }
    0
}

fn dice_code_to_value(value: u64) -> u64 {
    assert!(value < 27);
    let mut xvalue = value;
    let c = xvalue.wrapping_rem(3);
    xvalue = xvalue.wrapping_div(3);
    let b = if xvalue > 0 {
        xvalue.wrapping_rem(3)
    } else {
        0
    };
    xvalue = xvalue.wrapping_div(3);
    let a = if xvalue > 0 {
        xvalue.wrapping_rem(3)
    } else {
        0
    };
    a + b + c + 3
}

fn calc_pos_and_score(pos: u64, score: u64, dice_value: u64) -> (u64, u64) {
    let npos = 1 + (pos + dice_value - 1).wrapping_rem(10);
    let nscore = score + npos;
    (npos, nscore)
}

// Memoization
type Memo = HashMap<(bool, u64, u64, u64, u64), (u64, u64)>;

const MAX_SCORE: u64 = 21;

fn play(
    first: bool,
    pos1: u64,
    pos2: u64,
    score1: u64,
    score2: u64,
    memo: &mut Memo,
) -> (u64, u64) {
    // cache values
    let memo_key = (first, pos1, pos2, score1, score2);
    if let Some(cached) = memo.get(&memo_key) {
        return *cached;
    }

    // return number of wins for each player

    if score1 >= MAX_SCORE {
        return (1, 0);
    }
    if score2 >= MAX_SCORE {
        return (0, 1);
    }

    let mut accwin1 = 0;
    let mut accwin2 = 0;

    for next_code in 0..27 {
        let roll = dice_code_to_value(next_code);

        let (subwin1, subwin2) = if first {
            let (npos, nscore) = calc_pos_and_score(pos1, score1, roll);
            play(!first, npos, pos2, nscore, score2, memo)
        } else {
            let (npos, nscore) = calc_pos_and_score(pos2, score2, roll);
            play(!first, pos1, npos, score1, nscore, memo)
        };
        accwin1 += subwin1;
        accwin2 += subwin2;
    }
    // save to cache
    memo.insert(memo_key, (accwin1, accwin2));
    (accwin1, accwin2)
}

/*
fn play_dp(init1: u64, init2: u64) -> (u64, u64) {
    let mut table: HashMap<(bool, u64, u64, u64, u64), (u64, u64)> = HashMap::new();

    // init table
    for side in [false, true] {
        for pos1 in 0..11 {
            for pos2 in 0..11 {
                for score1 in 0..MAX_SCORE+5 {
                    for score2 in 0..MAX_SCORE+5 {
                        let cpos1 = if score1 == 0 { init1 } else { pos1 };
                        let cpos2 = if score2 == 0 { init2 } else { pos2 };
                        let key = (side, cpos1, cpos2, score1, score2);
                        table.insert(key, (0,0));
                    }
                }
            }
        }
    }

    for _side in [false, true] {
        for pos1 in (1..11).rev() {
            for pos2 in (1..11).rev() {
                for score1 in (0..MAX_SCORE+1).rev() {
                    for score2 in (0..MAX_SCORE+1).rev() {

                        for next_code in 0..27 {
                            let roll = dice_code_to_value(next_code);
                            println!("roll {}, pos {}/{} score {}/{}", roll, pos1, pos2, score1, score2);
                            // first player
                            let (_npos, nscore) = calc_pos_and_score(pos1, score1, roll);
                            if nscore >= MAX_SCORE{
                                let key1 = (true, pos1, score1, pos2, score2);
                                table.get_mut(&key1).unwrap().0 += 1;
                            }

                            // second player
                            let (_npos, nscore) = calc_pos_and_score(pos2, score2, roll);
                            if nscore >= MAX_SCORE {
                                let key2 = (false, pos1, score1, pos2, score2);
                                //println!("{:?}", key2);
                                table.get_mut(&key2).unwrap().1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    //let mut accwin1 = 0;
    //let mut accwin2 = 0;

    //for value in table.values() {
        //accwin1 += value.0;
        //accwin2 += value.1;
    //}
    //(accwin1, accwin2)

    (table.values().map(|x| x.0 ).sum(), table.values().map(|x| x.1 ).sum())
}*/

#[allow(dead_code)]
fn play_dp(init1: u64, _init2: u64) -> (u64, u64) {
    let mut table: HashMap<(u64, u64), u64> = HashMap::new();

    // init table
    for pos in 0..11 {
        for score in 0..MAX_SCORE + 1 {
            let cpos1 = if score == 0 { init1 } else { pos };
            let key = (cpos1, score);
            table.insert(key, 0);
        }
    }

    for pos1 in (1..11).rev() {
        for pos2 in (1..11).rev() {
            for score1 in (0..MAX_SCORE + 1).rev() {
                for score2 in (0..MAX_SCORE + 1).rev() {
                    for next_code in 0..27 {
                        let roll = dice_code_to_value(next_code);
                        println!(
                            "roll {}, pos {}/{} score {}/{}",
                            roll, pos1, pos2, score1, score2
                        );
                        // first player
                        let (_npos, nscore) = calc_pos_and_score(pos1, score1, roll);
                        if nscore >= MAX_SCORE {
                            let _key1 = (true, pos1, score1, pos2, score2);
                            //table.get_mut(&key1).unwrap().0 += 1;
                        }
                    }
                }
            }
        }
    }

    //let mut accwin1 = 0;
    //let mut accwin2 = 0;

    //for value in table.values() {
    //accwin1 += value.0;
    //accwin2 += value.1;
    //}
    //(accwin1, accwin2)

    //(table.values().map(|x| x.0 ).sum(), table.values().map(|x| x.1 ).sum())
    (0, 0)
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let init1: u64 = parse_init(&lines[0]);
    let init2: u64 = parse_init(&lines[1]);
    let pos1 = init1;
    let pos2 = init2;

    let mut memo = Memo::new();
    let (wins1, wins2) = play(true, pos1, pos2, 0, 0, &mut memo);
    println!("{}", memo.len());
    for k in memo.keys() {
        println!("{:?}", k);
    }
    wins1.max(wins2)
}

fn _process_b_dp(lines: Vec<String>) -> u64 {
    let init1: u64 = parse_init(&lines[0]);
    let init2: u64 = parse_init(&lines[1]);

    let (wins1, wins2) = play_dp(init1, init2);
    wins1.max(wins2)
}
