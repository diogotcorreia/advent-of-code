use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use crate::AocDay;

type Pos = (i16, i16);

#[derive(Debug)]
struct Movement(Direction, u8);

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, step) = s.split_once(' ').expect("malformatted input");

        Ok(Movement(
            direction.parse().expect("invalid direction"),
            step.parse()?,
        ))
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err("Unknown direction".to_string()),
        }
    }
}

impl Direction {
    fn move_pos(&self, pos: &Pos) -> Pos {
        match self {
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Up => (pos.0, pos.1 + 1),
            Self::Down => (pos.0, pos.1 - 1),
        }
    }
}

//fn print_map(head: &Pos, tail: &[Pos]) {
//    for j in (-15..=15).rev() {
//        for i in -15..=15 {
//            if (i, j) == *head {
//                print!("H");
//            } else if let Some((x, _)) = tail.iter().enumerate().find(|(_, x)| (i, j) == **x) {
//                print!("{}", x + 1);
//            } else if (i, j) == (0, 0) {
//                print!("s");
//            } else {
//                print!(".");
//            }
//        }
//        println!();
//    }
//    println!();
//}

fn pull_tail(head: &Pos, tail: &Pos /* , last_movement: &Direction*/) -> Pos {
    macro_rules! adj {
        ($pos: expr, $vec: expr) => {
            ($pos.0 + $vec.0, $pos.1 + $vec.1)
        };
    }
    macro_rules! dist_sq {
        ($pos: expr, $pos2: expr) => {
            ($pos.0 - $pos2.0).pow(2) + ($pos.1 - $pos2.1).pow(2)
        };
    }

    // Is adjacent
    let distance_sq = dist_sq!(head, tail);
    if distance_sq <= 2 {
        return *tail;
    }

    macro_rules! closer_adj {
        ($src: expr, $dest_base: expr, $($adj: expr),*) => {
            {
                let mut closer = None;
                let mut closer_distance = None;
                for pos in [$($adj,)*] {
                    let pos = adj!($dest_base, pos);
                    let distance = dist_sq!(pos, $src);
                    match closer_distance {
                        Some(x) if x < distance => {}
                        _ => {
                            closer_distance = Some(distance);
                            closer = Some(pos);
                        }
                    }
                }

                closer.expect("no closer pos")
            }
        };
    }

    // If in the same axis, move along axis
    if head.0 == tail.0 || head.1 == tail.1 {
        closer_adj!(tail, head, (0, 1), (0, -1), (1, 0), (-1, 0))
    } else {
        closer_adj!(head, tail, (1, 1), (-1, -1), (1, -1), (-1, 1))
    }
}

fn simulate_bridge<const N: usize>(movements: &[Movement]) -> usize {
    let mut head: Pos = (0, 0);
    let mut tails: [Pos; N] = [(0, 0); N];

    let mut visited = HashSet::new();
    visited.insert(tails[N - 1]);

    for movement in movements {
        for _ in 0..movement.1 {
            head = movement.0.move_pos(&head);

            for i in 0..N {
                match i {
                    0 => tails[i] = pull_tail(&head, &tails[i]),
                    _ => tails[i] = pull_tail(&tails[i - 1], &tails[i]),
                }
            }
            visited.insert(tails[N - 1]);
        }
    }

    visited.len()
}

pub struct AocDay09 {
    movements: Vec<Movement>,
}

impl AocDay<usize, usize> for AocDay09 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let movements = lines.map(|x| x.parse().expect("invalid input")).collect();

        AocDay09 { movements }
    }
    fn part1(&self) -> usize {
        simulate_bridge::<1>(&self.movements)
    }
    fn part2(&self) -> usize {
        simulate_bridge::<9>(&self.movements)
    }
}

#[cfg(test)]
mod day09tests {
    use super::*;

    const INPUT: &[&str] = &["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const INPUT2: &[&str] = &["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    #[test]
    fn part1() {
        let day = AocDay09::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part2_small() {
        let day = AocDay09::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 1);
    }

    #[test]
    fn part2_big() {
        let day = AocDay09::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 36);
    }
}
