use std::collections::{HashMap, HashSet};

use crate::AocDay;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn get_clearance_pos(&self, direction: &Direction) -> [Pos; 3] {
        // first position is the position to move it
        match *direction {
            Direction::North => [
                Pos(self.0, self.1 - 1),
                Pos(self.0 - 1, self.1 - 1),
                Pos(self.0 + 1, self.1 - 1),
            ],
            Direction::East => [
                Pos(self.0 + 1, self.1),
                Pos(self.0 + 1, self.1 - 1),
                Pos(self.0 + 1, self.1 + 1),
            ],
            Direction::South => [
                Pos(self.0, self.1 + 1),
                Pos(self.0 - 1, self.1 + 1),
                Pos(self.0 + 1, self.1 + 1),
            ],
            Direction::West => [
                Pos(self.0 - 1, self.1),
                Pos(self.0 - 1, self.1 - 1),
                Pos(self.0 - 1, self.1 + 1),
            ],
        }
    }

    fn get_adjacent_positions(&self) -> [Pos; 8] {
        [
            Pos(self.0 - 1, self.1 - 1),
            Pos(self.0 - 1, self.1),
            Pos(self.0 - 1, self.1 + 1),
            Pos(self.0 + 1, self.1 - 1),
            Pos(self.0 + 1, self.1),
            Pos(self.0 + 1, self.1 + 1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
    }
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_next(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::West,
            Self::West => Self::East,
            Self::East => Self::North,
        }
    }
}

fn exec_round(elves_positions: &HashSet<Pos>, move_direction: &Direction) -> (HashSet<Pos>, bool) {
    // hashmap<destination_pos, vec<origin_pos>>
    let mut new_positions: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut moved = false;

    // check where to move to
    'outer: for elf_pos in elves_positions {
        if elf_pos
            .get_adjacent_positions()
            .iter()
            .any(|p| elves_positions.contains(p))
        {
            // we gotta move
            let mut direction = move_direction.clone();
            for _ in 0..4 {
                let clearance = elf_pos.get_clearance_pos(&direction);
                if clearance.iter().all(|p| !elves_positions.contains(p)) {
                    // empty!
                    new_positions
                        .entry(clearance[0].clone())
                        .or_insert(Vec::new())
                        .push(elf_pos.clone());
                    moved = true;
                    continue 'outer;
                }
                direction = direction.get_next();
            }
        }

        new_positions
            .entry(elf_pos.clone())
            .or_insert(Vec::new())
            .push(elf_pos.clone());
    }

    // now move
    let mut new_elves_positions = HashSet::new();
    for (destination, origins) in new_positions {
        if origins.len() == 1 {
            new_elves_positions.insert(destination);
        } else {
            origins.into_iter().for_each(|p| {
                new_elves_positions.insert(p);
            });
        }
    }

    (new_elves_positions, moved)
}

fn get_map_bounds(elves_positions: &HashSet<Pos>) -> (Pos, Pos) {
    let mut nw_bound = Pos(i32::MAX, i32::MAX);
    let mut se_bound = Pos(i32::MIN, i32::MIN);

    for pos in elves_positions {
        if pos.0 < nw_bound.0 {
            nw_bound = Pos(pos.0, nw_bound.1)
        }
        if pos.1 < nw_bound.1 {
            nw_bound = Pos(nw_bound.0, pos.1)
        }
        if pos.0 > se_bound.0 {
            se_bound = Pos(pos.0, se_bound.1)
        }
        if pos.1 > se_bound.1 {
            se_bound = Pos(se_bound.0, pos.1)
        }
    }

    (nw_bound, se_bound)
}

fn count_empty(elves_positions: &HashSet<Pos>) -> i32 {
    let (nw_bound, se_bound) = get_map_bounds(elves_positions);

    let size = (se_bound.0 - nw_bound.0 + 1) * (se_bound.1 - nw_bound.1 + 1);

    size - elves_positions.len() as i32
}

#[allow(dead_code)]
fn dbg_map(elves_positions: &HashSet<Pos>) {
    let (nw_bound, se_bound) = get_map_bounds(elves_positions);

    for y in nw_bound.1..=se_bound.1 {
        for x in nw_bound.0..=se_bound.0 {
            if elves_positions.contains(&Pos(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub struct AocDay23 {
    elves_positions: HashSet<Pos>,
}

impl AocDay<i32, i32> for AocDay23 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let elves_positions = lines
            .enumerate()
            .fold(HashSet::new(), |mut acc, (y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, char)| *char == '#')
                    .for_each(|(x, _)| {
                        acc.insert(Pos(x as i32, y as i32));
                    });

                acc
            });

        AocDay23 { elves_positions }
    }
    fn part1(&self) -> i32 {
        let mut elves_positions = self.elves_positions.clone();
        let mut move_direction = Direction::North;

        for _ in 0..10 {
            elves_positions = exec_round(&elves_positions, &move_direction).0;
            move_direction = move_direction.get_next();
        }

        count_empty(&elves_positions)
    }
    fn part2(&self) -> i32 {
        let mut elves_positions = self.elves_positions.clone();
        let mut moved = true;
        let mut move_direction = Direction::North;
        let mut round = 0;

        while moved {
            (elves_positions, moved) = exec_round(&elves_positions, &move_direction);
            move_direction = move_direction.get_next();
            round += 1;
        }

        round
    }
}

#[cfg(test)]
mod day23tests {
    use super::*;

    const INPUT: &[&str] = &[
        "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
    ];

    #[test]
    fn part1() {
        let day = AocDay23::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 110);
    }

    #[test]
    fn part2() {
        let day = AocDay23::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 20);
    }
}
