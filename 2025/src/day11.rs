use std::collections::HashMap;

use aoc_common::{AocDay, DayError};
use itertools::Itertools;
use pathfinding::prelude::count_paths;

pub struct AocDay11 {
    nodes: HashMap<String, Vec<String>>,
}

impl AocDay11 {
    fn count_paths_from_to(
        &self,
        starting_node: &str,
        end_node: &str,
        skip_node: Option<&str>,
    ) -> usize {
        let def = vec![];
        count_paths(
            starting_node,
            |&node| {
                self.nodes
                    .get(node)
                    .unwrap_or(&def)
                    .iter()
                    .filter(|n| skip_node.map(|skip| skip != *n).unwrap_or(true))
                    .map(|s| s.as_ref())
            },
            |&node| node == end_node,
        )
    }
}

impl AocDay<usize, usize> for AocDay11 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let nodes = lines
            .map(|line| {
                let (name, outputs) = line
                    .split_once(": ")
                    .ok_or(DayError::GenericParseErr("no name-output delimiter"))?;

                Ok::<_, DayError>((
                    name.to_string(),
                    outputs
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect_vec(),
                ))
            })
            .process_results(|it| it.collect())?;

        Ok(AocDay11 { nodes })
    }
    fn part1(&self) -> usize {
        self.count_paths_from_to("you", "out", None)
    }
    fn part2(&self) -> usize {
        let svr_to_dac = self.count_paths_from_to("svr", "dac", Some("fft"));
        let dac_to_fft = self.count_paths_from_to("dac", "fft", None);
        let fft_to_out = self.count_paths_from_to("fft", "out", Some("dac"));

        let svr_to_fft = self.count_paths_from_to("svr", "fft", Some("dac"));
        let fft_to_dac = self.count_paths_from_to("fft", "dac", None);
        let dac_to_out = self.count_paths_from_to("dac", "out", Some("fft"));

        svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
    }
}

#[cfg(test)]
mod day11tests {
    use super::*;

    const INPUT1: &[&str] = &[
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];

    const INPUT2: &[&str] = &[
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay11::preprocessing_tests(INPUT1)?;
        assert_eq!(day.part1(), 5);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay11::preprocessing_tests(INPUT2)?;
        assert_eq!(day.part2(), 2);
        Ok(())
    }
}
