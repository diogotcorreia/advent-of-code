use std::str::FromStr;

use aoc_common::{
    navigation::{Vec2D, VecScale, VecSum},
    AocDay, DayError,
};
use itertools::Itertools;

const MAP_BOUNDS: Pos = Pos { x: 101, y: 103 };

type Pos = Vec2D<isize>;

#[derive(Debug)]
struct Robot {
    position: Pos,
    velocity: Pos,
}

impl Robot {
    fn move_robot(&self, time: isize) -> Pos {
        self.velocity
            .vec_scale(time)
            .and_then(|vel| self.position.vec_sum(&vel))
            .expect("overflow when moving robot")
    }
}

impl FromStr for Robot {
    type Err = DayError;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        fn parse_pos(pos: &str) -> Result<Pos, DayError> {
            let (x, y) = pos
                .get(2..)
                .and_then(|s| s.split_once(','))
                .ok_or(DayError::GenericParseErr("cannot find comma in position"))?;

            Ok(Pos::new(x.parse()?, y.parse()?))
        }

        let (pos, vel) = l
            .split_once(' ')
            .ok_or(DayError::GenericParseErr("cannot find space in input line"))?;

        Ok(Robot {
            position: parse_pos(pos)?,
            velocity: parse_pos(vel)?,
        })
    }
}

/// Assumed bounds is odd, quadrant order is irrelevant
fn get_quadrant(pos: &Pos, bounds: &Pos) -> Option<usize> {
    let half_x = bounds.x / 2;
    let half_y = bounds.y / 2;

    if pos.x < half_x && pos.y < half_y {
        Some(0)
    } else if pos.x > half_x && pos.y < half_y {
        Some(1)
    } else if pos.x < half_x && pos.y > half_y {
        Some(2)
    } else if pos.x > half_x && pos.y > half_y {
        Some(3)
    } else {
        None
    }
}

pub struct AocDay14 {
    robots: Vec<Robot>,
}

impl AocDay<usize, isize> for AocDay14 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let robots = lines
            .map(|l| l.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay14 { robots })
    }
    fn part1(&self) -> usize {
        self.part1_inner(&MAP_BOUNDS)
    }
    fn part2(&self) -> isize {
        // A possible Christmas Tree will likely be in the middle
        // of the map, so a good heuristic for finding the value is how
        // close the robots are to the center of the map.
        // Additionally, the puzzle instructions ask for the first instance
        // this happens, hinting that the positions are cyclic.
        // Therefore find the instance with lowest value for the heuristic
        // until we are back at the start.
        (1..)
            .map(|i| {
                let (score, is_beginning) = self
                    .robots
                    .iter()
                    .map(|robot| robot.move_robot(i).wrap_out_of_bounds(&MAP_BOUNDS))
                    .enumerate()
                    .fold((0usize, true), |(score, is_beginning), (i, robot)| {
                        let score = score
                            + robot.x.abs_diff(MAP_BOUNDS.x / 2)
                            + robot.y.abs_diff(MAP_BOUNDS.y / 2);
                        let is_beginning = is_beginning && self.robots[i].position == robot;
                        (score, is_beginning)
                    });
                (i, score, is_beginning)
            })
            .take_while(|(_, _, is_beginning)| !is_beginning)
            .min_by_key(|(_, score, _)| *score)
            .map(|(i, _, _)| i)
            .expect("cannot find solution")
    }
}

impl AocDay14 {
    fn part1_inner(&self, map_bounds: &Pos) -> usize {
        self.robots
            .iter()
            .map(|robot| robot.move_robot(100).wrap_out_of_bounds(map_bounds))
            .fold([0; 4], |mut quadrants, robot| {
                if let Some(quadrant) = get_quadrant(&robot, map_bounds) {
                    quadrants[quadrant] += 1;
                }
                quadrants
            })
            .iter()
            .product()
    }
}

#[cfg(test)]
mod day14tests {
    use super::*;

    const MAP_BOUNDS_TEST: Pos = Pos { x: 11, y: 7 };

    const INPUT: &[&str] = &[
        "p=0,4 v=3,-3",
        "p=6,3 v=-1,-3",
        "p=10,3 v=-1,2",
        "p=2,0 v=2,-1",
        "p=0,0 v=1,3",
        "p=3,0 v=-2,-2",
        "p=7,6 v=-1,-3",
        "p=3,0 v=-1,-2",
        "p=9,3 v=2,3",
        "p=7,3 v=-1,2",
        "p=2,4 v=2,-3",
        "p=9,5 v=-3,-3",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay14::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1_inner(&MAP_BOUNDS_TEST), 12);
        Ok(())
    }

    // part 2 is untestable on example input
}
