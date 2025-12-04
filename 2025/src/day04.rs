use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use itertools::Itertools;
use ndarray::Array2;

type Pos = Vec2D<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromChar)]
enum Tile {
    #[char_repr = '.']
    Air,
    #[char_repr = '@']
    PaperRoll,
}

pub struct AocDay04 {
    map: Array2<Tile>,
}

impl AocDay<usize, usize> for AocDay04 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines)?;

        Ok(AocDay04 { map })
    }
    fn part1(&self) -> usize {
        let limits = Pos::new(self.map.ncols(), self.map.nrows());
        self.map
            .indexed_iter()
            .filter(|(_, &tile)| tile == Tile::PaperRoll)
            .map(|((y, x), _)| Pos::new(x, y))
            .filter(|pos| {
                Direction::get_all()
                    .filter_map(|dir| {
                        let adj_pos = pos
                            .vec_sum(&Vec2D::<isize>::from(dir))?
                            .bind_to_map(&limits)?;
                        Some(self.map[(adj_pos.y, adj_pos.x)])
                    })
                    .filter(|&tile| tile == Tile::PaperRoll)
                    .count()
                    < 4
            })
            .count()
    }
    fn part2(&self) -> usize {
        let limits = Pos::new(self.map.ncols(), self.map.nrows());
        let mut map = self.map.clone();
        let mut count = 0;
        loop {
            let changed = map
                .indexed_iter()
                .filter(|(_, &tile)| tile == Tile::PaperRoll)
                .map(|((y, x), _)| Pos::new(x, y))
                .filter(|pos| {
                    Direction::get_all()
                        .filter_map(|dir| {
                            let adj_pos = pos
                                .vec_sum(&Vec2D::<isize>::from(dir))?
                                .bind_to_map(&limits)?;
                            Some(map[(adj_pos.y, adj_pos.x)])
                        })
                        .filter(|&tile| tile == Tile::PaperRoll)
                        .count()
                        < 4
                })
                .collect_vec();
            changed.iter().for_each(|pos| {
                map[(pos.y, pos.x)] = Tile::Air;
            });
            if changed.is_empty() {
                break;
            }
            count += changed.len();
        }
        count
    }
}

#[cfg(test)]
mod day04tests {
    use super::*;

    const INPUT: &[&str] = &[
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay04::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 13);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay04::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 43);
        Ok(())
    }
}
