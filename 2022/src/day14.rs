use std::{collections::HashSet, hash::Hash, str::FromStr};

use crate::AocDay;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn sum_vec(&self, vec: (i32, i32)) -> Self {
        Self(self.0 + vec.0, self.1 + vec.1)
    }
}

impl FromStr for Pos {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseErr)?;

        Ok(Self(
            x.parse().map_err(|_| ParseErr)?,
            y.parse().map_err(|_| ParseErr)?,
        ))
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Clone)]
struct Cave {
    occupied: HashSet<Pos>,
    lowest_rock: i32,
}

impl Cave {
    fn new() -> Self {
        Self {
            occupied: HashSet::new(),
            lowest_rock: 0,
        }
    }

    fn range(start: i32, end: i32) -> impl Iterator<Item = i32> {
        if start < end {
            start..=end
        } else {
            end..=start
        }
    }

    fn draw_line(&mut self, start: &Pos, end: &Pos) {
        for x in Self::range(start.0, end.0) {
            for y in Self::range(start.1, end.1) {
                self.occupied.insert(Pos(x, y));
            }
        }
        self.lowest_rock = self.lowest_rock.max(start.1).max(end.1);
    }

    fn simulate_sand_falling<const FLOOR: bool>(&mut self) -> bool {
        let mut sand_pos = Pos(500, 0);

        let stop_height = match FLOOR {
            true => self.lowest_rock + 1,
            false => self.lowest_rock,
        };
        while sand_pos.1 < stop_height {
            let next_pos = [(0, 1), (-1, 1), (1, 1)]
                .iter()
                .map(|&vec| sand_pos.sum_vec(vec))
                .find(|pos| !self.occupied.contains(pos));

            match next_pos {
                Some(next_pos) => sand_pos = next_pos,
                None => {
                    self.occupied.insert(sand_pos);
                    return true;
                }
            }
        }

        if FLOOR {
            self.occupied.insert(sand_pos);
        }
        false
    }
}

pub struct AocDay14 {
    cave: Cave,
}

impl AocDay<i32, i32> for AocDay14 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut cave = Cave::new();

        for line in lines {
            let points = line
                .split(" -> ")
                .map(|x| x.parse())
                .collect::<Result<Vec<Pos>, _>>()
                .expect("Failed to parse line");

            points
                .iter()
                .zip(points.iter().skip(1))
                .for_each(|(start, end)| cave.draw_line(start, end));
        }

        AocDay14 { cave }
    }
    fn part1(&self) -> i32 {
        let mut cave = self.cave.clone();
        let mut i = 0;
        while cave.simulate_sand_falling::<false>() {
            i += 1;
        }
        i
    }
    fn part2(&self) -> i32 {
        let mut cave = self.cave.clone();
        let target_pos = Pos(500, 0);
        let mut i = 0;
        while !cave.occupied.contains(&target_pos) {
            cave.simulate_sand_falling::<true>();
            i += 1;
        }
        i
    }
}

#[cfg(test)]
mod day14tests {
    use super::*;

    const INPUT: &[&str] = &[
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn part1() {
        let day = AocDay14::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 24);
    }

    #[test]
    fn part2() {
        let day = AocDay14::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 93);
    }
}
