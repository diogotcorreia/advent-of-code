use aoc_common::{AocDay, DayError};
use itertools::Itertools;
use memoize::memoize;

#[memoize]
fn process_stone(rounds: u32, stone: u64) -> usize {
    if rounds == 0 {
        1
    } else if stone == 0 {
        process_stone(rounds - 1, 1)
    } else {
        let digits = stone.checked_ilog10().unwrap_or(0) + 1;
        if digits % 2 == 0 {
            let leftmost_digits = stone / 10u64.pow(digits / 2);
            let rightmost_digits = stone % 10u64.pow(digits / 2);
            process_stone(rounds - 1, leftmost_digits) + process_stone(rounds - 1, rightmost_digits)
        } else {
            process_stone(rounds - 1, stone * 2024)
        }
    }
}

pub struct AocDay11 {
    stones: Vec<u64>,
}

impl AocDay<usize, usize> for AocDay11 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let stones = lines
            .next()
            .ok_or(DayError::GenericParseErr("input is empty"))?
            .split_whitespace()
            .map(|s| s.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay11 { stones })
    }
    fn part1(&self) -> usize {
        self.stones
            .iter()
            .map(|stone| process_stone(25, *stone))
            .sum()
    }
    fn part2(&self) -> usize {
        self.stones
            .iter()
            .map(|stone| process_stone(75, *stone))
            .sum()
    }
}

#[cfg(test)]
mod day11tests {
    use super::*;

    const INPUT: &[&str] = &["125 17"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay11::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 55312);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay11::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 65601038650482); // not provided
        Ok(())
    }
}
