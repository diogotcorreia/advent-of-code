use std::collections::VecDeque;

use ndarray::Array2;

use crate::AocDay;

#[derive(PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | 'S' => Ok(Self::Garden),
            '#' => Ok(Self::Rock),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn move_pos(&self, direction: &Direction, limits: &[usize]) -> Option<Pos> {
        let (x, y) = (self.x, self.y);
        match direction {
            Direction::North => y.checked_sub(1).map(|y| (x, y)),
            Direction::East => x.checked_add(1).map(|x| (x, y)),
            Direction::South => y.checked_add(1).map(|y| (x, y)),
            Direction::West => x.checked_sub(1).map(|x| (x, y)),
        }
        .filter(|(x, y)| *x < limits[1] && *y < limits[0])
        .map(|(x, y)| Pos { x, y })
    }
}

pub struct AocDay21 {
    map: Array2<Tile>,
    start: Pos,
}

impl AocDay21 {
    /// Given the amount of tiles to multiply (i.e. 0 will result in a 1x1 grid, 1 will be 3x3, 2
    /// will be 5x5, etc), run a BFS on it to find the minimum distance to each garden.
    fn bfs(&self, tiles_multiplier: usize) -> Array2<Option<usize>> {
        let side_len = self.map.nrows() + 2 * tiles_multiplier * self.map.nrows();
        let mut array = Array2::default((side_len, side_len));

        let start = Pos {
            x: self.start.x + tiles_multiplier * self.map.ncols(),
            y: self.start.y + tiles_multiplier * self.map.ncols(),
        };
        let mut queue = VecDeque::new();

        array[(start.y, start.x)] = Some(0);
        queue.push_back((0usize, start));

        while let Some((dist, pos)) = queue.pop_front() {
            Direction::get_all()
                .into_iter()
                .filter_map(|direction| pos.move_pos(&direction, &[side_len, side_len]))
                .filter(|pos| {
                    *self
                        .map
                        .get((
                            pos.y.rem_euclid(self.map.nrows()),
                            pos.x.rem_euclid(self.map.ncols()),
                        ))
                        .unwrap()
                        == Tile::Garden
                })
                .for_each(|pos| {
                    if array[(pos.y, pos.x)].is_none() {
                        array[(pos.y, pos.x)] = Some(dist + 1);
                        queue.push_back((dist + 1, pos));
                    }
                })
        }

        array
    }

    // this is black magic
    fn extrapolate_count(
        &self,
        cache: &mut Array2<Option<usize>>,
        dist: usize,
        steps: usize,
        is_corner: bool,
    ) -> usize {
        if dist > steps {
            return 0;
        }
        let cache_i = is_corner as usize; // 0 for edges, 1 for corners
        if let Some(count) = cache[(dist, cache_i)] {
            return count;
        }

        let tile_size = self.map.ncols();
        let remaining_tiles = (steps - dist) / tile_size;
        let max_dist = dist + tile_size * remaining_tiles;
        if remaining_tiles == 0 {
            cache[(dist, cache_i)] = Some(0);
            return 0;
        }
        if is_corner {
            // sum (n + 1), n=1 to a, for all n that have the right parity
            let mut count = if (dist + tile_size) % 2 == steps % 2 {
                // we want 2, 4, 6, etc
                let seq_length = remaining_tiles / 2;
                // sum 2n, n=1 to a
                seq_length * (seq_length + 1)
            } else {
                // we want 3, 5, 7, etc
                let seq_length = (remaining_tiles - 1) / 2;
                // sum 2n+1, n=1 to a
                seq_length * (seq_length + 2)
            };
            if max_dist <= steps && max_dist % 2 == steps % 2 {
                count += remaining_tiles + 1;
            }
            cache[(dist, cache_i)] = Some(count);
            count
        } else {
            let mut count = (remaining_tiles - 1) / 2;
            if remaining_tiles % 2 == 0 && (dist + tile_size) % 2 == steps % 2 {
                count += 1;
            }
            if max_dist <= steps && max_dist % 2 == steps % 2 {
                count += 1;
            }
            cache[(dist, cache_i)] = Some(count);
            count
        }
    }

    fn simulate(&self, steps: usize) -> usize {
        assert_eq!(self.map.nrows(), self.map.ncols());

        let tile_size = self.map.ncols();
        let tiles_multiplier = 3.min((steps + (tile_size / 2)) / tile_size);
        let grid_size = (1 + tiles_multiplier * 2) * tile_size;
        let bfs = self.bfs(tiles_multiplier);
        let max_dist = bfs.iter().flatten().max().cloned().unwrap_or(0) + 1;

        let mut count = 0;
        let mut cache: Array2<Option<usize>> = Array2::default((max_dist + 1, 2));

        for ((y, x), dist) in bfs
            .indexed_iter()
            .flat_map(|(pos, dist)| dist.map(|dist| (pos, dist)))
        {
            let is_col_edge = x.abs_diff(0) < tile_size || x.abs_diff(grid_size) <= tile_size;
            let is_row_edge = y.abs_diff(0) < tile_size || y.abs_diff(grid_size) <= tile_size;

            if is_col_edge && is_row_edge {
                // this is a corner
                count += self.extrapolate_count(&mut cache, dist, steps, true);
            } else if is_col_edge || is_row_edge {
                // just an edge
                count += self.extrapolate_count(&mut cache, dist, steps, false);
            }

            if dist <= steps && dist % 2 == steps % 2 {
                count += 1;
            }
        }

        count
    }
}

impl AocDay<usize, usize> for AocDay21 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut start = None;
        let mut lines = lines.peekable();
        let width = lines.peek().expect("map does not have any rows").len();
        let map: Vec<Tile> = lines
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some(Pos { x, y })
                        }
                        c.try_into().expect("unknown tile type")
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let map = Array2::from_shape_vec((map.len() / width, width), map)
            .expect("failed to build array2");

        AocDay21 {
            map,
            start: start.unwrap(),
        }
    }
    fn part1(&self) -> usize {
        self.simulate(64)
    }
    fn part2(&self) -> usize {
        self.simulate(26501365)
    }
}

#[cfg(test)]
mod day21tests {
    use super::*;

    const INPUT: &[&str] = &[
        "...........",
        ".....###.#.",
        ".###.##..#.",
        "..#.#...#..",
        "....#.#....",
        ".##..S####.",
        ".##..#...#.",
        ".......##..",
        ".##.#.####.",
        ".##..##.##.",
        "...........",
    ];

    #[test]
    fn part1() {
        let day = AocDay21::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.simulate(6), 16);
    }

    #[test]
    fn part2() {
        let day = AocDay21::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.simulate(10), 50);
        assert_eq!(day.simulate(50), 1594);
        assert_eq!(day.simulate(100), 6536);
        assert_eq!(day.simulate(500), 167004);
        assert_eq!(day.simulate(1000), 668697);
        assert_eq!(day.simulate(5000), 16733044);
    }
}
