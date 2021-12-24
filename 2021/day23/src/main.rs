use core::panic;
use priority_queue::PriorityQueue;
use regex::Regex;
use std::{cmp::Reverse, collections::HashMap, hash::Hash, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Burrow {
    rooms: [[i8; 4]; 4], // four rooms of two; index 0 is top, 1 is bottom
    hallway: [i8; 11],
    room_size: usize,
}

fn main() {
    let re = Regex::new(r"#(\w)#(\w)#(\w)#(\w)#").unwrap();
    let burrow = {
        let input: Vec<[String; 4]> = std::io::stdin()
            .lock()
            .lines()
            .skip(2)
            .take(2)
            .map(|v| {
                let v = v.unwrap();
                let cap = re.captures(&v).unwrap();
                [
                    String::from(&cap[1]),
                    String::from(&cap[2]),
                    String::from(&cap[3]),
                    String::from(&cap[4]),
                ]
            })
            .collect();

        let mut burrow = Burrow {
            rooms: [[-1; 4]; 4],
            hallway: [-1; 11],
            room_size: 2,
        };

        for (i, v) in input.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                burrow.rooms[j][i] = match c.as_str() {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    "D" => 3,
                    _ => unreachable!("invalid amphipod"),
                }
            }
        }

        burrow
    };

    println!("Part 1: {}", a_star(&burrow));

    let burrow_extended = Burrow {
        hallway: [-1; 11],
        rooms: [
            [burrow.rooms[0][0], 3, 3, burrow.rooms[0][1]],
            [burrow.rooms[1][0], 2, 1, burrow.rooms[1][1]],
            [burrow.rooms[2][0], 1, 0, burrow.rooms[2][1]],
            [burrow.rooms[3][0], 0, 2, burrow.rooms[3][1]],
        ],
        room_size: 4,
    };

    println!("Part 2: {}", a_star(&burrow_extended));
}

fn get_possible_next_states(burrow: &Burrow) -> Vec<(Burrow, i32)> {
    let mut next_states = Vec::new();

    for hallway_state in burrow.hallway.iter().enumerate() {
        if *hallway_state.1 < 0 {
            continue;
        }

        if let Some(insert_pos) = get_insert_pos_in_room(*hallway_state.1 as usize, burrow) {
            if is_path_free(burrow, hallway_state.0, *hallway_state.1 as usize, true) {
                next_states.push(swap_positions(
                    burrow,
                    hallway_state.0,
                    *hallway_state.1 as usize,
                    insert_pos,
                ));
            }
        }
    }

    for hallway_pos in [0, 1, 3, 5, 7, 9, 10] {
        for room_n in 0..4 {
            if is_path_free(burrow, hallway_pos, room_n, false) {
                if let Some(popout_pos) = get_popout_pos_in_room(room_n, burrow) {
                    next_states.push(swap_positions(burrow, hallway_pos, room_n, popout_pos));
                }
            }
        }
    }

    next_states
}

fn get_insert_pos_in_room(room_n: usize, burrow: &Burrow) -> Option<usize> {
    // an amphirod can only move from the hallway to its destination room
    // and only if it doesn't contain amphirod of different types
    let mut last_unoccupied_pos = None;
    for i in 0..burrow.room_size {
        if burrow.rooms[room_n][i] == -1 as i8 {
            last_unoccupied_pos = Some(i);
        } else if burrow.rooms[room_n][i] != room_n as i8 {
            return None;
        }
    }
    last_unoccupied_pos
}

fn get_popout_pos_in_room(room_n: usize, burrow: &Burrow) -> Option<usize> {
    // only force the amphirod out of the room if it's not in the correct
    // room or if there is an amphirod below it that is not in the correct room
    let mut first_occupied_pos = None;
    for i in 0..burrow.room_size {
        if burrow.rooms[room_n][i] != -1 {
            if first_occupied_pos.is_none() {
                first_occupied_pos = Some(i);
            }
            if burrow.rooms[room_n][i] != room_n as i8 {
                return first_occupied_pos;
            }
        }
    }
    None
}

fn swap_positions(
    burrow: &Burrow,
    hallway_pos: usize,
    room_n: usize,
    room_pos: usize,
) -> (Burrow, i32) {
    // position of room entrance on hallway
    let pos = 2 + 2 * room_n as i32;
    let cost = (pos - hallway_pos as i32).abs() + 1 + room_pos as i32;

    let mut new_state = burrow.clone();

    new_state.hallway[hallway_pos] = burrow.rooms[room_n][room_pos];
    new_state.rooms[room_n][room_pos] = burrow.hallway[hallway_pos];

    (
        new_state,
        cost * 10_i32.pow(
            burrow.rooms[room_n][room_pos]
                .max(burrow.hallway[hallway_pos])
                .try_into()
                .unwrap(),
        ),
    )
}

fn is_path_free(burrow: &Burrow, hallway_pos: usize, room_n: usize, ignore_self: bool) -> bool {
    // position of room entrance on hallway
    let pos = 2 + 2 * room_n;
    let range = if pos < hallway_pos {
        pos..=hallway_pos
    } else {
        hallway_pos..=pos
    };

    for i in range {
        if (!ignore_self || i != hallway_pos) && burrow.hallway[i] != -1 {
            return false;
        }
    }

    true
}

fn has_ended(burrow: &Burrow) -> bool {
    for room_n in 0..4 {
        for room_pos in 0..burrow.room_size {
            if burrow.rooms[room_n][room_pos] != room_n as i8 {
                return false;
            }
        }
    }
    true
}

fn heuristic_fun(burrow: &Burrow) -> i32 {
    // just calculate as if they "teleported" to their correct positions
    let mut total_cost: i32 = 0;
    for (hallway_pos, &room_n) in burrow.hallway.iter().enumerate() {
        if room_n != -1 {
            let pos = 2 + 2 * room_n;
            total_cost += ((pos as i32 - hallway_pos as i32).abs()).pow(room_n.try_into().unwrap());
        }
    }

    for room_n in 0..4 {
        for room_pos in 0..burrow.room_size {
            let num = burrow.rooms[room_n][room_pos];
            if num != -1 && num != room_n as i8 {
                total_cost += ((room_n as i32 - num as i32) * 2 + room_pos as i32)
                    .pow(num.try_into().unwrap()) as i32;
            }
        }
    }

    total_cost
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
fn a_star(start: &Burrow) -> i32 {
    let mut open_set = PriorityQueue::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let mut came_from = HashMap::new();

    let start_f_score = heuristic_fun(start);
    open_set.push(*start, Reverse(start_f_score));
    g_score.insert(*start, 0);
    f_score.insert(*start, start_f_score);

    while !open_set.is_empty() {
        let (current, current_f_score) = open_set.pop().unwrap();

        if has_ended(&current) {
            return current_f_score.0;
        }

        for (neighbor, neighbor_cost) in get_possible_next_states(&current) {
            let tentative_g_score = g_score.get(&current).unwrap() + neighbor_cost;
            if !g_score.contains_key(&neighbor)
                || tentative_g_score < *g_score.get(&neighbor).unwrap()
            {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                let neighbor_f_score = tentative_g_score + heuristic_fun(&neighbor);
                f_score.insert(neighbor.clone(), neighbor_f_score);

                open_set.push_increase(neighbor, Reverse(neighbor_f_score));
            }
        }
    }

    panic!("goal never reached")
}
