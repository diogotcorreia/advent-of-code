use std::{cmp::Ordering, collections::HashMap};

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

pub struct AocDay05 {
    restrictions: HashMap<u32, Vec<u32>>,
    books: Vec<Vec<u32>>,
}

fn is_sorted(pages: &[u32], restrictions: &HashMap<u32, Vec<u32>>) -> bool {
    pages.is_sorted_by(|a, b| restrictions.get(a).map_or(false, |r| r.contains(b)))
}

fn sort_pages(pages: &[u32], restrictions: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut pages = pages.to_vec();
    pages.sort_by(|a, b| {
        if restrictions.get(a).map_or(false, |r| r.contains(b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    pages
}

impl AocDay<u32, u32> for AocDay05 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let mut restrictions: HashMap<u32, Vec<u32>> = HashMap::new();
        for line in lines.by_ref().take_while(|l| !l.is_empty()) {
            let (k, v) = line
                .trim()
                .split_once('|')
                .ok_or(DayError::GenericParseErr(
                    "line does not include | separator",
                ))?;
            restrictions.entry(k.parse()?).or_default().push(v.parse()?);
        }

        let books = lines
            .map(|line| {
                line.split(",")
                    .map(|v| v.parse())
                    .process_results(|it| it.collect_vec())
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay05 {
            restrictions,
            books,
        })
    }
    fn part1(&self) -> u32 {
        self.books
            .iter()
            .filter(|v| is_sorted(v, &self.restrictions))
            .map(|v| v[v.len() / 2])
            .sum()
    }
    fn part2(&self) -> u32 {
        self.books
            .iter()
            .filter(|v| !is_sorted(v, &self.restrictions))
            .map(|v| sort_pages(v, &self.restrictions))
            .map(|v| v[v.len() / 2])
            .sum()
    }
}

#[cfg(test)]
mod day05tests {
    use super::*;

    const INPUT: &[&str] = &[
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay05::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 143);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay05::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 123);
        Ok(())
    }
}
