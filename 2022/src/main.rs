use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() -> Result<()> {
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

    match puzzle_index {
        1 => run_day(day01::AocDay01::preprocessing(input))?,
        2 => run_day(day02::AocDay02::preprocessing(input))?,
        3 => run_day(day03::AocDay03::preprocessing(input))?,
        4 => run_day(day04::AocDay04::preprocessing(input))?,
        5 => run_day(day05::AocDay05::preprocessing(input))?,
        6 => run_day(day06::AocDay06::preprocessing(input))?,
        7 => run_day(day07::AocDay07::preprocessing(input))?,
        _ => unimplemented!("Unknown puzzle"),
    };

    Ok(())
}

pub trait AocDay<R1: Display, R2: Display> {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> R1;
    fn part2(&self) -> R2;
}

fn run_day<T: AocDay<R1, R2>, R1: Display, R2: Display>(puzzle: T) -> Result<()> {
    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());

    Ok(())
}
