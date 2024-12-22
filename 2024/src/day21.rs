use std::iter;

use aoc_common::{navigation::Vec2D, AocDay, DayError};
use aoc_common_macros::TryFromChar;
use itertools::Itertools;
use memoize::memoize;

type Pos = Vec2D<i8>;

trait KeyPad
where
    Self: Sized,
{
    const GAP_POS: Pos;
    const START_KEY: Self;

    fn key_to_pos(&self) -> Pos;

    /// Find all shortest paths from key `from` to key `to`.
    /// The returned vector will have length of 1 or 2.
    fn shortest_path(from: &Self, to: &Self) -> Vec<Vec<DpadKey>> {
        let from = from.key_to_pos();
        let to = to.key_to_pos();

        let diff_x = to.x - from.x;
        let diff_y = to.y - from.y;

        let mut movements = Vec::with_capacity(2);

        let horiz_key = if diff_x < 0 {
            DpadKey::Left
        } else {
            DpadKey::Right
        };
        let vert_key = if diff_y < 0 {
            DpadKey::Up
        } else {
            DpadKey::Down
        };

        // Only test this branch if keys are not in a straight line.
        // Make sure to not go over the gap on the keypad.
        if (diff_x != 0 && diff_y != 0) && (Self::GAP_POS.x != to.x || Self::GAP_POS.y != from.y) {
            // move horizontally first
            let mut movement = Vec::with_capacity(6);
            (0..diff_x.abs()).for_each(|_| movement.push(horiz_key.clone()));
            (0..diff_y.abs()).for_each(|_| movement.push(vert_key.clone()));
            movement.push(DpadKey::Activate);
            movements.push(movement);
        }
        if Self::GAP_POS.x != from.x || Self::GAP_POS.y != to.y {
            // move vertically first
            let mut movement = Vec::with_capacity(6);
            (0..diff_y.abs()).for_each(|_| movement.push(vert_key.clone()));
            (0..diff_x.abs()).for_each(|_| movement.push(horiz_key.clone()));
            movement.push(DpadKey::Activate);
            movements.push(movement);
        }

        movements
    }

    fn process_keypad(keys: Vec<Self>, depth: usize) -> usize {
        iter::once(&Self::START_KEY)
            .chain(keys.iter())
            .tuple_windows()
            .map(|(from, to)| Self::shortest_path(from, to))
            .map(|paths| {
                paths
                    .into_iter()
                    .map(|path| process_dpad(path, depth - 1))
                    .min()
                    .unwrap_or(0)
            })
            .sum()
    }
}

#[memoize]
fn process_dpad(keys: Vec<DpadKey>, depth: usize) -> usize {
    if depth == 0 {
        return keys.len();
    }

    DpadKey::process_keypad(keys, depth)
}

#[derive(Debug, Clone, PartialEq, Eq, TryFromChar)]
enum NumpadKey {
    #[char_repr = '0']
    Zero,
    #[char_repr = '1']
    One,
    #[char_repr = '2']
    Two,
    #[char_repr = '3']
    Three,
    #[char_repr = '4']
    Four,
    #[char_repr = '5']
    Five,
    #[char_repr = '6']
    Six,
    #[char_repr = '7']
    Seven,
    #[char_repr = '8']
    Eight,
    #[char_repr = '9']
    Nine,
    #[char_repr = 'A']
    Activate,
}

impl KeyPad for NumpadKey {
    const GAP_POS: Pos = Pos { x: 0, y: 3 };
    const START_KEY: Self = Self::Activate;

    fn key_to_pos(&self) -> Pos {
        match self {
            Self::Zero => Pos::new(1, 3),
            Self::One => Pos::new(0, 2),
            Self::Two => Pos::new(1, 2),
            Self::Three => Pos::new(2, 2),
            Self::Four => Pos::new(0, 1),
            Self::Five => Pos::new(1, 1),
            Self::Six => Pos::new(2, 1),
            Self::Seven => Pos::new(0, 0),
            Self::Eight => Pos::new(1, 0),
            Self::Nine => Pos::new(2, 0),
            Self::Activate => Pos::new(2, 3),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DpadKey {
    Left,
    Right,
    Up,
    Down,
    Activate,
}

impl KeyPad for DpadKey {
    const GAP_POS: Pos = Pos { x: 0, y: 0 };
    const START_KEY: Self = Self::Activate;

    fn key_to_pos(&self) -> Pos {
        match self {
            Self::Left => Pos::new(0, 1),
            Self::Right => Pos::new(2, 1),
            Self::Up => Pos::new(1, 0),
            Self::Down => Pos::new(1, 1),
            Self::Activate => Pos::new(2, 0),
        }
    }
}

pub struct AocDay21 {
    codes: Vec<(usize, Vec<NumpadKey>)>,
}

impl AocDay<usize, usize> for AocDay21 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let codes = lines
            .map(|l| {
                let seq = l
                    .chars()
                    .map(|c| c.try_into())
                    .process_results(|it| it.collect_vec())?;
                let num: usize = l[..3].parse()?;
                Ok::<_, DayError>((num, seq))
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay21 { codes })
    }
    fn part1(&self) -> usize {
        self.codes
            .iter()
            .map(|(num, code)| num * NumpadKey::process_keypad(code.clone(), 2 + 1))
            .sum()
    }
    fn part2(&self) -> usize {
        self.codes
            .iter()
            .map(|(num, code)| num * NumpadKey::process_keypad(code.clone(), 25 + 1))
            .sum()
    }
}

#[cfg(test)]
mod day21tests {
    use super::*;

    const INPUT: &[&str] = &["029A", "980A", "179A", "456A", "379A"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay21::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 126384);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay21::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 154115708116294); // not provided in puzzle description
        Ok(())
    }
}
