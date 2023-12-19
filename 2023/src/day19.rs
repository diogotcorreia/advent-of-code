use std::{collections::HashMap, str::FromStr};

use crate::AocDay;

#[derive(Clone)]
struct Piece<T> {
    x: T,
    m: T,
    a: T,
    s: T,
}

type PieceRanges = Piece<Option<Range>>;

impl<T: Clone> Piece<T> {
    fn new(init: T) -> Self {
        Self {
            x: init.clone(),
            m: init.clone(),
            a: init.clone(),
            s: init.clone(),
        }
    }
}

impl<T> Piece<T> {
    fn set(&mut self, field: &Field, value: T) {
        match field {
            Field::XCoolLooking => self.x = value,
            Field::Musical => self.m = value,
            Field::Aerodynamic => self.a = value,
            Field::Shiny => self.s = value,
        }
    }
    fn get(&self, field: &Field) -> &T {
        match field {
            Field::XCoolLooking => &self.x,
            Field::Musical => &self.m,
            Field::Aerodynamic => &self.a,
            Field::Shiny => &self.s,
        }
    }
}

impl Piece<u32> {
    fn get_total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl PieceRanges {
    fn get_distinct_combinations(&self) -> u64 {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .map(|attribute| attribute.as_ref().map(Range::count).unwrap_or(0))
            .product()
    }
}

impl FromStr for Piece<u32> {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or(ParseErr)?;

        let mut piece = Self::new(0);
        for attribute in s.split(',') {
            let (field, value) = attribute.split_once('=').ok_or(ParseErr)?;
            let field = field.chars().nth(0).ok_or(ParseErr)?.try_into()?;
            let value = value.parse().map_err(|_| ParseErr)?;

            piece.set(&field, value);
        }

        Ok(piece)
    }
}

enum Field {
    XCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl TryFrom<char> for Field {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::XCoolLooking),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => Err(ParseErr),
        }
    }
}

struct Step {
    cond: Option<(Field, Cond)>,
    outcome: Outcome,
}

impl Step {
    fn test_piece(&self, piece: &Piece<u32>) -> bool {
        if let Some((field, condition)) = &self.cond {
            condition.test(*piece.get(field))
        } else {
            true
        }
    }

    // (pass, fail)
    fn test_piece_ranges(&self, mut piece: PieceRanges) -> (PieceRanges, PieceRanges) {
        if let Some((field, condition)) = &self.cond {
            let pass = condition.test_range(piece.get(field));
            let fail = condition.test_range_neg(piece.get(field));

            let mut pass_piece = piece.clone();
            pass_piece.set(field, pass);
            piece.set(field, fail);
            (pass_piece, piece)
        } else {
            (piece, PieceRanges::new(None))
        }
    }
}

impl FromStr for Step {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            Some((condition, outcome)) => {
                let field = condition.chars().nth(0).ok_or(ParseErr)?.try_into()?;
                let condition = condition.get(1..).ok_or(ParseErr)?.parse()?;

                Ok(Step {
                    cond: Some((field, condition)),
                    outcome: outcome.parse()?,
                })
            }
            None => Ok(Step {
                cond: None,
                outcome: s.parse()?,
            }),
        }
    }
}

#[derive(Debug)]
enum Cond {
    Lt(u32),
    Gt(u32),
}

impl Cond {
    fn test(&self, other: u32) -> bool {
        match self {
            Cond::Lt(v) => other < *v,
            Cond::Gt(v) => other > *v,
        }
    }
    fn test_range(&self, other: &Option<Range>) -> Option<Range> {
        match self {
            Cond::Lt(v) => other.clone().and_then(|range| {
                if range.min >= *v {
                    None
                } else {
                    Some(Range {
                        min: range.min,
                        max: range.max.min(*v - 1),
                    })
                }
            }),
            Cond::Gt(v) => other.clone().and_then(|range| {
                if range.max <= *v {
                    None
                } else {
                    Some(Range {
                        min: range.min.max(*v + 1),
                        max: range.max,
                    })
                }
            }),
        }
    }
    fn test_range_neg(&self, other: &Option<Range>) -> Option<Range> {
        match self {
            Cond::Lt(v) => Cond::Gt(v - 1).test_range(other),
            Cond::Gt(v) => Cond::Lt(v + 1).test_range(other),
        }
    }
}

impl FromStr for Cond {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('>') => Ok(Self::Gt(s[1..].parse().map_err(|_| ParseErr)?)),
            Some('<') => Ok(Self::Lt(s[1..].parse().map_err(|_| ParseErr)?)),
            _ => Err(ParseErr),
        }
    }
}

enum Outcome {
    Accept,
    Reject,
    Move(String),
}

impl FromStr for Outcome {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Move(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Clone)]
struct Range {
    min: u32, // inclusive
    max: u32, // inclusive
}

impl Range {
    fn count(&self) -> u64 {
        u64::from(self.min.abs_diff(self.max)) + 1
    }
}

pub struct AocDay19 {
    rules: HashMap<String, Vec<Step>>,
    pieces: Vec<Piece<u32>>,
}
impl AocDay19 {
    fn count_possible(&self, mut piece: PieceRanges, workflow: String) -> u64 {
        let mut accepted_count: u64 = 0;
        let w = self.rules.get(&workflow).unwrap();
        for step in w {
            let (pass_piece, fail) = step.test_piece_ranges(piece);
            piece = fail;

            match &step.outcome {
                Outcome::Accept => {
                    accepted_count += pass_piece.get_distinct_combinations();
                }
                Outcome::Move(s) => {
                    accepted_count += self.count_possible(pass_piece, s.to_string())
                }
                Outcome::Reject => {}
            }
        }
        accepted_count
    }
}

impl AocDay<u32, u64> for AocDay19 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let rules = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|line| {
                let (name, rules) = line
                    .split_once('{')
                    .expect("failed to parse workflow: doesn't contain any rules");
                let rules = rules
                    .strip_suffix('}')
                    .expect("failed to parse workflow: doesn't contain any rules")
                    .split(',')
                    .map(|rule| rule.parse().expect("failed to parse rule"))
                    .collect();
                (name.to_string(), rules)
            })
            .collect();
        let pieces = lines
            .map(|line| line.parse().expect("failed to parse piece"))
            .collect();

        AocDay19 { rules, pieces }
    }
    fn part1(&self) -> u32 {
        self.pieces
            .iter()
            .map(|piece| {
                let mut workflow: String = "in".to_string();
                'outer: loop {
                    let w = self.rules.get(&workflow).expect("unknown workflow");
                    for step in w {
                        if step.test_piece(piece) {
                            match &step.outcome {
                                Outcome::Accept => return piece.get_total_rating(),
                                Outcome::Reject => return 0,
                                Outcome::Move(s) => {
                                    workflow = s.to_string();
                                    continue 'outer;
                                }
                            }
                        }
                    }
                    unreachable!("did not match any rules in the workflow");
                }
            })
            .sum()
    }
    fn part2(&self) -> u64 {
        self.count_possible(
            PieceRanges::new(Some(Range { min: 1, max: 4000 })),
            "in".to_string(),
        )
    }
}

#[cfg(test)]
mod day19tests {
    use super::*;

    const INPUT: &[&str] = &[
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013}",
    ];

    #[test]
    fn part1() {
        let day = AocDay19::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 19114);
    }

    #[test]
    fn part2() {
        let day = AocDay19::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 167409079868000);
    }
}
