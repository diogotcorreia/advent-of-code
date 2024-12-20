use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use ndarray::Array2;
use pathfinding::prelude::{count_paths, dfs_reach};

type Pos = Vec2D<usize>;

struct Tile {
    height: u8,
}

impl TryFrom<char> for Tile {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
            height: value
                .to_digit(10)
                .ok_or(DayError::GenericParseErr("can't parse char as digit"))?
                as u8,
        })
    }
}

fn get_all_trailheads(map: &Array2<Tile>) -> impl Iterator<Item = Pos> + use<'_> {
    map.indexed_iter()
        .filter(|(_, tile)| tile.height == 0)
        .map(|((y, x), _)| Pos::new(x, y))
}

fn get_possible_next_positions(
    map: &Array2<Tile>,
    pos: Pos,
) -> impl Iterator<Item = Pos> + use<'_> {
    let old_height = map[&pos].height;
    let map_bounds = Pos::new(map.ncols(), map.nrows());
    Direction::get_all_orthogonal()
        .flat_map(move |dir| pos.vec_sum(&Vec2D::<isize>::from(dir)))
        .flat_map(move |pos| pos.bind_to_map(&map_bounds))
        .filter(move |new_pos| {
            let new_height = map[new_pos].height;
            old_height + 1 == new_height
        })
}

pub struct AocDay10 {
    map: Array2<Tile>,
}

impl AocDay<usize, usize> for AocDay10 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines)?;
        Ok(AocDay10 { map })
    }
    fn part1(&self) -> usize {
        get_all_trailheads(&self.map)
            .map(|pos| {
                dfs_reach(pos, |p| get_possible_next_positions(&self.map, p.clone()))
                    .filter(|pos| self.map[pos].height == 9)
                    .count()
            })
            .sum()
    }
    fn part2(&self) -> usize {
        get_all_trailheads(&self.map)
            .map(|pos| {
                count_paths(
                    pos,
                    |p| get_possible_next_positions(&self.map, p.clone()),
                    |pos| self.map[pos].height == 9,
                )
            })
            .sum()
    }
}

#[cfg(test)]
mod day10tests {
    use super::*;

    const INPUT: &[&str] = &[
        "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
        "10456732",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay10::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 36);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay10::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 81);
        Ok(())
    }
}
