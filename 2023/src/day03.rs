use std::collections::HashMap;

use crate::AocDay;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        let x: i32 = x.try_into().expect("x does not fit in 32-bit");
        let y: i32 = y.try_into().expect("y does not fit in 32-bit");
        Self(x, y)
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    length: u32,
    pos: Pos,
}

impl Number {
    fn get_adjacent_pos(&self) -> Vec<Pos> {
        let len: usize = self
            .length
            .try_into()
            .expect("length does not fit in usize");
        let mut pos = Vec::with_capacity(6 + len * 2);

        let (x, y) = (self.pos.0, self.pos.1);
        let len = self.length;

        pos.push(Pos(x - 1, y));
        pos.push(Pos(x - 1, y - 1));
        pos.push(Pos(x - 1, y + 1));

        for i in 0..len {
            pos.push(Pos(x + i as i32, y + 1));
            pos.push(Pos(x + i as i32, y - 1));
        }

        pos.push(Pos(x + len as i32, y));
        pos.push(Pos(x + len as i32, y - 1));
        pos.push(Pos(x + len as i32, y + 1));

        pos
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay03 {
    symbols: HashMap<Pos, char>,
    numbers: Vec<Number>,
}

impl AocDay<u32, u32> for AocDay03 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut symbols = HashMap::new();
        let mut numbers = vec![];

        for (y, line) in lines.enumerate() {
            let mut curr_number: u32 = 0;
            let mut curr_number_len: u32 = 0;
            let mut curr_number_pos = None;
            for (x, char) in line.chars().enumerate() {
                if char.is_ascii_digit() {
                    curr_number *= 10;
                    curr_number += char.to_digit(10).unwrap();
                    curr_number_len += 1;
                    if curr_number_pos.is_none() {
                        curr_number_pos = Some(Pos::new(x, y));
                    }
                    continue;
                }
                if let Some(pos) = curr_number_pos {
                    numbers.push(Number {
                        value: curr_number,
                        length: curr_number_len,
                        pos,
                    });
                    curr_number = 0;
                    curr_number_len = 0;
                    curr_number_pos = None;
                }
                match char {
                    '*' | '#' | '+' | '$' | '@' | '=' | '%' | '-' | '&' | '/' => {
                        symbols.insert(Pos::new(x, y), char);
                    }
                    '.' => {}
                    _ => unreachable!("unknown char in map: {}", char),
                }
            }
            if let Some(pos) = curr_number_pos {
                numbers.push(Number {
                    value: curr_number,
                    length: curr_number_len,
                    pos,
                });
            }
        }

        AocDay03 { symbols, numbers }
    }
    fn part1(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| {
                number
                    .get_adjacent_pos()
                    .iter()
                    .any(|pos| self.symbols.get(pos).is_some())
            })
            .map(|number| number.value)
            .sum()
    }
    fn part2(&self) -> u32 {
        let mut map: HashMap<Pos, Vec<u32>> = HashMap::new();
        self.numbers.iter().for_each(|number| {
            number
                .get_adjacent_pos()
                .iter()
                .filter(|pos| self.symbols.get(pos) == Some(&'*'))
                .for_each(|gear_pos| {
                    map.entry(gear_pos.clone()).or_default().push(number.value);
                })
        });

        map.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v.iter().product::<u32>())
            .sum()
    }
}

#[cfg(test)]
mod day03tests {
    use super::*;

    const INPUT: &[&str] = &[
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    #[test]
    fn part1() {
        let day = AocDay03::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 4361);
    }

    #[test]
    fn part2() {
        let day = AocDay03::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 467835);
    }
}
