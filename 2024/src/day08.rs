use std::iter;

use aoc_common::{
    navigation::{Vec2D, VecScale, VecSum},
    parsing::{try_parse_sparse_2d_array, MaybeParseChar},
    AocDay, DayError,
};
use itertools::Itertools;
use ndarray::Array2;

type Pos = Vec2D<usize>;
type IPos = Vec2D<isize>;

struct Tower {
    pos: Pos,
    freq: char,
}

impl MaybeParseChar for Tower {
    type Error = DayError;

    fn maybe_parse_char(pos: Pos, value: char) -> Result<Option<Self>, Self::Error> {
        match value {
            '.' => Ok(None),
            '0'..='9' => Ok(Some(Self { pos, freq: value })),
            'a'..='z' => Ok(Some(Self { pos, freq: value })),
            'A'..='Z' => Ok(Some(Self { pos, freq: value })),
            _ => Err(DayError::GenericParseErr("unknown tile")),
        }
    }
}

/// Return iterator of positions of line (defined by start position and vector)
/// when scaled by the given factors.
/// Only positions inside the given map_bounds are included.
fn line_pos_iter<'a, T: Iterator<Item = isize>>(
    pos: &'a Pos,
    vec: &'a IPos,
    map_bounds: &'a Pos,
    factors: T,
) -> impl Iterator<Item = Pos> + use<'a, T> {
    factors
        .map(move |scale_factor| {
            Some(pos)
                .and_then(|v| {
                    vec.vec_scale(scale_factor)
                        .and_then(|dist| v.vec_sum(&dist))
                })
                .and_then(|v| v.bind_to_map(map_bounds))
        })
        .take_while(|v| v.is_some())
        .flatten()
}

pub struct AocDay08 {
    map: Vec<Tower>,
    map_bounds: Pos,
}

impl AocDay<usize, usize> for AocDay08 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let (map, map_bounds) = try_parse_sparse_2d_array(lines)?;

        Ok(AocDay08 { map, map_bounds })
    }
    fn part1(&self) -> usize {
        let mut antinode_map: Array2<bool> =
            Array2::default((self.map_bounds.y, self.map_bounds.x));

        self.map
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.freq == b.freq)
            .for_each(|(a, b)| {
                let dist_x = b.pos.x as isize - a.pos.x as isize;
                let dist_y = b.pos.y as isize - a.pos.y as isize;
                let dist = IPos::new(dist_x, dist_y);

                line_pos_iter(&b.pos, &dist, &self.map_bounds, iter::once(1))
                    .for_each(|pos| antinode_map[(pos.y, pos.x)] = true);
                line_pos_iter(&a.pos, &dist, &self.map_bounds, iter::once(-1))
                    .for_each(|pos| antinode_map[(pos.y, pos.x)] = true);
            });

        antinode_map.iter().filter(|b| **b).count()
    }
    fn part2(&self) -> usize {
        let mut antinode_map: Array2<bool> =
            Array2::default((self.map_bounds.y, self.map_bounds.x));

        self.map
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.freq == b.freq)
            .for_each(|(a, b)| {
                let dist_x = b.pos.x as isize - a.pos.x as isize;
                let dist_y = b.pos.y as isize - a.pos.y as isize;
                let dist = IPos::new(dist_x, dist_y);

                line_pos_iter(&b.pos, &dist, &self.map_bounds, 0..)
                    .for_each(|pos| antinode_map[(pos.y, pos.x)] = true);
                line_pos_iter(&a.pos, &dist, &self.map_bounds, (0..).map(|v| -v))
                    .for_each(|pos| antinode_map[(pos.y, pos.x)] = true);
            });

        antinode_map.iter().filter(|b| **b).count()
    }
}

#[cfg(test)]
mod day08tests {
    use super::*;

    const INPUT: &[&str] = &[
        "............",
        "........0...",
        ".....0......",
        ".......0....",
        "....0.......",
        "......A.....",
        "............",
        "............",
        "........A...",
        ".........A..",
        "............",
        "............",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay08::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 14);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay08::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 34);
        Ok(())
    }
}
