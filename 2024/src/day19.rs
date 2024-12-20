use std::{
    cell::RefCell,
    ops::{Index, IndexMut},
    rc::Rc,
    str::FromStr,
};

use aoc_common::{AocDay, DayError};
use aoc_common_macros::TryFromChar;
use itertools::Itertools;
use pathfinding::prelude::{count_paths, dijkstra};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromChar)]
enum Color {
    #[char_repr = 'w']
    White,
    #[char_repr = 'u']
    Blue,
    #[char_repr = 'b']
    Black,
    #[char_repr = 'r']
    Red,
    #[char_repr = 'g']
    Green,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Towel {
    pattern: Vec<Color>,
}

impl FromStr for Towel {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .chars()
            .map(|c| c.try_into())
            .process_results(|it| it.collect_vec())?;

        Ok(Self { pattern })
    }
}

#[derive(Debug)]
enum ColorTreeNode {
    None,
    Leaf,
    Expanded(Rc<RefCell<ColorTreeNodeData>>),
    ExpandedLeaf(Rc<RefCell<ColorTreeNodeData>>),
}

#[derive(Debug)]
struct ColorTreeNodeData {
    white: ColorTreeNode,
    blue: ColorTreeNode,
    black: ColorTreeNode,
    red: ColorTreeNode,
    green: ColorTreeNode,
}

impl ColorTreeNodeData {
    fn new() -> Self {
        Self {
            white: ColorTreeNode::None,
            blue: ColorTreeNode::None,
            black: ColorTreeNode::None,
            red: ColorTreeNode::None,
            green: ColorTreeNode::None,
        }
    }

    fn insert(&mut self, colors: &[Color]) {
        let is_leaf = colors.len() == 1;

        let node = &mut self[colors[0]];

        match node {
            ColorTreeNode::None => {
                if is_leaf {
                    *node = ColorTreeNode::Leaf;
                } else {
                    let mut data = ColorTreeNodeData::new();
                    data.insert(&colors[1..]);
                    *node = ColorTreeNode::Expanded(Rc::new(RefCell::new(data)));
                }
            }
            ColorTreeNode::Leaf if !is_leaf => {
                let mut data = ColorTreeNodeData::new();
                data.insert(&colors[1..]);
                *node = ColorTreeNode::ExpandedLeaf(Rc::new(RefCell::new(data)));
            }
            ColorTreeNode::Expanded(data) => {
                if !is_leaf {
                    data.borrow_mut().insert(&colors[1..]);
                } else {
                    *node = ColorTreeNode::ExpandedLeaf(data.clone());
                }
            }
            ColorTreeNode::ExpandedLeaf(data) if !is_leaf => {
                data.borrow_mut().insert(&colors[1..]);
            }
            _ => {}
        }
    }

    /// Return an iterator of the size of the towels that are prefixes of the given colors.
    /// Only size is returned since there is only one possible towel for a given size that also
    /// matches the given colors.
    fn find_all<'a>(&'a self, colors: &'a [Color]) -> impl Iterator<Item = usize> + use<'a> {
        let mut curr_level: Option<Rc<RefCell<ColorTreeNodeData>>> = None;
        colors
            .iter()
            .enumerate()
            .map_while(move |(i, color)| {
                macro_rules! handle_node {
                    ($node: expr) => {
                        match $node {
                            ColorTreeNode::None => {
                                curr_level = None;
                                None
                            }
                            ColorTreeNode::Leaf => {
                                curr_level = None;
                                Some(Some(i))
                            }
                            ColorTreeNode::Expanded(data) => {
                                curr_level = Some(data.clone());
                                Some(None)
                            }
                            ColorTreeNode::ExpandedLeaf(data) => {
                                curr_level = Some(data.clone());
                                Some(Some(i))
                            }
                        }
                    };
                }
                if i == 0 {
                    let node = &self[*color];
                    handle_node!(node)
                } else if let Some(l) = curr_level.clone() {
                    let node = &l.as_ref().borrow()[*color];
                    handle_node!(node)
                } else {
                    None
                }
            })
            .flatten()
            .map(|size| size + 1)
    }
}

impl Index<Color> for ColorTreeNodeData {
    type Output = ColorTreeNode;

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.white,
            Color::Blue => &self.blue,
            Color::Black => &self.black,
            Color::Red => &self.red,
            Color::Green => &self.green,
        }
    }
}

impl IndexMut<Color> for ColorTreeNodeData {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self.white,
            Color::Blue => &mut self.blue,
            Color::Black => &mut self.black,
            Color::Red => &mut self.red,
            Color::Green => &mut self.green,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WantedTowel<'a> {
    towel: &'a Towel,
    found_index: usize,
}

impl<'a> WantedTowel<'a> {
    fn advance_index(&self, count: usize) -> Self {
        Self {
            towel: self.towel,
            found_index: self.found_index + count,
        }
    }

    fn is_completed(&self) -> bool {
        self.found_index >= self.towel.pattern.len()
    }
}

impl<'a> From<&'a Towel> for WantedTowel<'a> {
    fn from(towel: &'a Towel) -> Self {
        Self {
            towel,
            found_index: 0,
        }
    }
}

fn find_possible_towels<'a, 'b: 'a>(
    base_towels: &'b ColorTreeNodeData,
    wanted_towel: WantedTowel<'a>,
) -> impl Iterator<Item = WantedTowel<'a>> + use<'a, 'b> {
    base_towels
        .find_all(&wanted_towel.towel.pattern[wanted_towel.found_index..])
        .map(move |i| wanted_towel.advance_index(i))
}

pub struct AocDay19 {
    base_towels: ColorTreeNodeData,
    wanted_towels: Vec<Towel>,
}

impl AocDay<usize, usize> for AocDay19 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let base_towels = lines
            .next()
            .map(|line| {
                line.split(", ")
                    .map(|towel| towel.parse::<Towel>())
                    .process_results(|it| {
                        let mut tree = ColorTreeNodeData::new();
                        it.for_each(|t| tree.insert(&t.pattern));
                        tree
                    })
            })
            .ok_or(DayError::GenericParseErr(
                "base towels not present in input",
            ))??;

        let wanted_towels = lines
            .skip(1)
            .map(|towel| towel.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay19 {
            base_towels,
            wanted_towels,
        })
    }
    fn part1(&self) -> usize {
        self.wanted_towels
            .iter()
            .map(WantedTowel::from)
            .filter(|towel| {
                dijkstra(
                    towel,
                    |t| find_possible_towels(&self.base_towels, t.clone()).map(|v| (v, 1)),
                    |t| t.is_completed(),
                )
                .is_some()
            })
            .count()
    }
    fn part2(&self) -> usize {
        self.wanted_towels
            .iter()
            .map(WantedTowel::from)
            .map(|towel| {
                count_paths(
                    towel,
                    |t| find_possible_towels(&self.base_towels, t.clone()),
                    |t| t.is_completed(),
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod day19tests {
    use super::*;

    const INPUT: &[&str] = &[
        "r, wr, b, g, bwu, rb, gb, br",
        "",
        "brwrr",
        "bggr",
        "gbbr",
        "rrbgbr",
        "ubwu",
        "bwurrg",
        "brgr",
        "bbrgwb",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay19::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 6);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay19::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 16);
        Ok(())
    }
}
