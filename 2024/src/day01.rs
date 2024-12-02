use aoc_common::{AocDay, DayError};
use itertools::Itertools;

pub struct AocDay01 {
    list_l: Vec<u32>,
    list_r: Vec<u32>,
}

impl AocDay<u32, u32> for AocDay01 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let (mut list_l, mut list_r): (Vec<_>, Vec<_>) = lines
            .map(|l| {
                l.split_once(" ")
                    .ok_or(DayError::GenericParseErr("line must have two numbers"))
                    .and_then(|(l, r)| {
                        let l = l.trim().parse::<u32>()?;
                        let r = r.trim().parse::<u32>()?;
                        Ok((l, r))
                    })
            })
            .process_results(|iter| iter.unzip())?;

        list_l.sort();
        list_r.sort();

        Ok(AocDay01 { list_l, list_r })
    }
    fn part1(&self) -> u32 {
        (self.list_l)
            .iter()
            .zip(&self.list_r)
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }
    fn part2(&self) -> u32 {
        let counts = self.list_r.iter().counts();

        self.list_l
            .iter()
            .map(|l| l * (*counts.get(l).unwrap_or(&0) as u32))
            .sum()
    }
}

#[cfg(test)]
mod day01tests {
    use super::*;

    const INPUT: &[&str] = &["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay01::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 11);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay01::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 31);
        Ok(())
    }
}
