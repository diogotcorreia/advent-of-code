use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day01;

fn main() -> std::io::Result<()> {
    let days = vec![day01::AocDay01::preprocessing];

    let puzzle_index: usize = env::args()
        .skip(1)
        .next()
        .expect("Please provide a the day number")
        .parse()
        .expect("Puzzle day must be a number");

    let input_file = format!("inputs/day{:0>2}.txt", puzzle_index);
    let input = BufReader::new(File::open(input_file)?)
        .lines()
        .map(|r| r.expect("I/O error while reading input"));

    let puzzle = days.get(puzzle_index - 1).unwrap()(input);
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());

    Ok(())
}

pub trait AocDay {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> i64;
    fn part2(&self) -> i64;
}
