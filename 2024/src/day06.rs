use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use ndarray::Array2;

fn rotate_cw(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        _ => unreachable!(),
    }
}

fn dir_to_u8(direction: &Direction) -> u8 {
    match direction {
        Direction::North => 1 << 0,
        Direction::East => 1 << 1,
        Direction::South => 1 << 2,
        Direction::West => 1 << 3,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq, TryFromChar)]
enum Tile {
    #[char_repr = '#']
    Obstacle,
    #[char_repr = '.']
    Air,
    #[char_repr = '^']
    StartingPosition,
}

type Pos = Vec2D<usize>;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct DirectedPos {
    pos: Pos,
    direction: Direction,
}

pub struct AocDay06 {
    guard: DirectedPos,
    map: Array2<Tile>,
    map_size: Pos,
}

fn is_loop(
    map: &Array2<Tile>,
    mut guard_pos: DirectedPos,
    map_size: &Pos,
    mut seen: Array2<u8>,
    new_obstacle: &Pos,
) -> bool {
    while let Some(new_pos) = guard_pos
        .pos
        .vec_sum(&Vec2D::<isize>::from(guard_pos.direction.clone()))
        .and_then(|pos| pos.bind_to_map(map_size))
    {
        if map[&new_pos] == Tile::Obstacle || *new_obstacle == new_pos {
            guard_pos.direction = rotate_cw(&guard_pos.direction);
        } else {
            let dir_mask = dir_to_u8(&guard_pos.direction);
            if seen[&new_pos] & dir_mask != 0 {
                // already been in this position: loop
                return true;
            }
            seen[&new_pos] |= dir_mask;
            guard_pos.pos = new_pos;
        }
    }

    false
}

impl AocDay<usize, usize> for AocDay06 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines)?;

        let guard = map
            .indexed_iter()
            .find(|(_, tile)| **tile == Tile::StartingPosition)
            .map(|((y, x), _)| DirectedPos {
                direction: Direction::North,
                pos: Pos::new(x, y),
            })
            .ok_or(DayError::GenericParseErr(
                "couldn't find a starting position on the map",
            ))?;

        Ok(AocDay06 {
            guard,
            map_size: Pos::new(map.ncols(), map.nrows()),
            map,
        })
    }
    fn part1(&self) -> usize {
        let mut seen = Array2::<bool>::default(self.map.dim());
        let mut guard_pos = self.guard.clone();

        seen[&guard_pos.pos] = true;

        while let Some(new_pos) = guard_pos
            .pos
            .vec_sum(&Vec2D::<isize>::from(guard_pos.direction.clone()))
            .and_then(|pos| pos.bind_to_map(&self.map_size))
        {
            if self.map[&new_pos] == Tile::Obstacle {
                guard_pos.direction = rotate_cw(&guard_pos.direction);
            } else {
                seen[&new_pos] = true;
                guard_pos.pos = new_pos;
            }
        }

        seen.iter().filter(|b| **b).count()
    }
    fn part2(&self) -> usize {
        let mut seen = Array2::<u8>::zeros(self.map.dim());
        let mut guard_pos = self.guard.clone();
        let mut count = 0;

        while let Some(new_pos) = guard_pos
            .pos
            .vec_sum(&Vec2D::<isize>::from(guard_pos.direction.clone()))
            .and_then(|pos| pos.bind_to_map(&self.map_size))
        {
            if self.map[&new_pos] == Tile::Obstacle {
                guard_pos.direction = rotate_cw(&guard_pos.direction);
            } else {
                let is_new_pos = seen[&new_pos] == 0;
                seen[&new_pos] |= dir_to_u8(&guard_pos.direction);
                if is_new_pos
                    && is_loop(
                        &self.map,
                        guard_pos.clone(),
                        &self.map_size,
                        seen.clone(),
                        &new_pos,
                    )
                {
                    count += 1;
                }
                guard_pos.pos = new_pos;
            }
        }

        count
    }
}

#[cfg(test)]
mod day06tests {
    use super::*;

    const INPUT: &[&str] = &[
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay06::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 41);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay06::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 6);
        Ok(())
    }
}
