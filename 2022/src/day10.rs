use std::str::FromStr;

use crate::AocDay;

enum OpCode {
    NoOp,
    Addx(i32),
}

impl FromStr for OpCode {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        match split.next().ok_or(ParseErr)? {
            "noop" => Ok(Self::NoOp),
            "addx" => Ok(Self::Addx(
                split
                    .next()
                    .ok_or(ParseErr)?
                    .parse()
                    .map_err(|_| ParseErr)?,
            )),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay10 {
    strength: i32,
    image: String,
}

fn is_strength_cycle(cycle: usize) -> bool {
    (cycle + 20) % 40 == 0 && cycle <= 230
}

fn get_display_char(cycle: usize, register: i32) -> char {
    let pixel_pos = (cycle - 1) % 40;

    if (pixel_pos as i32 - register).abs() <= 1 {
        '#'
    } else {
        ' '
    }
}

impl AocDay<i32, String> for AocDay10 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let (_, strength, _, image) = lines
            .map(|x| x.parse::<OpCode>().expect("invalid input"))
            .fold(
                (1i32, 0i32, 1usize, Vec::new()),
                |(register, mut strength, cycle, mut image), opcode| {
                    // TODO
                    if is_strength_cycle(cycle) {
                        strength += cycle as i32 * register;
                    }
                    image.push(get_display_char(cycle, register));
                    match opcode {
                        OpCode::NoOp => (register, strength, cycle + 1, image),
                        OpCode::Addx(x) => {
                            if is_strength_cycle(cycle + 1) {
                                strength += (cycle + 1) as i32 * register;
                            }
                            image.push(get_display_char(cycle + 1, register));
                            (register + x, strength, cycle + 2, image)
                        }
                    }
                },
            );

        let image = image.chunks(40).take(6).fold(String::new(), |acc, x| {
            acc + "\n" + &String::from_iter(x.iter())
        });
        AocDay10 { strength, image }
    }
    fn part1(&self) -> i32 {
        self.strength
    }
    fn part2(&self) -> String {
        self.image.clone()
    }
}

#[cfg(test)]
mod day10tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day10_example.txt");

    #[test]
    fn part1() {
        let day = AocDay10::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(day.part1(), 13140);
    }

    #[test]
    fn part2() {
        let day = AocDay10::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(
            day.part2(),
            concat!(
                "\n##  ##  ##  ##  ##  ##  ##  ##  ##  ##  ",
                "\n###   ###   ###   ###   ###   ###   ### ",
                "\n####    ####    ####    ####    ####    ",
                "\n#####     #####     #####     #####     ",
                "\n######      ######      ######      ####",
                "\n#######       #######       #######     "
            )
        );
    }
}
