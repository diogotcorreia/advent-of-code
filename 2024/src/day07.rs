use aoc_common::{AocDay, DayError};
use itertools::Itertools;

struct Entry {
    result: u64,
    equation: Vec<u64>,
}

pub struct AocDay07 {
    entries: Vec<Entry>,
}

fn is_valid(target: u64, curr_value: u64, next_values: &[u64]) -> bool {
    if next_values.is_empty() {
        return target == curr_value;
    }

    if curr_value > target {
        return false;
    }

    is_valid(target, curr_value + next_values[0], &next_values[1..])
        || is_valid(target, curr_value * next_values[0], &next_values[1..])
}

fn is_valid2(target: u64, curr_value: u64, next_values: &[u64]) -> bool {
    if next_values.is_empty() {
        return target == curr_value;
    }

    if curr_value > target {
        return false;
    }

    is_valid2(target, curr_value + next_values[0], &next_values[1..])
        || is_valid2(target, curr_value * next_values[0], &next_values[1..])
        || is_valid2(
            target,
            concat_numbers(curr_value, next_values[0]),
            &next_values[1..],
        )
}

fn concat_numbers(left: u64, right: u64) -> u64 {
    let digits_right = right.checked_ilog10().unwrap_or(0) + 1;
    left * (10u64.pow(digits_right)) + right
}

impl AocDay<u64, u64> for AocDay07 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let entries = lines
            .map(|line| {
                let (result, equation) = line.split_once(": ").ok_or(DayError::GenericParseErr(
                    "line does not contain colon to split on",
                ))?;
                let equation = equation
                    .split_whitespace()
                    .map(|v| v.parse())
                    .process_results(|it| it.collect_vec())?;

                Ok::<_, DayError>(Entry {
                    result: result.parse()?,
                    equation,
                })
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay07 { entries })
    }
    fn part1(&self) -> u64 {
        self.entries
            .iter()
            .filter(|entry| is_valid(entry.result, entry.equation[0], &entry.equation[1..]))
            .map(|entry| entry.result)
            .sum()
    }
    fn part2(&self) -> u64 {
        self.entries
            .iter()
            .filter(|entry| is_valid2(entry.result, entry.equation[0], &entry.equation[1..]))
            .map(|entry| entry.result)
            .sum()
    }
}

#[cfg(test)]
mod day07tests {
    use super::*;

    const INPUT: &[&str] = &[
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay07::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 3749);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay07::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 11387);
        Ok(())
    }

    #[test]
    fn concat_nums() {
        assert_eq!(concat_numbers(123, 456), 123456);
        assert_eq!(concat_numbers(123, 1), 1231);
        assert_eq!(concat_numbers(120, 10), 12010);
    }
}
