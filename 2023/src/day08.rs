use std::collections::HashMap;

use crate::AocDay;

enum Direction {
    Left,
    Right,
}

struct Node {
    left: String,
    right: String,
}

fn steps_to_z<const PART2: bool>(day: &AocDay08, starting: &str) -> u64 {
    let mut steps = 0;
    let mut curr_node = starting;
    for dir in day.directions.iter().cycle() {
        let node = day
            .nodes
            .get(curr_node)
            .expect("could not find referenced node");
        curr_node = match dir {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };

        steps += 1;

        if !PART2 && curr_node == "ZZZ" {
            break;
        }
        if PART2 && curr_node.ends_with('Z') {
            break;
        }
    }

    steps
}

pub struct AocDay08 {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl AocDay<u64, u64> for AocDay08 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let directions = lines
            .next()
            .expect("cannot find line with directions")
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!("char direction"),
            })
            .collect();

        lines.next().expect("cannot find separation line");

        let mut nodes = HashMap::new();

        for line in lines {
            let (name, target) = line.split_once(" = ").expect("invalid node");
            let target = target.strip_prefix('(').expect("invalid node");
            let target = target.strip_suffix(')').expect("invalid node");
            let (left, right) = target.split_once(", ").expect("invalid node");

            nodes.insert(
                name.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }

        AocDay08 { directions, nodes }
    }
    fn part1(&self) -> u64 {
        steps_to_z::<false>(self, "AAA")
    }
    fn part2(&self) -> u64 {
        self.nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|starting_node| steps_to_z::<true>(self, starting_node))
            .reduce(num::integer::lcm)
            .expect("input has no nodes")
    }
}

#[cfg(test)]
mod day08tests {
    use super::*;

    const INPUT: &[&str] = &[
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];
    const INPUT2: &[&str] = &[
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    #[test]
    fn part1() {
        let day = AocDay08::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 6);
    }

    #[test]
    fn part2() {
        let day = AocDay08::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 6);
    }
}
