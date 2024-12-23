use std::collections::HashSet;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

fn comp_name_to_usize(name: &str) -> usize {
    let mut chars = name.chars();
    let left = (chars.next().unwrap() as u8 - b'a') as usize;
    let right = (chars.next().unwrap() as u8 - b'a') as usize;

    left * 26 + right
}

fn usize_to_comp_name(num: usize) -> String {
    let left = num / 26;
    let right = num % 26;

    format!(
        "{}{}",
        (left as u8 + b'a') as char,
        (right as u8 + b'a') as char
    )
}

fn is_t(computer: usize) -> bool {
    computer / 26 == (b't' - b'a') as usize
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
struct BronKerbosch<'a> {
    connections: &'a [Vec<usize>; 26 * 26],
    best_result: HashSet<usize>,
}

impl<'a> BronKerbosch<'a> {
    fn run(&mut self, r: HashSet<usize>, p: &HashSet<usize>, mut x: HashSet<usize>) {
        if p.is_empty() && x.is_empty() {
            if r.len() > self.best_result.len() {
                self.best_result = r;
            }
            return;
        }
        if r.len() + p.len() <= self.best_result.len() {
            return;
        }
        let mut curr_p = p.clone();
        for v in p {
            let neighbours: HashSet<usize> = self.connections[*v].iter().cloned().collect();
            let mut new_r = r.clone();
            new_r.insert(*v);
            let new_p = neighbours.intersection(&curr_p).cloned().collect();
            let new_x = neighbours.intersection(&x).cloned().collect();
            self.run(new_r, &new_p, new_x);
            curr_p.remove(v);
            x.insert(*v);
        }
    }
}

pub struct AocDay23 {
    computers: HashSet<usize>,
    connections: [Vec<usize>; 26 * 26],
}

impl AocDay<usize, String> for AocDay23 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let mut computers = HashSet::new();
        let mut connections = [0; 26 * 26].map(|_| Vec::new());

        for line in lines {
            let (left, right) = line.split_once('-').unwrap();
            let (left, right) = (comp_name_to_usize(left), comp_name_to_usize(right));

            computers.insert(left);
            computers.insert(right);

            connections[left].push(right);
            connections[right].push(left);
        }

        Ok(AocDay23 {
            computers,
            connections,
        })
    }
    fn part1(&self) -> usize {
        let a = self
            .computers
            .iter()
            .filter(|comp| is_t(**comp))
            .flat_map(|comp| {
                self.connections[*comp]
                    .iter()
                    .tuple_combinations()
                    .filter(|(left, right)| self.connections[**left].contains(right))
                    .map(|(left, right)| {
                        let mut a = [*comp, *left, *right];
                        a.sort();
                        a.to_vec()
                    })
                    .collect_vec()
            })
            .collect::<HashSet<Vec<usize>>>();

        a.len()
    }
    fn part2(&self) -> String {
        let mut algo = BronKerbosch {
            connections: &self.connections,
            best_result: HashSet::new(),
        };

        algo.run(HashSet::new(), &self.computers, HashSet::new());

        algo.best_result
            .into_iter()
            .sorted()
            .map(usize_to_comp_name)
            .join(",")
    }
}

#[cfg(test)]
mod day23tests {
    use super::*;

    const INPUT: &[&str] = &[
        "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka", "wh-tc",
        "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka", "td-qp", "aq-cg",
        "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de", "kh-ta", "co-tc", "wh-qp",
        "tb-vc", "td-yn",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay23::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 7);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay23::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), "co,de,ka,ta");
        Ok(())
    }
}
