use std::str::FromStr;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

type Battery = u8;

struct Bank {
    batteries: Vec<Battery>,
}

impl Bank {
    fn largest_joltage<const N: usize>(&self) -> u64 {
        let pos_diff = self.batteries.len() - N;
        let largest =
            self.batteries
                .iter()
                .enumerate()
                .fold([0u8; N], |mut acc, (until_end, &battery)| {
                    acc.iter_mut()
                        .skip(until_end.saturating_sub(pos_diff))
                        .fold(false, |changed, digit| {
                            if changed {
                                *digit = 0;
                                true
                            } else if battery > *digit {
                                *digit = battery;
                                true
                            } else {
                                changed
                            }
                        });
                    acc
                });

        largest
            .iter()
            .rev()
            .fold((0, 1), |(res, power), &digit| {
                (res + digit as u64 * power, power * 10)
            })
            .0
    }
}

impl FromStr for Bank {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or(DayError::GenericParseErr("joltage not digit"))
            })
            .process_results(|it| it.collect_vec())?;
        Ok(Bank { batteries })
    }
}

pub struct AocDay03 {
    banks: Vec<Bank>,
}

impl AocDay<u64, u64> for AocDay03 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let banks = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay03 { banks })
    }
    fn part1(&self) -> u64 {
        self.banks
            .iter()
            .map(|bank| bank.largest_joltage::<2>())
            .sum()
    }
    fn part2(&self) -> u64 {
        self.banks
            .iter()
            .map(|bank| bank.largest_joltage::<12>())
            .sum()
    }
}

#[cfg(test)]
mod day03tests {
    use super::*;

    const INPUT: &[&str] = &[
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay03::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 357);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay03::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 3121910778619);
        Ok(())
    }
}
