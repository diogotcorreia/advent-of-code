#![feature(iter_chain)]
#![feature(let_chains)]
#![feature(array_try_map)]
use aoc_common::{
    bootstrap::{get_part_number, get_puzzle_index, get_puzzle_input, run_day, Error},
    AocDay,
};

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
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() -> Result<(), Error> {
    let puzzle_index = get_puzzle_index()?;
    let part_number = get_part_number()?;

    let input = get_puzzle_input(puzzle_index)?;

    match puzzle_index {
        1 => run_day(day01::AocDay01::preprocessing(input)?, part_number),
        2 => run_day(day02::AocDay02::preprocessing(input)?, part_number),
        3 => run_day(day03::AocDay03::preprocessing(input)?, part_number),
        4 => run_day(day04::AocDay04::preprocessing(input)?, part_number),
        5 => run_day(day05::AocDay05::preprocessing(input)?, part_number),
        6 => run_day(day06::AocDay06::preprocessing(input)?, part_number),
        7 => run_day(day07::AocDay07::preprocessing(input)?, part_number),
        8 => run_day(day08::AocDay08::preprocessing(input)?, part_number),
        9 => run_day(day09::AocDay09::preprocessing(input)?, part_number),
        10 => run_day(day10::AocDay10::preprocessing(input)?, part_number),
        11 => run_day(day11::AocDay11::preprocessing(input)?, part_number),
        12 => run_day(day12::AocDay12::preprocessing(input)?, part_number),
        13 => run_day(day13::AocDay13::preprocessing(input)?, part_number),
        14 => run_day(day14::AocDay14::preprocessing(input)?, part_number),
        15 => run_day(day15::AocDay15::preprocessing(input)?, part_number),
        16 => run_day(day16::AocDay16::preprocessing(input)?, part_number),
        17 => run_day(day17::AocDay17::preprocessing(input)?, part_number),
        18 => run_day(day18::AocDay18::preprocessing(input)?, part_number),
        19 => run_day(day19::AocDay19::preprocessing(input)?, part_number),
        20 => run_day(day20::AocDay20::preprocessing(input)?, part_number),
        // 21 => run_day(day21::AocDay21::preprocessing(input)?, part_number),
        // 22 => run_day(day22::AocDay22::preprocessing(input)?, part_number),
        // 23 => run_day(day23::AocDay23::preprocessing(input)?, part_number),
        // 24 => run_day(day24::AocDay24::preprocessing(input)?, part_number),
        // 25 => run_day(day25::AocDay25::preprocessing(input)?, part_number),
        _ => unimplemented!("Unknown puzzle"),
    };

    Ok(())
}
