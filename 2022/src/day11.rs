use std::str::FromStr;

use crate::AocDay;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Add(OperationValue),
    Multiply(OperationValue),
}

impl FromStr for Operation {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s.split_once(' ').ok_or(ParseErr)?;

        let arg: OperationValue = arg.parse()?;
        match op {
            "+" => Ok(Self::Add(arg)),
            "*" => Ok(Self::Multiply(arg)),
            _ => Err(ParseErr),
        }
    }
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Self::Add(value) => old + value.get(old),
            Self::Multiply(value) => old * value.get(old),
        }
    }
}

#[derive(Debug, Clone)]
enum OperationValue {
    Constant(i64),
    Old,
}

impl FromStr for OperationValue {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            _ => Ok(Self::Constant(s.parse().map_err(|_| ParseErr)?)),
        }
    }
}

impl OperationValue {
    fn get(&self, old: i64) -> i64 {
        match self {
            Self::Constant(v) => *v,
            Self::Old => old,
        }
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    inspected_count: usize,
    operation: Operation,
    divisible_by: i64,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

fn process_monkey<const N: i64>(
    monkeys: &mut [Monkey],
    monkey_index: usize,
    mult_divisible_by: i64,
) {
    let items = {
        let monkey = monkeys.get_mut(monkey_index).expect("out of bounds monkey");
        monkey.inspected_count += monkey.items.len();
        let items = monkey.items.to_vec();
        monkey.items.clear();

        items
    };
    items.iter().for_each(|item| {
        let monkey = monkeys.get(monkey_index).expect("out of bounds monkey");
        let worry_level = (monkey.operation.apply(*item) / N) % mult_divisible_by;
        let target_monkey = match worry_level % monkey.divisible_by == 0 {
            true => monkey.monkey_if_true,
            false => monkey.monkey_if_false,
        };

        monkeys
            .get_mut(target_monkey)
            .expect("out of bounds target monkey")
            .items
            .push(worry_level);
    });
}

fn run_monkey_round<const N: i64>(monkeys: &mut [Monkey], mult_divisible_by: i64) {
    for i in 0..monkeys.len() {
        process_monkey::<N>(monkeys, i, mult_divisible_by);
    }
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> usize {
    let maxs =
        monkeys
            .iter()
            .map(|monkey| monkey.inspected_count)
            .fold((None, None), |acc, count| match acc {
                (Some(x), _) if count > x => (Some(count), Some(x)),
                (Some(x), Some(y)) if count <= x && count > y => (Some(x), Some(count)),
                (None, None) => (Some(count), None),
                (Some(x), None) => (Some(x), Some(count)),
                _ => acc,
            });

    maxs.0.unwrap() * maxs.1.unwrap()
}

pub struct AocDay11 {
    monkeys: Vec<Monkey>,
    mult_divisible_by: i64,
}

impl AocDay<usize, usize> for AocDay11 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        // lines.array_chunks is still on nightly only :/
        // using itertools instead
        let monkeys: Vec<Monkey> = lines
            .chunks(7)
            .into_iter()
            .map(|mut iter| {
                let starting_items: Vec<i64> = iter.nth(1).unwrap()[18..]
                    .split(", ")
                    .map(|x| x.parse().expect("starting items must be integers"))
                    .collect();
                let operation: Operation = iter.next().unwrap()[23..]
                    .parse()
                    .expect("invalid operation");
                let divisible_by: i64 = iter.next().unwrap()[21..]
                    .parse()
                    .expect("divisible by must be a number");
                let monkey_if_true: usize = iter.next().unwrap()[29..]
                    .parse()
                    .expect("target monkey must be a number");
                let monkey_if_false: usize = iter.next().unwrap()[30..]
                    .parse()
                    .expect("target monkey must be a number");
                Monkey {
                    items: starting_items,
                    inspected_count: 0,
                    operation,
                    divisible_by,
                    monkey_if_true,
                    monkey_if_false,
                }
            })
            .collect();

        let mult_divisible_by = monkeys.iter().map(|m| m.divisible_by).product();

        AocDay11 {
            monkeys,
            mult_divisible_by,
        }
    }
    fn part1(&self) -> usize {
        let mut monkeys: Vec<Monkey> = self.monkeys.to_vec();
        for _ in 0..20 {
            run_monkey_round::<3>(&mut monkeys, self.mult_divisible_by);
        }

        calculate_monkey_business(&monkeys)
    }
    fn part2(&self) -> usize {
        let mut monkeys: Vec<Monkey> = self.monkeys.to_vec();
        for _ in 0..10000 {
            run_monkey_round::<1>(&mut monkeys, self.mult_divisible_by);
        }

        calculate_monkey_business(&monkeys)
    }
}

#[cfg(test)]
mod day11tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day11_example.txt");

    #[test]
    fn part1() {
        let day = AocDay11::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(day.part1(), 10605);
    }

    #[test]
    fn part2() {
        let day = AocDay11::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(day.part2(), 2713310158);
    }
}
