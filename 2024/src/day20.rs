use aoc_common::{
    navigation::{Direction, Vec2D, VecRadius, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;

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

fn find_adjacent_pos(map: &Array2<Tile>, pos: Pos) -> impl Iterator<Item = (Pos, usize)> + use<'_> {
    Direction::get_all_orthogonal().flat_map(move |dir| {
        pos.vec_sum(&Vec2D::<isize>::from(dir))
            .filter(|new_pos| map[new_pos] != Tile::Wall)
            .map(|pos| (pos, 1))
    })
}

fn get_cheats<'a, 'b, const MAX_JUMP: isize>(
    dist_map: &'a Array2<Option<usize>>,
    path: &'b [Pos],
) -> impl Iterator<Item = usize> + use<'a, 'b, MAX_JUMP> {
    path.iter()
        .flat_map(|p| dist_map[p].map(|dist| (p, dist)))
        .flat_map(|(pos, dist)| {
            pos.all_points_manhattan_dist(MAX_JUMP)
                .flat_map(|(p, delta)| {
                    p.bind_to_map(&Pos::new(dist_map.ncols(), dist_map.nrows()))
                        .map(|d| (d, delta))
                })
                .flat_map(|(p, delta)| dist_map[&p].map(|d| (d, delta)))
                .filter(move |(d, _)| dist < *d)
                .map(move |(d, delta)| dist.abs_diff(d).saturating_sub(delta as usize))
                .filter(|savings| *savings > 0)
        })
}

pub struct AocDay20 {
    dist_map: Array2<Option<usize>>,
    path: Vec<Pos>,
}

impl AocDay<usize, usize> for AocDay20 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map: Array2<Tile> = try_parse_2d_array(lines)?;
        let start_pos = map
            .indexed_iter()
            .find(|(_, pos)| **pos == Tile::Start)
            .map(|((y, x), _)| Pos::new(x, y))
            .ok_or(DayError::GenericParseErr("can't find starting pos in map"))?;

        let (path, _) = dijkstra(
            &start_pos,
            |pos| find_adjacent_pos(&map, pos.clone()),
            |pos| map[pos] == Tile::End,
        )
        .expect("solution not found");

        let mut dist_map = Array2::default(map.raw_dim());

        path.iter()
            .enumerate()
            .for_each(|(dist, pos)| dist_map[pos] = Some(dist));

        Ok(AocDay20 { dist_map, path })
    }
    fn part1(&self) -> usize {
        get_cheats::<2>(&self.dist_map, &self.path)
            .filter(|savings| *savings >= 100)
            .count()
    }
    fn part2(&self) -> usize {
        get_cheats::<20>(&self.dist_map, &self.path)
            .filter(|savings| *savings >= 100)
            .count()
    }
}

#[cfg(test)]
mod day20tests {
    use itertools::Itertools;

    use super::*;

    const INPUT: &[&str] = &[
        "###############",
        "#...#...#.....#",
        "#.#.#.#.#.###.#",
        "#S#...#.#.#...#",
        "#######.#.#.###",
        "#######.#.#...#",
        "#######.#.###.#",
        "###..E#...#...#",
        "###.#######.###",
        "#...###...#...#",
        "#.#####.#.###.#",
        "#.#...#.#.#...#",
        "#.#.#.#.#.#.###",
        "#...#...#...###",
        "###############",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay20::preprocessing_tests(INPUT)?;
        let mut expected = vec![];
        expected.extend([2; 14]);
        expected.extend([4; 14]);
        expected.extend([6; 2]);
        expected.extend([8; 4]);
        expected.extend([10; 2]);
        expected.extend([12; 3]);
        expected.extend([20; 1]);
        expected.extend([36; 1]);
        expected.extend([38; 1]);
        expected.extend([40; 1]);
        expected.extend([64; 1]);

        let result = get_cheats::<2>(&day.dist_map, &day.path)
            .sorted()
            .collect_vec();
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay20::preprocessing_tests(INPUT)?;
        let mut expected = vec![];
        expected.extend([50; 32]);
        expected.extend([52; 31]);
        expected.extend([54; 29]);
        expected.extend([56; 39]);
        expected.extend([58; 25]);
        expected.extend([60; 23]);
        expected.extend([62; 20]);
        expected.extend([64; 19]);
        expected.extend([66; 12]);
        expected.extend([68; 14]);
        expected.extend([70; 12]);
        expected.extend([72; 22]);
        expected.extend([74; 4]);
        expected.extend([76; 3]);

        let result = get_cheats::<20>(&day.dist_map, &day.path)
            .filter(|savings| *savings >= 50)
            .sorted()
            .collect_vec();
        assert_eq!(result, expected);
        Ok(())
    }
}
