use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pair(char, char);

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let polymer: Vec<char> = input.trim().chars().collect();

    let mut pairs = HashMap::new();

    std::io::stdin().lock().lines().for_each(|line| {
        if let Some((pair, result)) = line
            .expect("failed to read stdin line")
            .trim()
            .split_once(" -> ")
        {
            let mut chars = pair.chars();
            pairs.insert(
                Pair(chars.next().unwrap(), chars.next().unwrap()),
                result.chars().next().unwrap(),
            );
        }
    });

    let mut letter_count = HashMap::new();
    let mut pair_count = polymer_to_pair_map(&polymer, &mut letter_count);

    for _ in 0..10 {
        pair_count = execute_step(&pair_count, &mut letter_count, &pairs);
    }

    println!(
        "Part 1: {}",
        letter_count.values().max().unwrap() - letter_count.values().min().unwrap()
    );

    for _ in 10..40 {
        pair_count = execute_step(&pair_count, &mut letter_count, &pairs);
    }

    println!(
        "Part 2: {}",
        letter_count.values().max().unwrap() - letter_count.values().min().unwrap()
    );
}

fn execute_step(
    pair_count: &HashMap<Pair, i64>,
    letter_count: &mut HashMap<char, i64>,
    pairs_map: &HashMap<Pair, char>,
) -> HashMap<Pair, i64> {
    let mut new_pair_count = HashMap::new();

    for (pair, count) in pair_count {
        match pairs_map.get(pair) {
            Some(new_char) => {
                new_pair_count
                    .entry(Pair(pair.0, *new_char))
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
                new_pair_count
                    .entry(Pair(*new_char, pair.1))
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
                letter_count
                    .entry(*new_char)
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            }
            None => {
                new_pair_count
                    .entry(pair.clone())
                    .and_modify(|c| *c += *count)
                    .or_insert(*count);
            }
        };
    }

    new_pair_count
}

fn polymer_to_pair_map(
    polymer: &Vec<char>,
    letter_count: &mut HashMap<char, i64>,
) -> HashMap<Pair, i64> {
    let mut new_pair_count = HashMap::new();

    for i in 0..polymer.len() - 1 {
        let pair = Pair(*polymer.get(i).unwrap(), *polymer.get(i + 1).unwrap());
        new_pair_count
            .entry(pair)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        letter_count
            .entry(pair.0)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    letter_count
        .entry(*polymer.last().unwrap())
        .and_modify(|c| *c += 1)
        .or_insert(1);

    new_pair_count
}
