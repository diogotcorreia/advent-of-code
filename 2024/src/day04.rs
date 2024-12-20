use aoc_common::{
    navigation::{Direction, Vec2D, VecScale, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use ndarray::Array2;

#[derive(Debug, PartialEq, Eq, Clone, TryFromChar)]
enum Letter {
    #[char_repr = 'X']
    X,
    #[char_repr = 'M']
    M,
    #[char_repr = 'A']
    A,
    #[char_repr = 'S']
    S,
}

type Pos = Vec2D<usize>;

fn move_pos(
    pos: &Pos,
    direction: Direction,
    count: isize,
    board_dim: (usize, usize),
) -> Option<Pos> {
    pos.vec_sum(&Vec2D::<isize>::from(direction).vec_scale(count)?)?
        .bind_to_map(&Pos::new(board_dim.1, board_dim.0))
}

fn is_word(mut iter: impl Iterator<Item = Letter>) -> bool {
    matches!(iter.next(), Some(Letter::X))
        && matches!(iter.next(), Some(Letter::M))
        && matches!(iter.next(), Some(Letter::A))
        && matches!(iter.next(), Some(Letter::S))
}

fn count_xmas_word(board: &Array2<Letter>, pos: Pos) -> usize {
    macro_rules! letter_iter {
        ($direction: expr) => {
            usize::from(is_word(
                (0..4)
                    .filter_map(|i| move_pos(&pos, $direction, i, board.dim()))
                    .map(|pos| board[(pos.y, pos.x)].clone()),
            ))
        };
    }

    letter_iter!(Direction::North)
        + letter_iter!(Direction::South)
        + letter_iter!(Direction::East)
        + letter_iter!(Direction::West)
        + letter_iter!(Direction::NorthEast)
        + letter_iter!(Direction::NorthWest)
        + letter_iter!(Direction::SouthEast)
        + letter_iter!(Direction::SouthWest)
}

fn is_mas(mut iter: impl Iterator<Item = Letter>) -> bool {
    matches!(iter.next(), Some(Letter::M))
        && matches!(iter.next(), Some(Letter::A))
        && matches!(iter.next(), Some(Letter::S))
}

fn count_x_mas(board: &Array2<Letter>, pos: Pos) -> bool {
    macro_rules! letter_iter {
        ($direction: expr) => {
            is_mas(
                (-1..=1)
                    .filter_map(|i| move_pos(&pos, $direction, i, board.dim()))
                    .map(|pos| board[(pos.y, pos.x)].clone()),
            )
        };
    }

    (letter_iter!(Direction::NorthEast) || letter_iter!(Direction::SouthWest))
        && (letter_iter!(Direction::NorthWest) || letter_iter!(Direction::SouthEast))
}

pub struct AocDay04 {
    board: Array2<Letter>,
}

impl AocDay<usize, usize> for AocDay04 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let board = try_parse_2d_array(lines)?;

        Ok(AocDay04 { board })
    }
    fn part1(&self) -> usize {
        self.board
            .indexed_iter()
            .filter(|(_, letter)| **letter == Letter::X)
            .map(|((y, x), _)| count_xmas_word(&self.board, Pos::new(x, y)))
            .sum()
    }
    fn part2(&self) -> usize {
        self.board
            .indexed_iter()
            .filter(|((y, x), letter)| {
                **letter == Letter::A && count_x_mas(&self.board, Pos::new(*x, *y))
            })
            .count()
    }
}

#[cfg(test)]
mod day04tests {
    use super::*;

    const INPUT: &[&str] = &[
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay04::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 18);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay04::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 9);
        Ok(())
    }
}
