use std::ops::RangeInclusive;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

pub struct AocDay05 {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn normalize_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    enum BoundType {
        Open,
        Close,
    }
    ranges
        .iter()
        .flat_map(|range| {
            assert!(!range.is_empty());
            [
                (range.start(), BoundType::Open),
                (range.end(), BoundType::Close),
            ]
        })
        .sorted()
        .fold(
            (vec![], 0usize, 0),
            |(mut res, mut depth, mut open_id), (&id, bound)| {
                match bound {
                    BoundType::Open => {
                        if depth == 0 {
                            open_id = id;
                        }
                        depth += 1;
                    }
                    BoundType::Close => {
                        depth -= 1;
                        if depth == 0 {
                            res.push(open_id..=id);
                        }
                    }
                }
                (res, depth, open_id)
            },
        )
        .0
}

impl AocDay05 {
    fn is_fresh(&self, id: u64) -> bool {
        let possible_range = self.fresh.partition_point(|range| id >= *range.start());
        self.fresh
            .get(possible_range.saturating_sub(1))
            .map(|range| range.contains(&id))
            .unwrap_or(false)
    }
}

impl AocDay<usize, u64> for AocDay05 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let fresh = lines
            .by_ref()
            .take_while(|l| !l.trim().is_empty())
            .map(|l| {
                let (from, to) = l.split_once('-').ok_or(DayError::GenericParseErr(
                    "range does not container delimiter",
                ))?;
                let (from, to) = (from.parse()?, to.parse()?);
                Ok::<_, DayError>(from..=to)
            })
            .process_results(|it| it.collect_vec())?;

        let ingredients = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay05 {
            fresh: normalize_ranges(&fresh),
            ingredients,
        })
    }
    fn part1(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|&&ingredient| self.is_fresh(ingredient))
            .count()
    }
    fn part2(&self) -> u64 {
        let ranges = normalize_ranges(&self.fresh);
        ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }
}

#[cfg(test)]
mod day05tests {
    use super::*;

    const INPUT: &[&str] = &[
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay05::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 3);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay05::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 14);
        Ok(())
    }
}
