use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use ndarray::Array2;
use pathfinding::prelude::{astar, astar_bag};

type Pos = Vec2D<usize>;

#[derive(Debug, PartialEq, Eq, TryFromChar)]
enum Tile {
    #[char_repr = '#']
    Wall,
    #[char_repr = '.']
    Air,
    #[char_repr = 'S']
    Start,
    #[char_repr = 'E']
    End,
}

fn find_adjacent_pos(
    map: &Array2<Tile>,
    pos: DirectedPos,
) -> impl Iterator<Item = (DirectedPos, usize)> + use<'_> {
    let forward = pos
        .pos
        .vec_sum(&Vec2D::<isize>::from(pos.direction.clone()))
        .filter(|new_pos| map[new_pos] != Tile::Wall)
        .map(|new_pos| DirectedPos {
            pos: new_pos,
            direction: pos.direction.clone(),
        })
        .map(|pos| (pos, 1));

    let cw = DirectedPos {
        pos: pos.pos.clone(),
        direction: pos.direction.rotate_cw_90(),
    };
    let ccw = DirectedPos {
        pos: pos.pos.clone(),
        direction: pos.direction.rotate_ccw_90(),
    };

    [cw, ccw].into_iter().map(|pos| (pos, 1000)).chain(forward)
}

fn pos_dist(from: &Pos, to: &Pos) -> usize {
    from.x.abs_diff(to.x) + from.y.abs_diff(to.y)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DirectedPos {
    pos: Pos,
    direction: Direction,
}

pub struct AocDay16 {
    map: Array2<Tile>,
    start_pos: DirectedPos,
    end_pos: Pos,
}

impl AocDay<usize, usize> for AocDay16 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines)?;

        let start_pos = DirectedPos {
            pos: map
                .indexed_iter()
                .find(|(_, pos)| **pos == Tile::Start)
                .map(|((y, x), _)| Pos::new(x, y))
                .ok_or(DayError::GenericParseErr("can't find starting pos in map"))?,
            direction: Direction::East,
        };
        let end_pos = map
            .indexed_iter()
            .find(|(_, pos)| **pos == Tile::End)
            .map(|((y, x), _)| Pos::new(x, y))
            .ok_or(DayError::GenericParseErr("can't find end pos in map"))?;

        Ok(AocDay16 {
            map,
            start_pos,
            end_pos,
        })
    }
    fn part1(&self) -> usize {
        let (_, cost) = astar(
            &self.start_pos,
            |pos| find_adjacent_pos(&self.map, pos.clone()),
            |pos| pos_dist(&pos.pos, &self.end_pos),
            |pos| self.map[&pos.pos] == Tile::End,
        )
        .expect("no solution found");

        cost
    }
    fn part2(&self) -> usize {
        let mut visited = Array2::<bool>::default(self.map.raw_dim());
        let (paths, _) = astar_bag(
            &self.start_pos,
            |pos| find_adjacent_pos(&self.map, pos.clone()),
            |pos| pos_dist(&pos.pos, &self.end_pos),
            |pos| self.map[&pos.pos] == Tile::End,
        )
        .expect("no solution found");

        paths.for_each(|path| {
            path.iter().for_each(|p| {
                visited[&p.pos] = true;
            })
        });

        visited.iter().filter(|b| **b).count()
    }
}

#[cfg(test)]
mod day16tests {
    use super::*;

    const INPUT: &[&str] = &[
        "###############",
        "#.......#....E#",
        "#.#.###.#.###.#",
        "#.....#.#...#.#",
        "#.###.#####.#.#",
        "#.#.#.......#.#",
        "#.#.#####.###.#",
        "#...........#.#",
        "###.#.#####.#.#",
        "#...#.....#.#.#",
        "#.#.#.###.#.#.#",
        "#.....#...#.#.#",
        "#.###.#.#.#.#.#",
        "#S..#.....#...#",
        "###############",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay16::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 7036);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay16::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 45);
        Ok(())
    }
}
