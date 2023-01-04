use priority_queue::PriorityQueue;
use regex::Regex;
use std::cmp::Reverse;
use std::fmt;

use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(12521, 44169);

/* Impl */

type Amphipod = char;

const EMPTY: char = '.';
const HALLWAY: usize = 11;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    // down room goes first (0), then upper room (1)
    rooms: [Vec<Amphipod>; 4],
    hallway: [Amphipod; HALLWAY],
}

impl State {
    fn height(&self) -> usize {
        self.rooms[0].len()
    }
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "{}", self.hallway.iter().collect::<String>())?;
        for floor in (0..self.height()).rev() {
            writeln!(
                fmt,
                "  {} {} {} {}",
                self.rooms[0][floor],
                self.rooms[1][floor],
                self.rooms[2][floor],
                self.rooms[3][floor]
            )?;
        }
        Ok(())
    }
}

fn read_problem(lines: Vec<String>) -> State {
    let mut it = lines.iter();
    it.next();
    let hallway_s = it.next().unwrap();
    let hallway_size: usize = hallway_s.len() - 2;
    assert!(hallway_size == HALLWAY);
    let hallway: [char; HALLWAY] = hallway_s.chars().collect::<Vec<char>>()[1..HALLWAY + 1]
        .try_into()
        .unwrap();
    let mut_rooms: [&mut Vec<Amphipod>; 4] = [
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
    ];
    let re = Regex::new(r"([ABCD\.])").unwrap();
    for _ in 0..lines.len() - 2 {
        let line = it.next().unwrap();
        for (idx, m) in re.find_iter(line).enumerate() {
            let a: Amphipod = m.as_str().chars().next().unwrap();
            mut_rooms[idx].push(a);
        }
    }
    let rooms = mut_rooms.map(|m| m.iter().copied().rev().collect());
    State { rooms, hallway }
}

fn is_final(state: &State) -> bool {
    state.hallway.iter().all(|r| *r == EMPTY)
        && (0..state.height())
            .all(|floor| (0..4).all(|room| home_room(state.rooms[room][floor]) == room))
}

fn cost(amphipod: Amphipod) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Not an amphipod {}", amphipod),
    }
}

fn home_room(amphipod: Amphipod) -> usize {
    match amphipod {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("Not an amphipod {}", amphipod),
    }
}

fn room_to_hallway(room_from: usize, hpos_to: usize, state: &State) -> Option<(State, usize)> {
    let hpos_from: usize = 2 + room_from * 2;

    let hpos1 = hpos_from.min(hpos_to);
    let hpos2 = hpos_from.max(hpos_to);
    for h in hpos1..=hpos2 {
        if state.hallway[h] != EMPTY {
            return None;
        }
    }

    let floor_from: usize = (0..state.height())
        .rev()
        .find(|h| state.rooms[room_from][*h] != EMPTY)?;
    let mut rooms = state.rooms.clone();
    let amphipod = rooms[room_from][floor_from];
    rooms[room_from][floor_from] = EMPTY;
    let mut hallway = state.hallway;
    hallway[hpos_to] = amphipod;
    let new_state = State { rooms, hallway };
    let moves: usize = hpos_to.abs_diff(hpos_from) + state.height().abs_diff(floor_from);
    let cost: usize = cost(amphipod) * moves;
    Some((new_state, cost))
}

fn hallway_to_room(hpos_from: usize, room_to: usize, state: &State) -> Option<(State, usize)> {
    let amphipod = state.hallway[hpos_from];
    if home_room(amphipod) != room_to {
        return None;
    }

    if (0..state.height())
        .any(|h| state.rooms[room_to][h] != EMPTY && state.rooms[room_to][h] != amphipod)
    {
        return None;
    }

    let hpos_to: usize = 2 + room_to * 2;
    let hpos1 = hpos_from.min(hpos_to);
    let hpos2 = hpos_from.max(hpos_to);
    for h in hpos1..=hpos2 {
        if h != hpos_from && state.hallway[h] != EMPTY {
            return None;
        }
    }

    let floor_to: usize = (0..state.height()).find(|h| state.rooms[room_to][*h] == EMPTY)?;
    let mut rooms = state.rooms.clone();
    rooms[room_to][floor_to] = amphipod;
    let mut hallway = state.hallway;
    hallway[hpos_from] = EMPTY;
    let new_state = State { rooms, hallway };
    let moves = hpos_from.abs_diff(hpos_to) + state.height().abs_diff(floor_to);
    let cost = cost(amphipod) * moves;
    Some((new_state, cost))
}

fn derive_states(state: &State) -> Vec<(State, usize)> {
    let mut result = vec![];

    // 1. From room to hallway
    for room_from in 0..4 {
        if (0..state.height()).all(|h| {
            state.rooms[room_from][h] == EMPTY || home_room(state.rooms[room_from][h]) == room_from
        }) {
            continue;
        }
        for hpos in 0..HALLWAY {
            if state.hallway[hpos] == EMPTY {
                // do not stop above room
                if hpos == 0 || hpos == HALLWAY - 1 || hpos % 2 != 0 {
                    if let Some((new_state, move_cost)) = room_to_hallway(room_from, hpos, state) {
                        result.push((new_state, move_cost));
                    }
                }
            }
        }
    }
    // 2. From hallway to room
    for hpos in 0..HALLWAY {
        if state.hallway[hpos] == EMPTY {
            continue;
        }
        let amphipod = state.hallway[hpos];
        for room_to in 0..4 {
            if home_room(amphipod) == room_to {
                if let Some((new_state, move_cost)) = hallway_to_room(hpos, room_to, &state) {
                    result.push((new_state, move_cost));
                }
            }
        }
    }
    // 3. From room to room
    for room_from in 0..4 {
        if (0..state.height()).all(|h| {
            state.rooms[room_from][h] == EMPTY || home_room(state.rooms[room_from][h]) == room_from
        }) {
            continue;
        }
        for room_to in 0..4 {
            if room_to == room_from {
                continue;
            }
            if (0..state.height()).all(|h| state.rooms[room_to][h] != EMPTY) {
                continue;
            }

            let hpos = 2 + room_from * 2;
            if let Some((interm_state, interm_cost)) = room_to_hallway(room_from, hpos, state) {
                if let Some((res_state, res_cost)) = hallway_to_room(hpos, room_to, &interm_state) {
                    result.push((res_state, res_cost + interm_cost));
                }
            }
        }
    }
    result
}

fn resolve_pq(init_state: State) -> u64 {
    println!("Init\n{}", init_state);

    let mut best_cost: usize = usize::MAX;
    let mut pq: PriorityQueue<State, Reverse<usize>> = PriorityQueue::new();

    pq.push(init_state, Reverse(0));

    while let Some((state, rcost)) = pq.pop() {
        let cost: usize = rcost.0; // get from Reverse wrapper

        if is_final(&state) {
            best_cost = best_cost.min(cost);
            continue;
        }

        for (new_state, move_cost) in derive_states(&state) {
            // Prune tree starting from this state
            if cost + move_cost >= best_cost {
                continue;
            }
            if let Some(existing_cost) = pq.get_priority(&new_state) {
                if cost + move_cost < existing_cost.0 {
                    // Update to lower cost for existing state
                    pq.push(new_state, Reverse(cost + move_cost));
                }
            } else {
                // Insert new state
                pq.push(new_state, Reverse(cost + move_cost));
            }
        }
    }

    best_cost as u64
}

pub fn process_a(lines: Vec<String>) -> u64 {
    resolve_pq(read_problem(lines))
}

pub fn process_b(lines: Vec<String>) -> u64 {
    process_a(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    static CASE_1: &str = "
#############
#.......A.C.#
###B#.#.#.###
  #D#B#C#D#
  #########
";

    static CASE_2: &str = "
#############
#.....B.A.C.#
###.#.#.#.###
  #D#B#C#D#
  #########
";

    static CASE_3: &str = "
#############
#.......A.C.#
###.#B#.#.###
  #D#B#C#D#
  #########
";

    static CASE_4: &str = "
#############
#.....B.A...#
###.#B#C#.###
  #D#B#C#D#
  #########
";

    static CASE_5: &str = "
#############
#.......A.C.#
###.#B#.#.###
  #D#B#C#D#
  #########
";

    static CASE_6: &str = "
#############
#.....A.A.C.#
###.#B#.#.###
  #D#B#C#D#
  #########
";

    static CASE_7: &str = "
#############
#...B.A.A.C.#
###.#.#.#.###
  #D#B#C#D#
  #########
";

    fn read_problem_case(sample: &str) -> State {
        let lines: Vec<String> = sample
            .split_terminator('\n')
            .filter(|s| !s.trim().is_empty())
            .map(String::from)
            .collect();
        read_problem(lines)
    }

    #[test]
    fn test_parse() {
        let state = &read_problem_case(CASE_1);
        assert_eq!(
            state.hallway,
            [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, 'A', EMPTY, 'C', EMPTY]
        );
        assert_eq!(
            state.rooms,
            [
                vec!['D', 'B'],
                vec!['B', EMPTY],
                vec!['C', EMPTY],
                vec!['D', EMPTY]
            ]
        );
    }

    #[test]
    fn test_room_to_hallway() {
        let derived = derive_states(&read_problem_case(CASE_1));
        assert!(derived
            .iter()
            .find(|c| read_problem_case(CASE_2) == c.0)
            .is_some());
        assert_eq!(
            derived
                .iter()
                .find(|c| read_problem_case(CASE_2) == c.0)
                .unwrap()
                .1,
            4 * cost('B')
        );
    }

    #[test]
    fn test_hallway_to_room() {
        let derived = derive_states(&read_problem_case(CASE_2));
        assert!(derived
            .iter()
            .find(|c| read_problem_case(CASE_3) == c.0)
            .is_some());
        assert_eq!(
            derived
                .iter()
                .find(|c| read_problem_case(CASE_3) == c.0)
                .unwrap()
                .1,
            2 * cost('B')
        );
        assert!(derived
            .iter()
            .find(|c| read_problem_case(CASE_4) == c.0)
            .is_none());
    }

    #[test]
    fn test_room_to_room() {
        let derived = derive_states(&read_problem_case(CASE_1));
        assert!(derived
            .iter()
            .find(|c| read_problem_case(CASE_5) == c.0)
            .is_some());
        assert_eq!(
            derived
                .iter()
                .find(|c| read_problem_case(CASE_5) == c.0)
                .unwrap()
                .1,
            4 * cost('B')
        );
    }

    #[test]
    fn test_stay_final_room() {
        let derived = derive_states(&read_problem_case(CASE_6));
        assert!(derived
            .iter()
            .find(|c| read_problem_case(CASE_7) == c.0)
            .is_none());
    }
}
