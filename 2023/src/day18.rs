use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::AocDay;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: u32,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl FromStr for Direction {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" | "0" => Ok(Direction::East),
            "D" | "1" => Ok(Direction::South),
            "L" | "2" => Ok(Direction::West),
            "U" | "3" => Ok(Direction::North),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct PosDir {
    pos: Pos,
    direction: Direction,
}

impl PosDir {
    fn move_pos(&self, amount: i64) -> Option<PosDir> {
        let (x, y) = (self.pos.x, self.pos.y);
        match self.direction {
            Direction::North => y.checked_sub(amount).map(|y| (x, y)),
            Direction::East => x.checked_add(amount).map(|x| (x, y)),
            Direction::South => y.checked_add(amount).map(|y| (x, y)),
            Direction::West => x.checked_sub(amount).map(|x| (x, y)),
        }
        .map(|(x, y)| PosDir {
            pos: Pos { x, y },
            direction: self.direction.clone(),
        })
    }
}

#[derive(Debug, Clone)]
struct Range {
    min: i64, // inclusive
    max: i64, // inclusive
    direction: Direction,
}

type Map1D = HashMap<i64, Vec<Range>>;

fn get_direction_of_pos(pos: &Pos, rows: &Map1D, cols: &Map1D) -> Option<Direction> {
    rows.get(&pos.y)
        .and_then(|ranges| {
            ranges
                .iter()
                .find(|range| range.min <= pos.x && range.max >= pos.x)
                .map(|range| range.direction.clone())
        })
        .or_else(|| {
            cols.get(&pos.x).and_then(|ranges| {
                ranges
                    .iter()
                    .find(|range| range.min <= pos.y && range.max >= pos.y)
                    .map(|range| range.direction.clone())
            })
        })
}

fn get_interesting_ranges(y: i64, rows: &Map1D, cols: &Map1D) -> Vec<Range> {
    rows.get(&y)
        .unwrap_or(&vec![])
        .iter()
        .cloned()
        .chain(cols.iter().filter_map(|(x_range, ranges)| {
            let range = ranges.iter().find(|r| r.min <= y && r.max >= y);
            range.map(|range| Range {
                min: *x_range,
                max: *x_range,
                direction: range.direction.clone(),
            })
        }))
        .sorted_by_key(|range| range.min)
        .collect()
}

fn solve<'a>(instructions: impl Iterator<Item = &'a Instruction>) -> u64 {
    let mut rows: HashMap<i64, Vec<Range>> = HashMap::new();
    let mut cols: HashMap<i64, Vec<Range>> = HashMap::new();

    let mut curr_pos = PosDir {
        pos: Pos { x: 0, y: 0 },
        direction: Direction::North,
    };

    // build map
    for inst in instructions {
        curr_pos.direction = inst.direction.clone();
        curr_pos = curr_pos.move_pos(1).unwrap();
        let next_pos = curr_pos.move_pos(i64::from(inst.count) - 1).unwrap();
        match curr_pos.direction {
            Direction::North | Direction::South => {
                cols.entry(curr_pos.pos.x).or_default().push(Range {
                    min: curr_pos.pos.y.min(next_pos.pos.y),
                    max: curr_pos.pos.y.max(next_pos.pos.y),
                    direction: curr_pos.direction.clone(),
                });
            }
            Direction::East | Direction::West => {
                rows.entry(curr_pos.pos.y).or_default().push(Range {
                    min: curr_pos.pos.x.min(next_pos.pos.x),
                    max: curr_pos.pos.x.max(next_pos.pos.x),
                    direction: curr_pos.direction.clone(),
                });
            }
        }
        curr_pos = next_pos;
    }

    // count inside of map
    let mut count: u64 = 0;
    let interesting_rows: Vec<i64> = rows
        .keys()
        .cloned()
        .flat_map(|y| vec![y, y + 1])
        .sorted()
        .collect();
    let mut last_y = i64::MIN;
    let mut last_count_line: u64 = 0;
    for y in interesting_rows {
        count += y.abs_diff(last_y) * last_count_line;
        last_y = y;
        last_count_line = 0;

        let mut inside_depth = 0;
        let mut inside = None;
        for x_range in get_interesting_ranges(y, &rows, &cols) {
            let dir = x_range.direction;
            match dir {
                Direction::North => {
                    if inside.is_none() {
                        last_count_line += x_range.max.abs_diff(x_range.min) + 1;
                    }
                    inside_depth += 1;
                }
                Direction::South | Direction::East | Direction::West => {
                    if inside.is_none() {
                        last_count_line += x_range.max.abs_diff(x_range.min) + 1;
                    }
                    if let Some(dir2) = get_direction_of_pos(
                        &Pos {
                            x: if dir == Direction::East {
                                x_range.max
                            } else {
                                x_range.min
                            },
                            y: y + 1,
                        },
                        &rows,
                        &cols,
                    ) {
                        if dir2 == Direction::South {
                            inside_depth -= 1;
                        }
                    }
                }
            }
            if inside_depth == 0 {
                if let Some(border_x) = inside {
                    last_count_line += x_range.max.abs_diff(border_x);
                    inside = None;
                }
            } else if inside.is_none() {
                inside = Some(x_range.max);
            }
        }
        assert!(inside.is_none(), "finished line inside the trench");
    }
    assert_eq!(last_count_line, 0, "something was not counted properly");

    count
}

pub struct AocDay18 {
    instructions: Vec<(Instruction, Instruction)>,
}

impl AocDay<u64, u64> for AocDay18 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let instructions = lines
            .map(|line| {
                let mut it = line.split_whitespace();
                let direction = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .expect("invalid direction");
                let count: u32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .expect("invalid step count");

                let color = it
                    .next()
                    .and_then(|s| s.strip_prefix("(#"))
                    .and_then(|s| s.strip_suffix(')'))
                    .expect("invalid color code");

                let dist = u32::from_str_radix(&color[0..5], 16).expect("invalid step count");
                let direction2 = color[5..6].parse().expect("invalid direction");

                (
                    Instruction { direction, count },
                    Instruction {
                        direction: direction2,
                        count: dist,
                    },
                )
            })
            .collect();

        AocDay18 { instructions }
    }
    fn part1(&self) -> u64 {
        solve(self.instructions.iter().map(|x| &x.0))
    }
    fn part2(&self) -> u64 {
        solve(self.instructions.iter().map(|x| &x.1))
    }
}

#[cfg(test)]
mod day18tests {
    use super::*;

    const INPUT: &[&str] = &[
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    #[test]
    fn part1() {
        let day = AocDay18::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 62);
    }

    #[test]
    fn part2() {
        let day = AocDay18::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 952408144115);
    }
}
