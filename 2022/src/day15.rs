use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

use crate::AocDay;

#[cfg(test)]
const TARGET_Y: i32 = 10;
#[cfg(not(test))]
const TARGET_Y: i32 = 2000000;

#[cfg(test)]
const MAX_Y: i32 = 20;
#[cfg(not(test))]
const MAX_Y: i32 = 4000000;

type Pos = (i32, i32);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Range1D {
    start: i32,
    end: i32,
}

impl Range1D {
    fn new(start: i32, end: i32) -> Range1D {
        Range1D { start, end }
    }

    fn len(&self) -> i32 {
        (self.end - self.start + 1).max(0)
    }

    // Remove "other" from "self"
    fn subtract(&self, other: &Range1D) -> impl Iterator<Item = Range1D> {
        [
            Range1D::new(self.start, self.end.min(other.start - 1)),
            Range1D::new(self.start.max(other.end + 1), self.end),
        ]
        .into_iter()
        .filter(|range| range.len() > 0)
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    closest_beacon: Pos,
    distance: i32,
}

impl Sensor {
    fn parse_coord<const N: usize>(string: &str) -> Result<i32, ParseErr> {
        string
            .get(2..(string.len() - N))
            .ok_or(ParseErr)?
            .parse()
            .map_err(|_| ParseErr)
    }

    fn get_1d_range_for_y(&self, y: i32) -> Option<Range1D> {
        let distance_to_y = (self.pos.1 - y).abs();

        let distance_left = self.distance - distance_to_y;
        if distance_left < 0 {
            None
        } else {
            Some(Range1D::new(
                self.pos.0 - distance_left,
                self.pos.0 + distance_left,
            ))
        }
    }
}

impl FromStr for Sensor {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let pos: Pos = (
            Self::parse_coord::<1>(split.nth(2).ok_or(ParseErr)?)?,
            Self::parse_coord::<1>(split.next().ok_or(ParseErr)?)?,
        );
        let closest_beacon: Pos = (
            Self::parse_coord::<1>(split.nth(4).ok_or(ParseErr)?)?,
            Self::parse_coord::<0>(split.next().ok_or(ParseErr)?)?,
        );

        Ok(Sensor {
            pos,
            closest_beacon,
            distance: manhattan_distance(&pos, &closest_beacon),
        })
    }
}

#[derive(Debug)]
struct ParseErr;

fn manhattan_distance(pos1: &Pos, pos2: &Pos) -> i32 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

pub struct AocDay15 {
    sensors: Vec<Sensor>,
}

impl AocDay15 {
    fn ranges_in_row(&self, target_y: i32) -> BTreeSet<Range1D> {
        let mut ranges: BTreeSet<Range1D> = BTreeSet::new();

        for sensor in self.sensors.iter() {
            let range = sensor.get_1d_range_for_y(target_y);
            if range.is_none() {
                continue;
            }
            ranges
                .iter()
                .fold(vec![range.unwrap().clone()], |r1, r2| {
                    r1.iter().flat_map(|r| r.subtract(r2)).collect()
                })
                .into_iter()
                .for_each(|r| {
                    ranges.insert(r);
                });
        }

        ranges
    }

    fn is_sequential(ranges: &BTreeSet<Range1D>) -> bool {
        ranges
            .iter()
            .zip(ranges.iter().skip(1))
            .all(|(r1, r2)| r1.end + 1 == r2.start)
    }

    fn find_hole(ranges: &BTreeSet<Range1D>) -> Option<i32> {
        ranges
            .iter()
            .zip(ranges.iter().skip(1))
            .find_map(|(r1, r2)| match r1.end + 1 - r2.start {
                0 => None,
                _ => Some(r1.end + 1),
            })
    }
}

impl AocDay<i32, i128> for AocDay15 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let sensors = lines
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to parse input");

        AocDay15 { sensors }
    }
    fn part1(&self) -> i32 {
        let ranges = self.ranges_in_row(TARGET_Y);
        let beacons_in_row: HashSet<Pos> = self
            .sensors
            .iter()
            .map(|sensor| sensor.closest_beacon)
            .filter(|beacon| beacon.1 == TARGET_Y)
            .collect();

        ranges.iter().map(Range1D::len).sum::<i32>() - beacons_in_row.len() as i32
    }

    fn part2(&self) -> i128 {
        let ranges = (0..=MAX_Y)
            .map(|y| (y, self.ranges_in_row(y)))
            .find(|(_, ranges)| !Self::is_sequential(ranges));

        let (y, ranges) = ranges.expect("no non-sequential row found");

        let x: i128 =
            Self::find_hole(&ranges).expect("non-sequential row does not have a hole") as i128;

        x * 4000000 + y as i128
    }
}

#[cfg(test)]
mod day15tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn range_subtract() {
        let range1 = Range1D::new(2, 2);
        let range2 = Range1D::new(12, 12);

        assert_eq!(
            range1.subtract(&range2).collect::<Vec<_>>(),
            vec![Range1D::new(2, 2)]
        );
    }

    #[test]
    fn part1() {
        let day = AocDay15::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 26);
    }

    #[test]
    fn part2() {
        let day = AocDay15::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 56000011);
    }
}
