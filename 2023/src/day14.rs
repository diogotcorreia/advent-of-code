use ndarray::prelude::*;

use crate::AocDay;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
    Empty,
}

impl TryFrom<char> for Rock {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Rock::Round),
            '#' => Ok(Rock::Square),
            '.' => Ok(Rock::Empty),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

fn roll_generic<const OUTER_AXIS: usize, const REV: bool>(map: &Array2<Rock>) -> Array2<Rock> {
    let mut last_outer_i = vec![0; map.shape()[1 - OUTER_AXIS]];
    let mut new_map = map.clone();

    macro_rules! cond_rev_iter {
        ($cond: expr, $iter: expr) => {
            if $cond {
                either::Either::Left($iter.rev().enumerate())
            } else {
                either::Either::Right($iter.enumerate())
            }
        };
    }
    macro_rules! index {
        ($outer: expr, $inner: expr) => {
            if REV {
                if OUTER_AXIS == 0 {
                    (map.shape()[OUTER_AXIS] - 1 - $outer, $inner)
                } else {
                    ($inner, map.shape()[OUTER_AXIS] - 1 - $outer)
                }
            } else {
                if OUTER_AXIS == 0 {
                    ($outer, $inner)
                } else {
                    ($inner, $outer)
                }
            }
        };
    }

    for (outer_i, row) in cond_rev_iter!(REV, map.axis_iter(Axis(OUTER_AXIS))) {
        for (inner_i, rock) in row.iter().enumerate() {
            match *rock {
                Rock::Round => {
                    while map[index!(last_outer_i[inner_i], inner_i)] == Rock::Square {
                        last_outer_i[inner_i] += 1;
                    }
                    new_map[index!(outer_i, inner_i)] = Rock::Empty;
                    new_map[index!(last_outer_i[inner_i], inner_i)] = Rock::Round;
                    last_outer_i[inner_i] += 1;
                }
                Rock::Square => last_outer_i[inner_i] = last_outer_i[inner_i].max(outer_i),
                Rock::Empty => {}
            }
        }
    }

    new_map
}

fn roll_north(map: &Array2<Rock>) -> Array2<Rock> {
    roll_generic::<0, false>(map)
}
fn roll_south(map: &Array2<Rock>) -> Array2<Rock> {
    roll_generic::<0, true>(map)
}
fn roll_west(map: &Array2<Rock>) -> Array2<Rock> {
    roll_generic::<1, false>(map)
}
fn roll_east(map: &Array2<Rock>) -> Array2<Rock> {
    roll_generic::<1, true>(map)
}

#[allow(dead_code)]
fn dbg_map(map: &Array2<Rock>) {
    for row in map.outer_iter() {
        for c in row {
            match *c {
                Rock::Round => print!("O"),
                Rock::Square => print!("#"),
                Rock::Empty => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn find_cycle(results: &[usize]) -> Vec<usize> {
    for i in (results.len() / 2)..results.len() {
        let cycle_size = results.len() - i;
        if results[i..] == results[(i - cycle_size)..i] {
            // cycle!
            return results[i..].to_vec();
        }
    }

    panic!("no cycle found");
}

pub struct AocDay14 {
    map: Array2<Rock>,
}

impl AocDay<usize, usize> for AocDay14 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.peekable();
        let width = lines.peek().expect("map does not have any rows").len();
        let map: Vec<Rock> = lines
            .flat_map(|row| {
                row.chars()
                    .map(|c| c.try_into().expect("failed to parse character in map"))
                    .collect::<Vec<_>>()
            })
            .collect();

        AocDay14 {
            map: Array2::from_shape_vec((map.len() / width, width), map)
                .expect("failed to build array2"),
        }
    }
    fn part1(&self) -> usize {
        let height = self.map.shape()[0];
        roll_generic::<0, false>(&self.map)
            .indexed_iter()
            .map(|((y, _), rock)| match rock {
                Rock::Round => height - y,
                _ => 0,
            })
            .sum()
    }
    fn part2(&self) -> usize {
        let height = self.map.nrows();
        let (_, results) = (0..1000).fold((self.map.clone(), vec![]), |(map, mut results), _| {
            let map = roll_north(&map);
            let map = roll_west(&map);
            let map = roll_south(&map);
            let map = roll_east(&map);
            let result: usize = map
                .indexed_iter()
                .map(|((y, _), rock)| match rock {
                    Rock::Round => height - y,
                    _ => 0,
                })
                .sum();
            results.push(result);
            (map, results)
        });

        let cycle = find_cycle(&results);

        let remaining = 1000000000 - 1000;
        cycle[(cycle.len() - 1 + remaining) % cycle.len()]
    }
}

#[cfg(test)]
mod day14tests {
    use super::*;

    const INPUT: &[&str] = &[
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    #[test]
    fn part1() {
        let day = AocDay14::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 136);
    }

    #[test]
    fn part2() {
        let day = AocDay14::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 64);
    }
}
