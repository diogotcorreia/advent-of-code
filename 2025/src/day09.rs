use aoc_common::{navigation::Vec2D, AocDay, DayError};
use itertools::Itertools;

type Pos = Vec2D<usize>;

fn rectangle_area(p1: &Pos, p2: &Pos) -> usize {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    pos: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Polygon {
    horizontal_segments: Vec<Segment>,
    vertical_segments: Vec<Segment>,
}

impl Polygon {
    fn new(points: &[Pos]) -> Self {
        let mut horizontal_segments = vec![];
        let mut vertical_segments = vec![];

        for i in 0..points.len() {
            let from = &points[i];
            let to = &points[(i + 1) % points.len()];
            if from.x == to.x {
                vertical_segments.push(Segment {
                    pos: from.x,
                    from: from.y.min(to.y),
                    to: from.y.max(to.y),
                });
            } else if from.y == to.y {
                horizontal_segments.push(Segment {
                    pos: from.y,
                    from: from.x.min(to.x),
                    to: from.x.max(to.x),
                });
            } else {
                unreachable!("input is not a contiguous polygon");
            }
        }

        vertical_segments.sort();
        horizontal_segments.sort();

        Self {
            horizontal_segments,
            vertical_segments,
        }
    }

    fn contains_rectangle(&self, p1: &Pos, p2: &Pos) -> bool {
        let top = p1.y.min(p2.y);
        let left = p1.x.min(p2.x);
        let bottom = p1.y.max(p2.y);
        let right = p1.x.max(p2.x);

        if left.abs_diff(right) > top.abs_diff(bottom) {
            // check rows (there are less rows than columns)
            Self::contains_rectangle_inner(
                &self.vertical_segments,
                &self.horizontal_segments,
                top,
                left,
                bottom,
                right,
            )
        } else {
            // check columns (there are more rows than columns)
            Self::contains_rectangle_inner(
                &self.horizontal_segments,
                &self.vertical_segments,
                left,
                top,
                right,
                bottom,
            )
        }
    }

    fn contains_rectangle_inner(
        vertical_segments: &[Segment],
        horizontal_segments: &[Segment],
        top: usize,
        left: usize,
        bottom: usize,
        right: usize,
    ) -> bool {
        // check if each "row" of the rectangle is inside the polygon
        for y in top..=bottom {
            let mut last_inside = None;
            let mut is_inside = false;
            let mut elbow = None;
            for segment in vertical_segments {
                if segment.pos >= right {
                    break;
                }
                if segment.from > y || segment.to < y {
                    continue;
                }
                let new_elbow = Self::classify_elbow(
                    segment,
                    Self::find_elbow_segment(horizontal_segments, segment.pos, y),
                );
                if new_elbow.is_none() || elbow != new_elbow {
                    if !is_inside && segment.pos > left && last_inside != Some(segment.pos - 1) {
                        return false;
                    }
                    is_inside = !is_inside;
                }
                elbow = new_elbow;
                if is_inside {
                    last_inside = Some(segment.pos);
                }
            }
            if !is_inside {
                return false;
            }
        }

        true
    }

    fn find_elbow_segment(horizontal_segments: &[Segment], x: usize, y: usize) -> Option<&Segment> {
        let partition = horizontal_segments.partition_point(|s| s.pos < y);
        horizontal_segments
            .iter()
            .skip(partition)
            .take_while(|s| s.pos <= y)
            .find(|s| s.pos == y && (s.from == x || s.to == x))
    }

    fn classify_elbow(seg1: &Segment, seg2: Option<&Segment>) -> Option<ElbowType> {
        let seg2 = seg2?;

        let x = seg1.pos;
        if x == seg2.from {
            if seg2.pos == seg1.from {
                Some(ElbowType::Type1)
            } else if seg2.pos == seg1.to {
                Some(ElbowType::Type2)
            } else {
                None
            }
        } else if x == seg2.to {
            if seg2.pos == seg1.from {
                Some(ElbowType::Type2)
            } else if seg2.pos == seg1.to {
                Some(ElbowType::Type1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ElbowType {
    Type1, // south-east, north-west
    Type2, // north-east, south-west
}

pub struct AocDay09 {
    red_tiles: Vec<Pos>,
}

impl AocDay<usize, usize> for AocDay09 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let red_tiles = lines
            .map(|line| {
                let (x, y) = line
                    .split_once(',')
                    .ok_or(DayError::GenericParseErr("missing delimiter"))?;
                Ok::<_, DayError>(Pos::new(x.parse()?, y.parse()?))
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay09 { red_tiles })
    }
    fn part1(&self) -> usize {
        self.red_tiles
            .iter()
            .tuple_combinations()
            .map(|(p1, p2)| rectangle_area(p1, p2))
            .max()
            .unwrap_or_default()
    }
    fn part2(&self) -> usize {
        let polygon = Polygon::new(&self.red_tiles);
        let mut result = 0;
        for (p1, p2) in self.red_tiles.iter().tuple_combinations() {
            let area = rectangle_area(p1, p2);
            if area > result && polygon.contains_rectangle(p1, p2) {
                result = area;
            }
        }
        result
    }
}

#[cfg(test)]
mod day09tests {
    use super::*;

    const INPUT: &[&str] = &["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay09::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 50);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay09::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 24);
        Ok(())
    }
}
