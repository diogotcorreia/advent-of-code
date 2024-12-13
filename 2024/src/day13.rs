use aoc_common::{navigation::Vec2D, AocDay, DayError};
use itertools::Itertools;

type Pos = Vec2D<usize>;

struct Game {
    btn_a: Pos,
    btn_b: Pos,
    prize: Pos,
}

fn parse_loc(value: &str) -> Result<Pos, DayError> {
    let (_, pos) = value.split_once(": ").ok_or(DayError::GenericParseErr("can't split on ': '"))?;
    let (x, y) = pos.split_once(", ").ok_or(DayError::GenericParseErr("can't split on ', '"))?;
    Ok(Pos::new(x[2..].parse()?, y[2..].parse()?))
}

fn div_no_remainder(lhs: isize, rhs: isize) -> Option<isize> {
    if lhs % rhs != 0 {
        None
    } else {
        Some(lhs / rhs)
    }
}

fn solve(game: &Game, prize_offset: isize) -> Option<(usize, usize)> {
    let a_x = isize::try_from(game.btn_a.x).ok()?;
    let a_y = isize::try_from(game.btn_a.y).ok()?;
    let b_x = isize::try_from(game.btn_b.x).ok()?;
    let b_y = isize::try_from(game.btn_b.y).ok()?;
    let p_x = isize::try_from(game.prize.x).ok()? + prize_offset;
    let p_y = isize::try_from(game.prize.y).ok()? + prize_offset;

    // solve linear equation system:
    // a_x * s + b_x * t = p_x
    // a_y * s + b_y * t = p_y
    let t = div_no_remainder(p_x * a_y - p_y * a_x, b_x * a_y - a_x * b_y)?;
    let s = div_no_remainder(p_y - b_y * t, a_y)?;

    let s = Some(s)
        .filter(|s| *s >= 0)
        .and_then(|s| usize::try_from(s).ok());
    let t = Some(t)
        .filter(|t| *t >= 0)
        .and_then(|t| usize::try_from(t).ok());

    s.and_then(|s| t.map(|t| (s, t)))
}

pub struct AocDay13 {
    games: Vec<Game>,
}

impl AocDay<usize, usize> for AocDay13 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let games = lines
            .tuple_windows()
            .step_by(4)
            .map(|(btn_a, btn_b, prize)| {
                Ok::<_, DayError>(Game {
                    btn_a: parse_loc(&btn_a)?,
                    btn_b: parse_loc(&btn_b)?,
                    prize: parse_loc(&prize)?,
                })
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay13 { games })
    }
    fn part1(&self) -> usize {
        self.games
            .iter()
            .flat_map(|game| solve(game, 0))
            .map(|(s, t)| s * 3 + t)
            .sum()
    }
    fn part2(&self) -> usize {
        self.games
            .iter()
            .flat_map(|game| solve(game, 10000000000000))
            .map(|(s, t)| s * 3 + t)
            .sum()
    }
}

#[cfg(test)]
mod day13tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Button A: X+94, Y+34",
        "Button B: X+22, Y+67",
        "Prize: X=8400, Y=5400",
        "",
        "Button A: X+26, Y+66",
        "Button B: X+67, Y+21",
        "Prize: X=12748, Y=12176",
        "",
        "Button A: X+17, Y+86",
        "Button B: X+84, Y+37",
        "Prize: X=7870, Y=6450",
        "",
        "Button A: X+69, Y+23",
        "Button B: X+27, Y+71",
        "Prize: X=18641, Y=10279",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay13::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 480);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay13::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 875318608908); // not provided by puzzle
        Ok(())
    }
}
