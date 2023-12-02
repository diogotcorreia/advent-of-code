use std::str::FromStr;

use crate::AocDay;

struct Game {
    id: u32,
    subgames: Vec<SubGame>,
}

impl FromStr for Game {
    type Err = ParseErr;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, subgames) = line.split_once(": ").ok_or(ParseErr)?;
        let id: u32 = id
            .split(' ')
            .last()
            .ok_or(ParseErr)?
            .parse()
            .map_err(|_| ParseErr)?;
        let subgames = subgames
            .split("; ")
            .map(|subgame_str| subgame_str.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, subgames })
    }
}

struct SubGame {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for SubGame {
    type Err = ParseErr;
    fn from_str(subgame_str: &str) -> Result<Self, Self::Err> {
        let mut subgame = SubGame::new();
        subgame_str.split(", ").try_for_each(|color_str| {
            let (value, color_name) = color_str.split_once(' ').ok_or(ParseErr)?;
            let value: u32 = value.parse().map_err(|_| ParseErr)?;
            match color_name {
                "red" => subgame.red = value,
                "green" => subgame.green = value,
                "blue" => subgame.blue = value,
                _ => unreachable!("unknown color"),
            }
            Ok(())
        })?;
        Ok(subgame)
    }
}

impl SubGame {
    fn new() -> SubGame {
        SubGame {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay02 {
    games: Vec<Game>,
}

impl AocDay<u32, u32> for AocDay02 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let games = lines
            .map(|line| line.parse().expect("failed to parse game"))
            .collect();

        AocDay02 { games }
    }
    fn part1(&self) -> u32 {
        self.games
            .iter()
            .filter(|game| {
                game.subgames
                    .iter()
                    .all(|count| count.red <= 12 && count.green <= 13 && count.blue <= 14)
            })
            .map(|game| game.id)
            .sum()
    }
    fn part2(&self) -> u32 {
        self.games
            .iter()
            .map(|game| {
                game.subgames
                    .iter()
                    .fold(SubGame::new(), |acc, count| SubGame {
                        red: acc.red.max(count.red),
                        green: acc.green.max(count.green),
                        blue: acc.blue.max(count.blue),
                    })
                    .get_power()
            })
            .sum()
    }
}

#[cfg(test)]
mod day02tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn part1() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 8);
    }

    #[test]
    fn part2() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 2286);
    }
}
