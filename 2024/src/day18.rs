use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    AocDay, DayError,
};
use itertools::Itertools;
use ndarray::Array2;
use pathfinding::prelude::astar;

type Pos = Vec2D<usize>;

fn get_possible_next_positions(
    map: &Array2<bool>,
    pos: Pos,
) -> impl Iterator<Item = (Pos, usize)> + use<'_> {
    let map_bounds = Pos::new(map.ncols(), map.nrows());
    Direction::get_all_orthogonal()
        .flat_map(move |dir| pos.vec_sum(&Vec2D::<isize>::from(dir)))
        .flat_map(move |pos| pos.bind_to_map(&map_bounds))
        .filter(move |new_pos| !map[new_pos])
        .map(|pos| (pos, 1))
}

fn pos_dist(from: &Pos, to: &Pos) -> usize {
    from.x.abs_diff(to.x) + from.y.abs_diff(to.y)
}

pub struct AocDay18 {
    bytes: Vec<Pos>,
}

impl AocDay<usize, String> for AocDay18 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let bytes = lines
            .map(|l| {
                let (x, y) = l.split_once(',').ok_or(DayError::GenericParseErr(
                    "input pos does not include comma separator",
                ))?;

                Ok::<_, DayError>(Pos::new(x.parse()?, y.parse()?))
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay18 { bytes })
    }
    fn part1(&self) -> usize {
        self.part1_inner::<70, 1024>()
    }
    fn part2(&self) -> String {
        self.part2_inner::<70, 1024>()
    }
}

impl AocDay18 {
    fn part1_inner<const MAP_SIZE: usize, const BYTE_COUNT: usize>(&self) -> usize {
        let mut map = Array2::default((MAP_SIZE + 1, MAP_SIZE + 1));
        self.bytes
            .iter()
            .take(BYTE_COUNT)
            .for_each(|pos| map[pos] = true);

        let start_pos = Pos::new(0, 0);
        let end_pos = Pos::new(MAP_SIZE, MAP_SIZE);
        let (_, cost) = astar(
            &start_pos,
            |pos| get_possible_next_positions(&map, pos.clone()),
            |pos| pos_dist(pos, &end_pos),
            |pos| *pos == end_pos,
        )
        .expect("no solution found");

        cost
    }
    fn part2_inner<const MAP_SIZE: usize, const BYTE_COUNT: usize>(&self) -> String {
        let start_pos = Pos::new(0, 0);
        let end_pos = Pos::new(MAP_SIZE, MAP_SIZE);

        // binary search when it stops being possible to traverse the map
        let mut low = 0;
        let mut high = self.bytes.len();
        while low != high {
            let i = (low + high + 1) / 2;
            let mut map = Array2::default((MAP_SIZE + 1, MAP_SIZE + 1));
            self.bytes.iter().take(i).for_each(|pos| map[pos] = true);

            if astar(
                &start_pos,
                |pos| get_possible_next_positions(&map, pos.clone()),
                |pos| pos_dist(pos, &end_pos),
                |pos| *pos == end_pos,
            )
            .is_none()
            {
                high = i - 1;
            } else {
                low = i;
            }
        }

        let pos = &self.bytes[low];
        format!("{},{}", pos.x, pos.y)
    }
}

#[cfg(test)]
mod day18tests {
    use super::*;

    const INPUT: &[&str] = &[
        "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1", "1,2",
        "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6", "2,0",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay18::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1_inner::<6, 12>(), 22);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay18::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2_inner::<6, 12>(), "6,1");
        Ok(())
    }
}
