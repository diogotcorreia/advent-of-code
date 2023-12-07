use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::AocDay;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Label {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl Label {
    fn cmp_part2(&self, other: &Label) -> Ordering {
        match (self, other) {
            (Label::J, Label::J) => Ordering::Equal,
            (Label::J, _) => Ordering::Greater,
            (_, Label::J) => Ordering::Less,
            _ => self.cmp(other),
        }
    }
}

impl TryFrom<char> for Label {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'K' => Ok(Self::K),
            'Q' => Ok(Self::Q),
            'J' => Ok(Self::J),
            'T' => Ok(Self::T),
            '9' => Ok(Self::N9),
            '8' => Ok(Self::N8),
            '7' => Ok(Self::N7),
            '6' => Ok(Self::N6),
            '5' => Ok(Self::N5),
            '4' => Ok(Self::N4),
            '3' => Ok(Self::N3),
            '2' => Ok(Self::N2),
            _ => Err(ParseErr),
        }
    }
}

fn by_type<const PART2: bool>(cards: &[Label]) -> HashMap<Label, usize> {
    let mut map = HashMap::new();

    let cards = if PART2 {
        let mut cards = cards.to_vec();
        cards.sort_by(|a, b| a.cmp_part2(b));
        cards
    } else {
        cards.to_vec()
    };

    for card in cards {
        if PART2 && card == Label::J && !map.is_empty() {
            // account for JJJJJ with !map.is_empty
            map.values_mut().for_each(|v| *v += 1);
        } else {
            *map.entry(card.clone()).or_default() += 1;
        }
    }

    map
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Label>, // [Label; 5],
    bid: u64,
}

impl Hand {
    fn get_hand_type<const PART2: bool>(&self) -> HandType {
        let types = by_type::<PART2>(&self.cards);

        if types.len() == 1 {
            HandType::FiveKind
        } else if types.len() == 2 {
            if types.values().filter(|&&n| n >= 4).count() > 0 {
                HandType::FourKind
            } else {
                HandType::FullHouse
            }
        } else if types.len() == 3 {
            if types.values().filter(|&&n| n >= 3).count() > 0 {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        } else if types.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn cmp<const PART2: bool>(&self, other: &Self) -> Ordering {
        match self
            .get_hand_type::<PART2>()
            .cmp(&other.get_hand_type::<PART2>())
        {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let cmp = if PART2 {
                        self_card.cmp_part2(other_card)
                    } else {
                        self_card.cmp(other_card)
                    };
                    match cmp {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        }
    }
}

impl FromStr for Hand {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(ParseErr)?;

        Ok(Self {
            cards: cards
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<_, _>>()?,
            bid: bid.parse().map_err(|_| ParseErr)?,
        })
    }
}

#[derive(Debug)]
struct ParseErr;

fn get_hands_winnings(sorted_hands: &[Hand]) -> u64 {
    sorted_hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum()
}

pub struct AocDay07 {
    hands: Vec<Hand>,
}

impl AocDay<u64, u64> for AocDay07 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let hands = lines
            .map(|line| line.parse().expect("unable to parse hand"))
            .collect();

        AocDay07 { hands }
    }
    fn part1(&self) -> u64 {
        let mut sorted_hands = self.hands.clone();
        sorted_hands.sort_by(|a, b| a.cmp::<false>(b));

        get_hands_winnings(&sorted_hands)
    }
    fn part2(&self) -> u64 {
        let mut sorted_hands = self.hands.clone();
        sorted_hands.sort_by(|a, b| a.cmp::<true>(b));

        get_hands_winnings(&sorted_hands)
    }
}

#[cfg(test)]
mod day07tests {
    use super::*;

    const INPUT: &[&str] = &[
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    #[test]
    fn part1() {
        let day = AocDay07::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 6440);
    }

    #[test]
    fn part2() {
        let day = AocDay07::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 5905);
    }

    #[test]
    fn part2_hands() -> Result<(), ParseErr> {
        let hand: Hand = "JJJJJ 0".parse()?;
        assert_eq!(hand.get_hand_type::<true>(), HandType::FiveKind);

        Ok(())
    }
}
