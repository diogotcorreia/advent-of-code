use crate::AocDay;

pub struct AocDay02 {
    guide: Vec<(Play, Play)>,
}

#[derive(Clone, Copy, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Play {
    fn from_str(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!("Unknown play type"),
        }
    }

    fn get_play_points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_outcome_points(&self, opponent: &Self) -> i32 {
        if self == opponent {
            return 3;
        }
        if (self == &Self::Rock && opponent == &Self::Scissors)
            || (self == &Self::Scissors && opponent == &Self::Paper)
            || (self == &Self::Paper && opponent == &Self::Rock)
        {
            return 6;
        }
        0
    }
}

impl From<Play> for Outcome {
    fn from(play: Play) -> Self {
        match play {
            Play::Rock => Self::Lose,
            Play::Paper => Self::Draw,
            Play::Scissors => Self::Win,
        }
    }
}

impl Outcome {
    fn get_play_points(&self, opponent: &Play) -> i32 {
        match self {
            Self::Lose => match opponent {
                Play::Rock => &Play::Scissors,
                Play::Paper => &Play::Rock,
                Play::Scissors => &Play::Paper,
            },
            Self::Draw => opponent,
            Self::Win => match opponent {
                Play::Rock => &Play::Paper,
                Play::Paper => &Play::Scissors,
                Play::Scissors => &Play::Rock,
            },
        }
        .get_play_points()
    }

    fn get_outcome_points(&self) -> i32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl AocDay<i32, i32> for AocDay02 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut guide: Vec<(Play, Play)> = Vec::new();

        for line in lines {
            let guide_entry: Vec<Play> = line
                .split_whitespace()
                .map(Play::from_str)
                .take(2)
                .collect();

            let guide_entry: (Play, Play) = (
                *guide_entry.get(0).expect("line must have first play"),
                *guide_entry.get(1).expect("line must have second play"),
            );

            guide.push(guide_entry);
        }

        AocDay02 { guide }
    }
    fn part1(&self) -> i32 {
        self.guide
            .iter()
            .map(|(opponent, player)| {
                player.get_play_points() + player.get_outcome_points(opponent)
            })
            .sum()
    }
    fn part2(&self) -> i32 {
        self.guide
            .iter()
            .map(|(opponent, player)| {
                let player: Outcome = (*player).into();
                player.get_play_points(opponent) + player.get_outcome_points()
            })
            .sum()
    }
}

#[cfg(test)]
mod day02tests {
    use super::*;

    const INPUT: &[&str] = &["A Y", "B X", "C Z"];

    #[test]
    fn part1() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 15);
    }

    #[test]
    fn part2() {
        let day = AocDay02::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 12);
    }
}
