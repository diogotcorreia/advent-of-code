use ndarray::Array2;
use pathfinding::prelude::astar;

use crate::AocDay;

fn get_possible_next_positions<const MAX_STRAIGHT: usize, const MIN_STRAIGHT: usize>(
    pos: &Pos,
    map: &Array2<u32>,
) -> Vec<(Pos, u32)> {
    Direction::get_all()
        .into_iter()
        .filter(|d| *d != pos.direction.get_opposite())
        .filter(|d| pos.straight_line_count < MAX_STRAIGHT || pos.direction != *d)
        // if straight_line_count == 0, we're in the start, so allow any direction
        .filter(|d| {
            pos.straight_line_count == 0
                || pos.straight_line_count >= MIN_STRAIGHT
                || pos.direction == *d
        })
        .filter_map(|d| {
            if pos.direction == d {
                pos.move_pos(map.shape())
            } else {
                let new_pos = Pos {
                    x: pos.x,
                    y: pos.y,
                    straight_line_count: 0,
                    direction: d,
                };
                new_pos.move_pos(map.shape())
            }
        })
        .map(|p| {
            let cost = map[(p.y, p.x)];
            (p, cost)
        })
        .collect()
}

fn heuristic_fun(pos: &Pos, map: &Array2<u32>) -> u32 {
    (pos.x.abs_diff(map.ncols() - 1) + pos.y.abs_diff(map.nrows() - 1)) as u32
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
    fn get_opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
    straight_line_count: usize,
    direction: Direction,
}

impl Pos {
    fn move_pos(&self, limits: &[usize]) -> Option<Pos> {
        let (x, y) = (self.x, self.y);
        match self.direction {
            Direction::North => y.checked_sub(1).map(|y| (x, y)),
            Direction::East => x.checked_add(1).map(|x| (x, y)),
            Direction::South => y.checked_add(1).map(|y| (x, y)),
            Direction::West => x.checked_sub(1).map(|x| (x, y)),
        }
        .filter(|(x, y)| *x < limits[1] && *y < limits[0])
        .map(|(x, y)| Pos {
            x,
            y,
            straight_line_count: self.straight_line_count + 1,
            direction: self.direction.clone(),
        })
    }
}

pub struct AocDay17 {
    map: Array2<u32>,
}

impl AocDay17 {
    fn solve<const MAX_STRAIGHT: usize, const MIN_STRAIGHT: usize>(&self) -> u32 {
        let start = Pos {
            x: 0,
            y: 0,
            straight_line_count: 0,
            direction: Direction::East,
        };
        let (path, _steps) = astar(
            &start,
            |pos| get_possible_next_positions::<MAX_STRAIGHT, MIN_STRAIGHT>(pos, &self.map),
            |pos| heuristic_fun(pos, &self.map),
            |pos| {
                pos.x == self.map.ncols() - 1
                    && pos.y == self.map.nrows() - 1
                    && pos.straight_line_count >= MIN_STRAIGHT
            },
        )
        .expect("no goal reached");

        path.into_iter()
            .skip(1)
            .map(|pos| self.map[(pos.y, pos.x)])
            .sum()
    }
}

impl AocDay<u32, u32> for AocDay17 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.peekable();
        let width = lines.peek().expect("map does not have any rows").len();
        let map: Vec<u32> = lines
            .flat_map(|row| {
                row.chars()
                    .map(|c| c.to_digit(10).expect("failed to parse character in map"))
                    .collect::<Vec<_>>()
            })
            .collect();

        AocDay17 {
            map: Array2::from_shape_vec((map.len() / width, width), map)
                .expect("failed to build array2"),
        }
    }
    fn part1(&self) -> u32 {
        self.solve::<3, 0>()
    }
    fn part2(&self) -> u32 {
        self.solve::<10, 4>()
    }
}

#[cfg(test)]
mod day17tests {
    use super::*;

    const INPUT: &[&str] = &[
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];
    const INPUT2: &[&str] = &[
        "111111111111",
        "999999999991",
        "999999999991",
        "999999999991",
        "999999999991",
    ];

    #[test]
    fn part1() {
        let day = AocDay17::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 102);
    }

    #[test]
    fn part2() {
        let day = AocDay17::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 94);
        let day = AocDay17::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 71);
    }
}
