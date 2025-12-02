use std::ops::RangeInclusive;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

fn is_number_invalid(group_size: u32, mut n: u64) -> bool {
    let factor = 10_u64.pow(group_size);
    let mut prev = n % factor;
    n /= factor;
    while n != 0 {
        let next = n % factor;
        n /= factor;
        if prev != next {
            return false;
        }
        prev = next;
    }
    true
}

fn sum_invalid_part1(range: RangeInclusive<u64>) -> u64 {
    range
        .filter(|&n| {
            let digits = n.ilog10() + 1;
            if digits % 2 == 1 {
                false
            } else {
                let half = digits / 2;
                is_number_invalid(half, n)
            }
        })
        .sum()
}

fn sum_invalid_part2(range: RangeInclusive<u64>) -> u64 {
    range
        .filter(|&n| {
            let digits = n.ilog10() + 1;
            for i in 1..=(digits / 2) {
                if digits % i == 0 && is_number_invalid(i, n) {
                    return true;
                }
            }
            false
        })
        .sum()
}

pub struct AocDay02 {
    ranges: Vec<RangeInclusive<u64>>,
}

impl AocDay<u64, u64> for AocDay02 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let ranges = lines
            .flat_map(|line| {
                line.split(",")
                    .map(|range| {
                        let (from, to) = range
                            .split_once("-")
                            .ok_or(DayError::GenericParseErr("range does not have delimiter"))?;
                        Ok::<_, DayError>(RangeInclusive::new(from.parse()?, to.parse()?))
                    })
                    .collect_vec()
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay02 { ranges })
    }
    fn part1(&self) -> u64 {
        self.ranges
            .iter()
            .map(|range| sum_invalid_part1(range.clone()))
            .sum()
    }
    fn part2(&self) -> u64 {
        self.ranges
            .iter()
            .map(|range| sum_invalid_part2(range.clone()))
            .sum()
    }
}

#[cfg(test)]
mod day02tests {
    use super::*;

    const INPUT: &[&str] = &["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay02::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 1227775554);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay02::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 4174379265);
        Ok(())
    }
}
