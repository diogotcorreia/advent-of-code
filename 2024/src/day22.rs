use std::{iter, str::FromStr};

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

const MOD: u64 = 16777216;

#[derive(Debug, Clone)]
struct Prng {
    seed: u64,
}

impl FromStr for Prng {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { seed: s.parse()? })
    }
}

impl Iterator for Prng {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let secret = self.seed;
        let secret = (secret ^ (secret << 6)) % MOD; // mul by 64
        let secret = (secret ^ (secret >> 5)) % MOD; // div by 32
        let secret = (secret ^ (secret << 11)) % MOD; // mul by 2048

        self.seed = secret;

        Some(secret)
    }
}

pub struct AocDay22 {
    numbers: Vec<Prng>,
}

impl AocDay<u64, u16> for AocDay22 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let numbers = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay22 { numbers })
    }
    fn part1(&self) -> u64 {
        self.numbers
            .iter()
            .cloned()
            .map(|mut prng| prng.nth(1999).unwrap())
            .sum()
    }
    fn part2(&self) -> u16 {
        self.numbers
            .iter()
            .cloned()
            .fold(vec![0u16; 19 * 19 * 19 * 19], |mut acc, prng| {
                let mut seq_seen = vec![false; 19 * 19 * 19 * 19];
                let seed = prng.seed;
                iter::once(seed)
                    .chain(prng.take(2000))
                    .map(|secret| (secret % 10) as i8)
                    .tuple_windows()
                    .fold(0usize, |seq, (a, b)| {
                        let delta = b - a;
                        // convert seq to a base19 number
                        let seq = (seq * 19 + ((delta + 9) as usize)) % (19 * 19 * 19 * 19);

                        if !seq_seen[seq] {
                            acc[seq] += b as u16;
                        }
                        seq_seen[seq] = true;

                        seq
                    });
                acc
            })
            .into_iter()
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod day22tests {
    use super::*;

    const INPUT: &[&str] = &["1", "10", "100", "2024"];
    const INPUT2: &[&str] = &["1", "2", "3", "2024"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay22::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 37327623);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay22::preprocessing_tests(INPUT2)?;
        assert_eq!(day.part2(), 23);
        Ok(())
    }
}
