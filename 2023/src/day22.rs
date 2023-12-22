use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use ndarray::Array3;

use crate::AocDay;

#[derive(Debug, Clone)]
struct Brick {
    start: Pos3D,
    end: Pos3D,
}

impl FromStr for Brick {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();

        Ok(Brick {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

impl Brick {
    fn get_pos(&self) -> impl Iterator<Item = Pos3D> + '_ {
        (self.start.x..=self.end.x).flat_map(move |x| {
            (self.start.y..=self.end.y)
                .flat_map(move |y| (self.start.z..=self.end.z).map(move |z| Pos3D { x, y, z }))
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos3D {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Pos3D {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s
            .split(',')
            .take(3)
            .map(|num| num.parse().map_err(|_| ParseErr));
        Ok(Pos3D {
            x: coords.next().ok_or(ParseErr)??,
            y: coords.next().ok_or(ParseErr)??,
            z: coords.next().ok_or(ParseErr)??,
        })
    }
}

#[derive(Debug)]
struct ParseErr;

fn simulate_fall(
    sorted_bricks: &[Brick],
    dimensions: (usize, usize, usize),
    skip_brick: Option<usize>,
) -> (Vec<Brick>, Array3<Option<usize>>, usize) {
    let mut new_bricks = Vec::with_capacity(sorted_bricks.len());
    let mut brick_map: Array3<Option<usize>> = Array3::default(dimensions);
    let mut changed_count = 0;

    for (i, brick) in sorted_bricks.iter().enumerate() {
        if skip_brick == Some(i) {
            continue;
        }
        let mut offset: usize = 1;

        'outer: while brick.start.z.checked_sub(offset).is_some() {
            for pos in brick.get_pos() {
                if brick_map[(pos.x, pos.y, pos.z - offset)].is_some() {
                    break 'outer;
                }
            }
            offset += 1;
        }

        offset -= 1;
        if offset != 0 {
            changed_count += 1;
        }

        let new_brick = Brick {
            start: Pos3D {
                x: brick.start.x,
                y: brick.start.y,
                z: brick.start.z - offset,
            },
            end: Pos3D {
                x: brick.end.x,
                y: brick.end.y,
                z: brick.end.z - offset,
            },
        };

        new_brick.get_pos().for_each(|pos| {
            brick_map[(pos.x, pos.y, pos.z)] = Some(new_bricks.len());
        });
        new_bricks.push(new_brick);
    }

    (new_bricks, brick_map, changed_count)
}

/// Get mapping of bricks that are adjacent to the brick
/// when changing z as per the given function.
fn get_adjacent_bricks_map(
    sorted_bricks: &[Brick],
    brick_map: &Array3<Option<usize>>,
    z_change_fn: impl Fn(usize) -> Option<usize>,
) -> HashMap<usize, Vec<usize>> {
    sorted_bricks
        .iter()
        .enumerate()
        .map(|(i, brick)| {
            let brick_pos = brick.get_pos();

            let adjacent = brick_pos
                .into_iter()
                .filter_map(|pos| {
                    z_change_fn(pos.z).map(|z| Pos3D {
                        x: pos.x,
                        y: pos.y,
                        z,
                    })
                })
                .filter_map(|pos| brick_map[(pos.x, pos.y, pos.z)]) // make sure resulting position is a brick
                .filter(|j| i != *j) // don't add itself
                .sorted()
                .dedup()
                .collect();

            (i, adjacent)
        })
        .collect()
}

pub struct AocDay22 {
    bricks: Vec<Brick>,
    safe_to_disintegrate: Vec<bool>,
    dimensions: (usize, usize, usize),
}

impl AocDay<usize, usize> for AocDay22 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut bricks: Vec<Brick> = lines
            .map(|line| line.parse().expect("invalid brick"))
            .collect();

        bricks.sort_by_key(|pos| pos.start.z);

        let dimensions = (
            bricks
                .iter()
                .map(|brick| brick.start.x.max(brick.end.x))
                .max()
                .unwrap_or(0)
                + 1,
            bricks
                .iter()
                .map(|brick| brick.start.y.max(brick.end.y))
                .max()
                .unwrap_or(0)
                + 1,
            bricks
                .iter()
                .map(|brick| brick.start.z.max(brick.end.z))
                .max()
                .unwrap_or(0)
                + 1,
        );

        let (new_bricks, brick_map, _) = simulate_fall(&bricks, dimensions, None);

        // which bricks does brick <key> rely on for support (i.e. are below it)?
        let supports_map = get_adjacent_bricks_map(&new_bricks, &brick_map, |z| z.checked_sub(1));

        // which bricks does brick <key> support (i.e. are above it)?
        let supporting_map = get_adjacent_bricks_map(&new_bricks, &brick_map, |z| z.checked_add(1));

        let safe_to_disintegrate = new_bricks
            .iter()
            .enumerate()
            .map(|(i, _)| {
                supporting_map
                    .get(&i)
                    .map(|vec| {
                        vec.iter()
                            .all(|j| supports_map.get(j).map(|vec| vec.len()).unwrap_or(0) >= 2)
                    })
                    .unwrap_or(true)
            })
            .collect();

        AocDay22 {
            bricks: new_bricks,
            safe_to_disintegrate,
            dimensions,
        }
    }
    fn part1(&self) -> usize {
        self.safe_to_disintegrate.iter().filter(|b| **b).count()
    }
    fn part2(&self) -> usize {
        self.safe_to_disintegrate
            .iter()
            .enumerate()
            .filter(|(_, b)| !**b)
            .map(|(unstable_brick, _)| {
                let (_, _, count) =
                    simulate_fall(&self.bricks, self.dimensions, Some(unstable_brick));
                count
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod day22tests {
    use super::*;

    const INPUT: &[&str] = &[
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ];

    #[test]
    fn part1() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 5);
    }

    #[test]
    fn part2() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 7);
    }
}
