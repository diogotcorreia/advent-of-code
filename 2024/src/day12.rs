use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use itertools::Itertools;
use ndarray::Array2;
use pathfinding::prelude::bfs_reach;

type Pos = Vec2D<usize>;

fn get_possible_next_positions(
    map: &Array2<char>,
    pos: Pos,
) -> impl Iterator<Item = Pos> + use<'_> {
    let region = map[&pos];
    let map_bounds = Pos::new(map.ncols(), map.nrows());
    Direction::get_all_orthogonal()
        .flat_map(move |dir| pos.vec_sum(&Vec2D::<isize>::from(dir)))
        .flat_map(move |pos| pos.bind_to_map(&map_bounds))
        .filter(move |new_pos| map[new_pos] == region)
}

fn get_perimeter(map: &Array2<char>, pos: Pos) -> usize {
    Direction::get_all_orthogonal().count() - get_possible_next_positions(map, pos).count()
}

/// First return value is the delta to apply to the perimeter of the region, while
/// the second is a mask of the directions that are part of the perimeter.
///
/// Delta is calculated by counting all directions part of perimeter and subtracting
/// one per neighbour that also has each direction.
/// This delta can be negative in the (edge) case that two sides are joined by a tile,
/// therefore the two separate sides are now a single side.
fn get_perimeter_mask(map: &Array2<char>, pos: &Pos, visited: &Array2<u8>) -> (isize, u8) {
    let region = map[pos];
    let map_bounds = Pos::new(map.ncols(), map.nrows());
    Direction::get_all_orthogonal()
        .map(move |dir| {
            let dir_mask = dir.to_mask();
            (
                dir_mask,
                pos.vec_sum(&Vec2D::<isize>::from(dir)).and_then(|pos| {
                    pos.bind_to_map(&map_bounds)
                        .filter(|p| map[p] == region)
                }),
            )
        })
        // sort here so that we can know the full mask before calculating the delta
        .sorted_by_key(move |(_, pos)| pos.is_some())
        .fold(
            (0, 0),
            |(perimeter_delta, unfiltered_mask), (dir_mask, new_pos)| {
                if let Some(new_pos) = new_pos {
                    (
                        perimeter_delta
                            - (unfiltered_mask & visited[&new_pos]).count_ones()
                                as isize,
                        unfiltered_mask,
                    )
                } else {
                    let new_mask = unfiltered_mask | dir_mask;
                    (new_mask.count_ones() as isize, new_mask)
                }
            },
        )
}

pub struct AocDay12 {
    map: Array2<char>,
}

impl AocDay<usize, isize> for AocDay12 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines)?;

        Ok(AocDay12 { map })
    }
    fn part1(&self) -> usize {
        let mut visited = Array2::<bool>::default(self.map.dim());

        self.map
            .indexed_iter()
            .map(|((y, x), _)| {
                if !visited[(y, x)] {
                    let pos = Pos::new(x, y);
                    let (area, perimeter) =
                        bfs_reach(pos, |p| get_possible_next_positions(&self.map, p.clone())).fold(
                            (0, 0),
                            |(area, perimeter), pos| {
                                visited[&pos] = true;
                                let pos_perimeter = get_perimeter(&self.map, pos);
                                (area + 1, perimeter + pos_perimeter)
                            },
                        );

                    area * perimeter
                } else {
                    0
                }
            })
            .sum()
    }
    fn part2(&self) -> isize {
        let mut visited = Array2::<u8>::zeros(self.map.dim());

        self.map
            .indexed_iter()
            .map(|((y, x), _)| {
                if visited[(y, x)] == 0 {
                    let pos = Pos::new(x, y);
                    let (area, perimeter) =
                        bfs_reach(pos, |p| get_possible_next_positions(&self.map, p.clone())).fold(
                            (0, 0),
                            |(area, perimeter), pos| {
                                let (perimeter_delta, unfiltered_mask) =
                                    get_perimeter_mask(&self.map, &pos, &visited);
                                // add 1 << 7 so that inner positions are marked as visited
                                visited[&pos] |= (1 << 7) | unfiltered_mask;
                                (area + 1, perimeter + perimeter_delta)
                            },
                        );

                    area * perimeter
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod day12tests {
    use super::*;

    const INPUT: &[&str] = &[
        "RRRRIICCFF",
        "RRRRIICCCF",
        "VVRRRCCFFF",
        "VVRCCCJFFF",
        "VVVVCJJCFE",
        "VVIVCCJJEE",
        "VVIIICJJEE",
        "MIIIIIJJEE",
        "MIIISIJEEE",
        "MMMISSJEEE",
    ];

    // Edge case on my strategy only due to how sides are counted
    const INPUT_EDGE_CASE: &[&str] = &[
        "AAAAAAAAAAAAAAAAAAAAAA",
        "ABBBBBBBBBBBBBBBBBBBBA",
        "AAAABBBBBBBBBBBBBBBBBA",
        "BBBABBBBBBBBBBBBBBBBBA",
        "AAAABBBBBBBBBBBBBBBBBA",
        "ABBBBBBBBBBBBBBBBBBBBA",
        "AAAAAAAAAAAAAAAAAAAAAA",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay12::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 1930);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay12::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 1206);
        Ok(())
    }

    #[test]
    fn part2_edgecase() -> Result<(), DayError> {
        let day = AocDay12::preprocessing_tests(INPUT_EDGE_CASE)?;
        assert_eq!(day.part2(), 1700);
        Ok(())
    }
}
