use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

use crate::AocDay;

#[derive(Debug)]
pub enum Error {
    IoErr(std::io::Error),
    NoDayProvided,
    DayNotNumber(ParseIntError),
    PartNotNumber(ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoErr(value)
    }
}

pub fn run_day<T: AocDay<R1, R2>, R1: Display, R2: Display>(puzzle: T, part_number: Option<usize>) {
    if part_number.unwrap_or(1) == 1 {
        println!("Part 1: {}", puzzle.part1());
    }
    if part_number.unwrap_or(2) == 2 {
        println!("Part 2: {}", puzzle.part2());
    }
}

pub fn get_puzzle_index() -> Result<usize, Error> {
    env::args()
        .nth(1)
        .ok_or(Error::NoDayProvided)?
        .parse()
        .map_err(Error::DayNotNumber)
}

pub fn get_part_number() -> Result<Option<usize>, Error> {
    env::args()
        .nth(2)
        .map(|x| x.parse().map_err(Error::PartNotNumber))
        .transpose()
        .map(|opt| opt.filter(|&x| x == 1 || x == 2))
}

pub fn get_puzzle_input(puzzle_index: usize) -> Result<impl Iterator<Item = String>, Error> {
    let input_file = format!("inputs/day{:0>2}.txt", puzzle_index);
    Ok(BufReader::new(File::open(input_file)?)
        .lines()
        .map(|r| r.expect("I/O error while reading input")))
}
