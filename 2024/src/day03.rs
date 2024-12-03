use aoc_common::{AocDay, DayError};
use regex::Regex;

pub struct AocDay03 {
    enabled_result: u32,
    disabled_result: u32,
}

impl AocDay<u32, u32> for AocDay03 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let re = Regex::new(r"(do)()\(\)|(don't)()\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        let mut enabled_result = 0;
        let mut disabled_result = 0;
        let mut enabled = true;
        for line in lines {
            for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
                match a {
                    "do" => enabled = true,
                    "don't" => enabled = false,
                    _ => {
                        let a: u32 = a.parse()?;
                        let b: u32 = b.parse()?;
                        if enabled {
                            enabled_result += a * b;
                        } else {
                            disabled_result += a * b;
                        }
                    }
                }
            }
        }

        Ok(AocDay03 {
            enabled_result,
            disabled_result,
        })
    }
    fn part1(&self) -> u32 {
        self.enabled_result + self.disabled_result
    }
    fn part2(&self) -> u32 {
        self.enabled_result
    }
}

#[cfg(test)]
mod day03tests {
    use super::*;

    const INPUT: &[&str] =
        &["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];
    const INPUT2: &[&str] =
        &["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay03::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 161);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay03::preprocessing_tests(INPUT2)?;
        assert_eq!(day.part2(), 48);
        Ok(())
    }
}
