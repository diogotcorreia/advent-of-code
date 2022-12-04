use std::{num::ParseIntError, str::FromStr, u8};

use crate::AocDay;

struct SectionPair(Section, Section);

impl FromStr for SectionPair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s
            .split_once(",")
            .expect("Section pair must contain , delimiter");

        Ok(SectionPair(pair.0.parse()?, pair.1.parse()?))
    }
}

impl SectionPair {
    fn pair_contains(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }
    fn pair_overlaps(&self) -> bool {
        self.0.overlaps_with(&self.1)
    }
}

struct Section(u128);

impl FromStr for Section {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds = s.split_once("-").expect("Section must contain - delimiter");

        Ok(Self::from_range(bounds.0.parse()?, bounds.1.parse()?))
    }
}

impl Section {
    fn from_range(start: u8, end: u8) -> Section {
        let mut mask: u128 = 0;

        for _ in start..=end {
            // Add the necessary bits
            mask <<= 1;
            mask |= 1;
        }
        for _ in 1..start {
            // Move them into place
            mask <<= 1;
        }

        Section(mask)
    }

    fn fully_contains(&self, other: &Section) -> bool {
        self.0 | other.0 == self.0
    }
    fn overlaps_with(&self, other: &Section) -> bool {
        self.0 & other.0 != 0
    }
}

pub struct AocDay04 {
    section_pairs: Vec<SectionPair>,
}

impl AocDay<usize, usize> for AocDay04 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        AocDay04 {
            section_pairs: lines
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }
    fn part1(&self) -> usize {
        self.section_pairs
            .iter()
            .filter(|x| x.pair_contains())
            .count()
    }
    fn part2(&self) -> usize {
        self.section_pairs
            .iter()
            .filter(|x| x.pair_overlaps())
            .count()
    }
}

#[cfg(test)]
mod day04tests {
    use super::*;

    const INPUT: &'static [&'static str] = &[
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn part1() {
        let day = AocDay04::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 2);
    }

    #[test]
    fn part2() {
        let day = AocDay04::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 4);
    }
}
