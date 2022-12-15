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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() -> Result<()> {
    let puzzle_index: usize = env::args()
        .nth(1)
        .expect("Please provide a the day number")
        .parse()
        .expect("Puzzle day must be a number");
    let part_number: Option<usize> = env::args()
        .nth(2)
        .map(|x| x.parse().expect("Part must be a number"))
        .filter(|&x| x == 1 || x == 2);

    let input_file = format!("inputs/day{:0>2}.txt", puzzle_index);
    let input = BufReader::new(File::open(input_file)?)
        .lines()
        .map(|r| r.expect("I/O error while reading input"));

    match puzzle_index {
        1 => run_day(day01::AocDay01::preprocessing(input), part_number)?,
        2 => run_day(day02::AocDay02::preprocessing(input), part_number)?,
        3 => run_day(day03::AocDay03::preprocessing(input), part_number)?,
        4 => run_day(day04::AocDay04::preprocessing(input), part_number)?,
        5 => run_day(day05::AocDay05::preprocessing(input), part_number)?,
        6 => run_day(day06::AocDay06::preprocessing(input), part_number)?,
        7 => run_day(day07::AocDay07::preprocessing(input), part_number)?,
        8 => run_day(day08::AocDay08::preprocessing(input), part_number)?,
        9 => run_day(day09::AocDay09::preprocessing(input), part_number)?,
        10 => run_day(day10::AocDay10::preprocessing(input), part_number)?,
        11 => run_day(day11::AocDay11::preprocessing(input), part_number)?,
        12 => run_day(day12::AocDay12::preprocessing(input), part_number)?,
        13 => run_day(day13::AocDay13::preprocessing(input), part_number)?,
        14 => run_day(day14::AocDay14::preprocessing(input), part_number)?,
        15 => run_day(day15::AocDay15::preprocessing(input), part_number)?,
        _ => unimplemented!("Unknown puzzle"),
    };

    Ok(())
}

pub trait AocDay<R1: Display, R2: Display> {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> R1;
    fn part2(&self) -> R2;
}

fn run_day<T: AocDay<R1, R2>, R1: Display, R2: Display>(
    puzzle: T,
    part_number: Option<usize>,
) -> Result<()> {
    if part_number.unwrap_or(1) == 1 {
        println!("Part 1: {}", puzzle.part1());
    }
    if part_number.unwrap_or(2) == 2 {
        println!("Part 2: {}", puzzle.part2());
    }

    Ok(())
}
