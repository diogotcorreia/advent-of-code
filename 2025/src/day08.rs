use std::str::FromStr;

use aoc_common::{AocDay, DayError};
use disjoint::DisjointSet;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for Pos {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split
            .next()
            .ok_or(DayError::GenericParseErr("missing coordinate x"))?;
        let y = split
            .next()
            .ok_or(DayError::GenericParseErr("missing coordinate y"))?;
        let z = split
            .next()
            .ok_or(DayError::GenericParseErr("missing coordinate z"))?;
        Ok(Pos {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

impl Pos {
    fn distance_to(&self, other: &Pos) -> u64 {
        // no need to sqrt since we are just comparing
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

// edge in "graph"
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    weight: u64,
    from: usize,
    to: usize,
}

fn calculate_edges(positions: &[Pos]) -> Vec<Edge> {
    (0..positions.len())
        .tuple_combinations()
        .map(|(pos1, pos2)| Edge {
            weight: positions[pos1].distance_to(&positions[pos2]),
            from: pos1,
            to: pos2,
        })
        .sorted_by_key(|edge| edge.weight)
        .collect_vec()
}

pub struct AocDay08 {
    junction_boxes: Vec<Pos>,
    edges: Vec<Edge>,
}

impl AocDay<usize, u64> for AocDay08 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let junction_boxes = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        let edges = calculate_edges(&junction_boxes);
        Ok(AocDay08 {
            junction_boxes,
            edges,
        })
    }
    fn part1(&self) -> usize {
        self.part1_inner::<1000>()
    }
    fn part2(&self) -> u64 {
        let mut vertices = DisjointSet::with_len(self.junction_boxes.len());
        let mut last_connected = None;
        for edge in &self.edges {
            if vertices.join(edge.from, edge.to) {
                last_connected = Some((edge.from, edge.to));
            }
        }

        let (pos1, pos2) = last_connected.expect("no solution found");
        self.junction_boxes[pos1].x * self.junction_boxes[pos2].x
    }
}

impl AocDay08 {
    fn part1_inner<const MAX_ITER: usize>(&self) -> usize {
        let mut vertices = DisjointSet::with_len(self.junction_boxes.len());
        for edge in self.edges.iter().take(MAX_ITER) {
            vertices.join(edge.from, edge.to);
        }
        vertices
            .sets()
            .iter()
            .map(|set| set.len())
            .k_largest(3)
            .product()
    }
}

#[cfg(test)]
mod day08tests {
    use super::*;

    const INPUT: &[&str] = &[
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay08::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1_inner::<10>(), 40);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay08::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 25272);
        Ok(())
    }
}
