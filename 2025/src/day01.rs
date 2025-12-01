use std::str::FromStr;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

struct Dial {
    max: i32,
    current: i32,
}

impl Dial {
    fn new(max: i32, starting: i32) -> Self {
        Dial {
            max: max + 1,
            current: starting,
        }
    }

    fn rotate(&mut self, rotation: &Rotation) -> (i32, i32) {
        let move_to = self.current + rotation.delta;
        let dest = move_to.rem_euclid(self.max);
        let mut zero_count = move_to.div_euclid(self.max).abs();
        if dest == 0 && rotation.delta < 0 {
            zero_count += 1;
        }
        if self.current == 0 && rotation.delta < 0 {
            zero_count -= 1;
        }
        self.current = dest;
        (dest, zero_count)
    }
}

struct Rotation {
    delta: i32,
}

impl FromStr for Rotation {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mult = match s.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => return Err(DayError::GenericParseErr("invalid direction")),
        };

        let step: i32 = s
            .get(1..)
            .ok_or(DayError::GenericParseErr("no delta"))?
            .parse()?;

        Ok(Rotation { delta: mult * step })
    }
}

pub struct AocDay01 {
    rotations: Vec<Rotation>,
}

impl AocDay<usize, i32> for AocDay01 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let rotations = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay01 { rotations })
    }
    fn part1(&self) -> usize {
        let mut dial = Dial::new(99, 50);

        self.rotations
            .iter()
            .map(|rot| dial.rotate(rot).0)
            .filter(|&res| res == 0)
            .count()
    }
    fn part2(&self) -> i32 {
        let mut dial = Dial::new(99, 50);

        self.rotations.iter().map(|rot| dial.rotate(rot).1).sum()
    }
}

#[cfg(test)]
mod day01tests {
    use super::*;

    const INPUT: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay01::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 3);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay01::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 6);
        Ok(())
    }
}
