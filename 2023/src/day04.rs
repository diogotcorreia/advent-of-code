use std::str::FromStr;

use crate::AocDay;

#[derive(Clone)]
struct Card {
    winning: Vec<u32>,
    own: Vec<u32>,
}

impl Card {
    fn win_count(&self) -> usize {
        self.winning
            .iter()
            .filter(|wv| self.own.contains(wv))
            .count()
    }
}

impl FromStr for Card {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.split_once(": ").ok_or(ParseErr)?;

        let (winning, own) = numbers.split_once(" | ").ok_or(ParseErr)?;

        Ok(Card {
            winning: winning
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().map_err(|_| ParseErr))
                .collect::<Result<_, _>>()?,
            own: own
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().map_err(|_| ParseErr))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay04 {
    cards: Vec<Card>,
}

impl AocDay<u32, u32> for AocDay04 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let cards = lines
            .map(|line| line.parse().expect("failed to parse card"))
            .collect();

        AocDay04 { cards }
    }
    fn part1(&self) -> u32 {
        self.cards
            .iter()
            .map(|card| match card.win_count() {
                0 => 0,
                win_count => 1 << (win_count - 1),
            })
            .sum()
    }
    fn part2(&self) -> u32 {
        let mut card_count = vec![0u32; self.cards.len()];

        for i in (0..self.cards.len()).rev() {
            let card = self.cards.get(i).expect("getting unknown card");
            let new_count: u32 = card_count[(i + 1)..(card.win_count() + i + 1)].iter().sum();
            *card_count.get_mut(i).expect("card_count out of bounds") = new_count + 1;
        }

        card_count.iter().sum()
    }
    // Naive approach :(
    // Takes 13s to run in debug mode
    // fn part2_naive(&self) -> i32 {
    //     let mut pile = self.cards.clone();
    //     let mut i = 0;

    //     while i < pile.len() {
    //         let card = pile.get(i).unwrap();
    //         let card_id = card.id;
    //         let matches = card.matches();

    //         for i in 0..matches {
    //             pile.push(self.cards.get((card_id + i) as usize).unwrap().clone());
    //         }
    //         i += 1;
    //     }

    //     pile.len() as i32
    // }
}

#[cfg(test)]
mod day04tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn part1() {
        let day = AocDay04::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 13);
    }

    #[test]
    fn part2() {
        let day = AocDay04::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 30);
    }
}
