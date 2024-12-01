use std::fmt::Display;

pub mod bootstrap;

pub trait AocDay<R1: Display, R2: Display> {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self;
    fn part1(&self) -> R1;
    fn part2(&self) -> R2;
}
