use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::AocDay;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Subtract,
    Multipy,
    Divide,
}

impl Operator {
    fn apply(&self, value1: i64, value2: i64) -> i64 {
        match self {
            Self::Add => value1 + value2,
            Self::Subtract => value1 - value2,
            Self::Multipy => value1 * value2,
            Self::Divide => value1 / value2,
        }
    }
}

impl FromStr for Operator {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multipy),
            "/" => Ok(Self::Divide),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Value(i64),
    Recipe(Operator, String, String),
}

impl FromStr for Monkey {
    type Err = ParseErr;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.parse::<i64>() {
            Ok(v) => Ok(Monkey::Value(v)),
            Err(_) => {
                let mut it = value.split_ascii_whitespace();
                let monkey1 = it.next().ok_or(ParseErr)?;
                let op = it.next().ok_or(ParseErr)?.parse()?;
                let monkey2 = it.next().ok_or(ParseErr)?;

                Ok(Monkey::Recipe(op, monkey1.to_string(), monkey2.to_string()))
            }
        }
    }
}

#[derive(Debug)]
struct ParseErr;

fn calculate_monkey_value(monkeys: &mut HashMap<String, Monkey>, monkey: &str) -> i64 {
    match monkeys.get(monkey) {
        Some(Monkey::Value(v)) => *v,
        Some(Monkey::Recipe(op, m1, m2)) => {
            let op = op.clone();
            let m1 = m1.clone();
            let m2 = m2.clone();

            let v1 = calculate_monkey_value(monkeys, &m1);
            let v2 = calculate_monkey_value(monkeys, &m2);
            let result = op.apply(v1, v2);

            monkeys.insert(monkey.to_string(), Monkey::Value(result));

            result
        }
        None => unreachable!(),
    }
}

fn compare_monkey(monkeys: &mut HashMap<String, Monkey>, human_value: i64) -> i64 {
    if let Monkey::Value(v) = monkeys.get_mut("humn").unwrap() {
        *v = human_value;
    }

    calculate_monkey_value(monkeys, "root")
}

pub struct AocDay21 {
    monkeys: HashMap<String, Monkey>,
}

impl AocDay<i64, i64> for AocDay21 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let monkeys = lines
            .map(|line| {
                let (name, value) = line.split_once(": ").expect("invalid monkey");

                (name.to_string(), value.parse().expect("invalid monkey"))
            })
            .collect();

        AocDay21 { monkeys }
    }
    fn part1(&self) -> i64 {
        let mut monkeys = self.monkeys.clone();
        calculate_monkey_value(&mut monkeys, "root")
    }
    fn part2(&self) -> i64 {
        let mut monkeys = self.monkeys.clone();
        if let Monkey::Recipe(op, _, _) = monkeys.get_mut("root").unwrap() {
            *op = Operator::Subtract;
        }

        // some numbers that are big enough to get to an answer but don't overflow
        let (mut low, mut high) = (-(1 << 50), 1 << 50);
        let reversed = compare_monkey(&mut monkeys.clone(), low) > 0;

        while low < high {
            let m = low + (high - low) / 2;

            let comparison_result = compare_monkey(&mut monkeys.clone(), m);

            match (reversed, comparison_result.cmp(&0)) {
                (_, Ordering::Equal) => high = m,
                (false, Ordering::Less) | (true, Ordering::Greater) => {
                    // target value is higher than m
                    low = m + 1;
                }
                (true, Ordering::Less) | (false, Ordering::Greater) => {
                    // target value is lower than m
                    high = m - 1;
                }
            }
        }

        low
    }
}

#[cfg(test)]
mod day21tests {
    use super::*;

    const INPUT: &[&str] = &[
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32",
    ];

    #[test]
    fn part1() {
        let day = AocDay21::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 152);
    }

    #[test]
    fn part2() {
        let day = AocDay21::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 301);
    }
}
