use std::{cmp::Ordering, str::FromStr};

use itertools::{EitherOrBoth, Itertools};

use crate::AocDay;

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketElement {
    List(Vec<PacketElement>),
    Constant(i32),
}

impl PacketElement {
    fn cmp_internal(left: &[PacketElement], right: &[PacketElement]) -> Ordering {
        for comparison in left.iter().zip_longest(right) {
            match comparison {
                EitherOrBoth::Both(x, y) => {
                    let result = x.cmp(y);
                    if !result.is_eq() {
                        return result;
                    }
                }
                EitherOrBoth::Left(..) => return Ordering::Greater,
                EitherOrBoth::Right(..) => return Ordering::Less,
            }
        }

        Ordering::Equal
    }
}

impl FromStr for PacketElement {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().ok_or(ParseErr)? != '[' {
            return Ok(Self::Constant(s.parse().map_err(|_| ParseErr)?));
        }
        if s.len() == 2 {
            return Ok(Self::List(Vec::new()));
        }

        let mut element_boundaries: Vec<usize> = vec![0];
        let mut depth = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' if depth == 1 => element_boundaries.push(i),
                _ => {}
            }
        }
        element_boundaries.push(s.len() - 1);

        let children = element_boundaries
            .iter()
            .zip(element_boundaries.iter().skip(1))
            .map(|(&i, &j)| s[(i + 1)..j].parse())
            .collect::<Result<_, _>>()?;

        Ok(Self::List(children))
    }
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        if let PacketElement::Constant(x) = self {
            if let PacketElement::Constant(y) = other {
                return x.cmp(y);
            }
        }

        match (self, other) {
            (Self::Constant(x), Self::Constant(y)) => x.cmp(y),
            (Self::Constant(x), Self::List(vec)) => {
                PacketElement::cmp_internal(&[PacketElement::Constant(*x)], vec)
            }
            (Self::List(vec), Self::Constant(y)) => {
                PacketElement::cmp_internal(vec, &[PacketElement::Constant(*y)])
            }
            (Self::List(v1), Self::List(v2)) => PacketElement::cmp_internal(v1, v2),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay13 {
    packets: Vec<PacketElement>,
}

impl AocDay<usize, usize> for AocDay13 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let packets = lines.filter_map(|line| line.parse().ok()).collect();

        AocDay13 { packets }
    }
    fn part1(&self) -> usize {
        self.packets
            .iter()
            .step_by(2)
            .zip(self.packets.iter().skip(1).step_by(2))
            .enumerate()
            .filter(|(_, (x, y))| x < y)
            .map(|(i, _)| i + 1)
            .sum()
    }
    fn part2(&self) -> usize {
        let mut sorted_packets = self.packets.clone();
        let decoder_keys: (PacketElement, PacketElement) =
            ("[[2]]".parse().unwrap(), "[[6]]".parse().unwrap());
        sorted_packets.push(decoder_keys.0.clone());
        sorted_packets.push(decoder_keys.1.clone());

        sorted_packets.sort();

        sorted_packets
            .iter()
            .enumerate()
            .filter(|(_, x)| x == &&decoder_keys.0 || x == &&decoder_keys.1)
            .map(|(i, _)| i + 1)
            .product()
    }
}

#[cfg(test)]
mod day13tests {
    use super::*;

    const INPUT: &[&str] = &[
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    #[test]
    fn part1() {
        let day = AocDay13::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part2() {
        let day = AocDay13::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 140);
    }
}
