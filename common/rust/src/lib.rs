use std::{fmt::Display, num::ParseIntError};

pub mod bootstrap;
pub mod navigation;
pub mod parsing;

#[derive(Debug)]
pub enum DayError {
    NumParseErr(ParseIntError),
    GenericParseErr(&'static str),
}

impl From<ParseIntError> for DayError {
    fn from(value: ParseIntError) -> Self {
        Self::NumParseErr(value)
    }
}

pub trait AocDay<R1: Display, R2: Display> {
    fn preprocessing_tests(lines: &[&str]) -> Result<Self, DayError>
    where
        Self: std::marker::Sized,
    {
        Self::preprocessing(lines.iter().map(|x| String::from(*x)))
    }
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError>
    where
        Self: std::marker::Sized;
    fn part1(&self) -> R1;
    fn part2(&self) -> R2;
}
