use std::collections::HashSet;

use aoc_common::{navigation::Vec2D, AocDay, DayError};
use itertools::Itertools;
use ndarray::Array2;
use pathfinding::prelude::dfs;

type ShapeInner = [bool; 9];
type Pos = Vec2D<usize>;

#[derive(Debug)]
struct Shape {
    all_variants: HashSet<ShapeInner>,
    tile_count: usize,
}

impl Shape {
    fn new(tiles: [bool; 9]) -> Self {
        let all_variants = Self::rotate_all(tiles)
            .chain(Self::rotate_all(Self::flip_v(tiles)))
            .chain(Self::rotate_all(Self::flip_h(tiles)))
            .collect();
        let tile_count = tiles.iter().filter(|&&b| b).count();
        Self {
            all_variants,
            tile_count,
        }
    }

    fn rotate_cw(tiles: ShapeInner) -> ShapeInner {
        [6, 3, 0, 7, 4, 1, 8, 5, 2].map(|i| tiles[i])
    }

    fn rotate_all(tiles: ShapeInner) -> impl Iterator<Item = ShapeInner> {
        FullRotation::new(tiles).take(4)
    }

    fn flip_v(tiles: ShapeInner) -> ShapeInner {
        [6, 7, 8, 3, 4, 5, 0, 1, 2].map(|i| tiles[i])
    }

    fn flip_h(tiles: ShapeInner) -> ShapeInner {
        [2, 5, 8, 1, 4, 7, 0, 3, 6].map(|i| tiles[i])
    }
}

struct FullRotation {
    start: ShapeInner,
    prev: Option<ShapeInner>,
}

impl FullRotation {
    fn new(tiles: ShapeInner) -> Self {
        Self {
            start: tiles,
            prev: None,
        }
    }
}

impl Iterator for FullRotation {
    type Item = ShapeInner;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prev) = self.prev {
            let next = Shape::rotate_cw(prev);
            self.prev = Some(next);
            Some(next)
        } else {
            self.prev = Some(self.start);
            Some(self.start)
        }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    pieces: Vec<usize>,
}

impl Region {
    fn has_solution_trivial(&self, shapes: &[Shape]) -> Option<bool> {
        // TIHI, why did I even write the rest of the code
        let area = self.width * self.height;
        let total_piece_count: usize = self.pieces.iter().sum();
        let needed_area_9_by_9 = total_piece_count * 9;
        if needed_area_9_by_9 <= area {
            Some(true)
        } else {
            let needed_area_tiles: usize = self
                .pieces
                .iter()
                .zip(shapes)
                .map(|(count, shape)| count * shape.tile_count)
                .sum();
            if needed_area_tiles > area {
                Some(false)
            } else {
                None
            }
        }
    }

    fn has_solution(&self, shapes: &[Shape]) -> bool {
        if let Some(sol) = self.has_solution_trivial(shapes) {
            return sol;
        }
        let arr = Array2::from_elem((self.width, self.height), false);
        dfs(
            (
                arr,
                0,
                self.pieces
                    .iter()
                    .enumerate()
                    .find(|(_, &c)| c > 0)
                    .map(|(i, _)| i)
                    .unwrap_or(usize::MAX),
                0,
            ),
            |(arr, start_at, shape_i, piece_i)| {
                let (next_shape_i, next_piece_i, next_start_at) =
                    if piece_i + 1 >= self.pieces[*shape_i] {
                        (
                            self.pieces
                                .iter()
                                .enumerate()
                                .skip(shape_i + 1)
                                .find(|(_, &c)| c > 0)
                                .map(|(i, _)| i)
                                .unwrap_or(usize::MAX),
                            0,
                            Some(0),
                        )
                    } else {
                        (*shape_i, piece_i + 1, None)
                    };

                Self::calculate_new_regions(arr, &shapes[*shape_i], *start_at)
                    .into_iter()
                    .map(move |(arr, start_at)| {
                        (
                            arr,
                            next_start_at.unwrap_or(start_at),
                            next_shape_i,
                            next_piece_i,
                        )
                    })
            },
            |(_, _, shape_i, _)| *shape_i >= self.pieces.len(),
        )
        .is_some()
    }

    fn calculate_new_regions(
        current: &Array2<bool>,
        piece: &Shape,
        start_at: usize,
    ) -> Vec<(Array2<bool>, usize)> {
        let ncols = current.ncols();
        (0..current.len())
            .skip(start_at)
            .flat_map(|i| {
                let (x, y) = (i % ncols, i / ncols);
                piece
                    .all_variants
                    .iter()
                    .filter_map(move |shape_inner| {
                        Self::place_on_region(current, &Pos::new(x, y), shape_inner)
                    })
                    .map(move |arr| (arr, i))
            })
            .collect_vec()
    }

    fn place_on_region(
        current: &Array2<bool>,
        anchor: &Pos,
        piece: &ShapeInner,
    ) -> Option<Array2<bool>> {
        let mut clone = current.clone();
        for delta_x in 0..3 {
            for delta_y in 0..3 {
                if !piece[delta_y * 3 + delta_x] {
                    continue;
                }
                let pos = Pos::new(anchor.x + delta_x, anchor.y + delta_y);
                if *clone.get((pos.y, pos.x)).unwrap_or(&true) {
                    return None;
                }
                clone[&pos] = true;
            }
        }
        Some(clone)
    }
}

pub struct AocDay12 {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl AocDay<usize, &'static str> for AocDay12 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let mut shapes = vec![];

        let mut lines = lines.peekable();
        while lines.peek().map(|s| s.ends_with(':')).unwrap_or_default() {
            lines.next(); // consume index line
            let mut tiles = [false; 9];
            for i in 0..3 {
                let line = lines
                    .next()
                    .ok_or(DayError::GenericParseErr("expected tile"))?;
                for (j, c) in line.chars().take(3).enumerate() {
                    let value = match c {
                        '.' => false,
                        '#' => true,
                        _ => return Err(DayError::TryFromCharErr("failed to parse tile")),
                    };
                    tiles[i * 3 + j] = value;
                }
            }
            lines.next(); // consume empty line

            shapes.push(Shape::new(tiles));
        }

        let regions = lines
            .map(|line| {
                let (dim, counts) = line
                    .split_once(": ")
                    .ok_or(DayError::GenericParseErr("missing : in region"))?;
                let (width, height) = dim
                    .split_once('x')
                    .ok_or(DayError::GenericParseErr("missing x in region"))?;
                let (width, height) = (width.parse()?, height.parse()?);
                let counts = counts
                    .split_whitespace()
                    .map(|c| c.parse())
                    .process_results(|it| it.collect_vec())?;

                Ok::<_, DayError>(Region {
                    width,
                    height,
                    pieces: counts,
                })
            })
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay12 { shapes, regions })
    }
    fn part1(&self) -> usize {
        self.regions
            .iter()
            .filter(|r| r.has_solution(&self.shapes))
            .count()
    }
    fn part2(&self) -> &'static str {
        // there is no part 2
        "none"
    }
}

#[cfg(test)]
mod day12tests {
    use super::*;

    const INPUT: &[&str] = &[
        "0:",
        "###",
        "##.",
        "##.",
        "",
        "1:",
        "###",
        "##.",
        ".##",
        "",
        "2:",
        ".##",
        "###",
        "##.",
        "",
        "3:",
        "##.",
        "###",
        "##.",
        "",
        "4:",
        "###",
        "#..",
        "###",
        "",
        "5:",
        "###",
        ".#.",
        "###",
        "",
        "4x4: 0 0 0 0 2 0",
        "12x5: 1 0 1 0 2 2",
        "12x5: 1 0 1 0 3 2",
    ];

    #[test]
    #[ignore] // takes too long to run
    fn part1() -> Result<(), DayError> {
        let day = AocDay12::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 2);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay12::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), "none");
        Ok(())
    }
}
