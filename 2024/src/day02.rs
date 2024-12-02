use aoc_common::AocDay;
use itertools::Itertools;

enum Gradient {
    Increasing,
    Decreasing,
}

fn is_valid_report<'a>(iter: impl Iterator<Item = &'a u32>) -> bool {
    let mut gradient: Option<Gradient> = None;
    iter.tuple_windows().all(|(&a, &b)| {
        a.abs_diff(b) <= 3
            && match gradient {
                Some(Gradient::Increasing) => a < b,
                Some(Gradient::Decreasing) => a > b,
                None if a < b => {
                    gradient = Some(Gradient::Increasing);
                    true
                }
                None if a > b => {
                    gradient = Some(Gradient::Decreasing);
                    true
                }
                None => false,
            }
    })
}

pub struct AocDay02 {
    records: Vec<Vec<u32>>,
}

impl AocDay<usize, usize> for AocDay02 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let records = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|p| p.parse().expect("record entry is not number"))
                    .collect_vec()
            })
            .collect_vec();

        AocDay02 { records }
    }
    fn part1(&self) -> usize {
        self.records
            .iter()
            .filter(|record| is_valid_report(record.iter()))
            .count()
    }
    fn part2(&self) -> usize {
        self.records
            .iter()
            .filter(|record| {
                (0..record.len())
                    .map(|i| std::iter::chain(record.iter().take(i), record.iter().skip(i + 1)))
                    .any(is_valid_report)
            })
            .count()
    }
}

#[cfg(test)]
mod day02tests {
    use super::*;

    const INPUT: &[&str] = &[
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    #[test]
    fn part1() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 2);
    }

    #[test]
    fn part2() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 4);
    }
}
