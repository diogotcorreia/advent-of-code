use pathfinding::prelude::astar;

use crate::AocDay;

const NORTH_MASK: u8 = 0x1;
const EAST_MASK: u8 = 0x2;
const SOUTH_MASK: u8 = 0x4;
const WEST_MASK: u8 = 0x8;
const WALL_MASK: u8 = 0x10;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos(i32, i32); // y, x

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TimePos(usize, Pos);

fn calculate_next_blizzards(blizzards: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (map_height, map_width) = (
        blizzards.len() as i32 - 2,
        blizzards.first().unwrap().len() as i32 - 2,
    );
    let get_position_vec = |pos: (usize, usize), vec: (i32, i32)| -> (usize, usize) {
        if pos.0 == 0 || pos.0 == (map_height as usize + 1) {
            return pos;
        }
        let (y, x) = (pos.0 as i32 + vec.0, pos.1 as i32 + vec.1);
        let (y, x) = (
            (y - 1).rem_euclid(map_height),
            (x - 1).rem_euclid(map_width),
        );

        (y as usize + 1, x as usize + 1)
    };

    blizzards
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    if cell & WALL_MASK != 0 {
                        *cell
                    } else {
                        let north = get_position_vec((y, x), (-1, 0));
                        let east = get_position_vec((y, x), (0, 1));
                        let south = get_position_vec((y, x), (1, 0));
                        let west = get_position_vec((y, x), (0, -1));

                        (*blizzards.get(north.0).unwrap().get(north.1).unwrap() & SOUTH_MASK)
                            + (*blizzards.get(east.0).unwrap().get(east.1).unwrap() & WEST_MASK)
                            + (*blizzards.get(south.0).unwrap().get(south.1).unwrap() & NORTH_MASK)
                            + (*blizzards.get(west.0).unwrap().get(west.1).unwrap() & EAST_MASK)
                    }
                })
                .collect()
        })
        .collect()
}

fn get_possible_next_positions(map: &mut Vec<Vec<Vec<u8>>>, pos: &TimePos) -> Vec<(TimePos, i32)> {
    let (time, pos) = (pos.0, &pos.1);

    if time + 1 >= map.len() {
        map.push(calculate_next_blizzards(map.last().unwrap()));
    }

    let next_blizzard = map.get(time + 1).unwrap();
    [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|vec| (pos.0 + vec.0, pos.1 + vec.1))
        .filter(|p| {
            *next_blizzard
                .get(p.0 as usize)
                .and_then(|row| row.get(p.1 as usize))
                .unwrap_or(&WALL_MASK)
                == 0
        })
        .map(|p| (TimePos(time + 1, Pos(p.0, p.1)), 1))
        .collect()
}

fn heuristic_fun(pos: &Pos, target_pos: &Pos) -> i32 {
    // Manhattan distance
    (pos.0 - target_pos.0).abs() + (pos.1 - target_pos.1).abs()
}

#[allow(unused)]
fn print_blizzards(blizzards: &Vec<Vec<u8>>) {
    for row in blizzards {
        for cell in row {
            let mut chars = Vec::new();
            if cell & NORTH_MASK != 0 {
                chars.push('^');
            }
            if cell & EAST_MASK != 0 {
                chars.push('>');
            }
            if cell & SOUTH_MASK != 0 {
                chars.push('v');
            }
            if cell & WEST_MASK != 0 {
                chars.push('<');
            }
            if cell & WALL_MASK != 0 {
                chars.push('#');
            }
            match chars.len() {
                0 => print!("."),
                1 => print!("{}", chars.first().unwrap()),
                _ => print!("{}", chars.len()),
            }
        }
        println!();
    }
}

#[derive(Debug)]
pub struct AocDay24 {
    blizzards: Vec<Vec<u8>>, // use a bitmask; indexed [y][x]
    start_pos: Pos,
    target_pos: Pos,
}

impl AocDay<i32, i32> for AocDay24 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let blizzards = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '^' => NORTH_MASK,
                        '>' => EAST_MASK,
                        'v' => SOUTH_MASK,
                        '<' => WEST_MASK,
                        '#' => WALL_MASK,
                        _ => 0,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let start_pos = Pos(
            0,
            blizzards
                .first()
                .and_then(|row| row.iter().position(|cell| *cell == 0))
                .map(|x| x as i32)
                .expect("can't find start position"),
        );

        let target_pos = Pos(
            blizzards.len() as i32 - 1,
            blizzards
                .last()
                .and_then(|row| row.iter().position(|cell| *cell == 0))
                .map(|x| x as i32)
                .expect("can't find start position"),
        );

        AocDay24 {
            blizzards,
            start_pos,
            target_pos,
        }
    }
    fn part1(&self) -> i32 {
        let mut blizzards_history = vec![self.blizzards.clone()];

        let (_path, steps) = astar(
            &TimePos(0, self.start_pos.clone()),
            |pos| get_possible_next_positions(&mut blizzards_history, pos),
            |pos| heuristic_fun(&pos.1, &self.target_pos),
            |pos| pos.1 == self.target_pos,
        )
        .expect("no goal reached");

        steps
    }
    fn part2(&self) -> i32 {
        let mut blizzards_history = vec![self.blizzards.clone()];

        let (path, steps_first) = astar(
            &TimePos(0, self.start_pos.clone()),
            |pos| get_possible_next_positions(&mut blizzards_history, pos),
            |pos| heuristic_fun(&pos.1, &self.target_pos),
            |pos| pos.1 == self.target_pos,
        )
        .expect("no goal reached");

        let (path, steps_second) = astar(
            path.last().unwrap(),
            |pos| get_possible_next_positions(&mut blizzards_history, pos),
            |pos| heuristic_fun(&pos.1, &self.start_pos),
            |pos| pos.1 == self.start_pos,
        )
        .expect("no goal reached");

        let (_path, steps_third) = astar(
            path.last().unwrap(),
            |pos| get_possible_next_positions(&mut blizzards_history, pos),
            |pos| heuristic_fun(&pos.1, &self.target_pos),
            |pos| pos.1 == self.target_pos,
        )
        .expect("no goal reached");

        steps_first + steps_second + steps_third
    }
}

#[cfg(test)]
mod day24tests {
    use super::*;

    const INPUT: &[&str] = &[
        "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
    ];

    #[test]
    fn part1() {
        let day = AocDay24::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 18);
    }

    #[test]
    fn part2() {
        let day = AocDay24::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 54);
    }
}
