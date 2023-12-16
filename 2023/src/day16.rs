use ndarray::{Array2, Array3, Axis};

use crate::AocDay;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorBackward,
    MirrorForward,
    SplitterVertical,
    SplitterHorizontal,
}

impl Tile {
    fn change_direction(&self, direction: &Direction) -> [Option<Direction>; 2] {
        match self {
            Tile::Empty => [Some(direction.clone()), None],
            Tile::MirrorBackward => {
                let new_direction = match direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                [Some(new_direction), None]
            }
            Tile::MirrorForward => {
                let new_direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                [Some(new_direction), None]
            }
            Tile::SplitterVertical => {
                match direction {
                    Direction::North | Direction::South => [Some(direction.clone()), None],
                    Direction::East | Direction::West => {
                        // go north and south
                        [Some(Direction::North), Some(Direction::South)]
                    }
                }
            }
            Tile::SplitterHorizontal => {
                match direction {
                    Direction::East | Direction::West => [Some(direction.clone()), None],
                    Direction::North | Direction::South => {
                        // go east and west
                        [Some(Direction::East), Some(Direction::West)]
                    }
                }
            }
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::MirrorForward),
            '\\' => Ok(Self::MirrorBackward),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Pos {
    fn move_pos(&self, limits: &[usize]) -> Option<Pos> {
        let (x, y) = (self.x, self.y);
        match self.direction {
            Direction::North => y.checked_sub(1).map(|y| (x, y)),
            Direction::East => x.checked_add(1).map(|x| (x, y)),
            Direction::South => y.checked_add(1).map(|y| (x, y)),
            Direction::West => x.checked_sub(1).map(|x| (x, y)),
        }
        .filter(|(x, y)| *x < limits[1] && *y < limits[0])
        .map(|(x, y)| Pos {
            x,
            y,
            direction: self.direction.clone(),
        })
    }
}

fn run_mirrors(map: &Array2<Tile>, pos: &Pos) -> usize {
    let mut cache = Array3::default((map.nrows(), map.ncols(), 4));

    run_mirrors_inner(map, pos, &mut cache);

    cache
        .lanes(Axis(2))
        .into_iter()
        .filter(|lane| lane.iter().any(|b| *b))
        .count()
}

fn run_mirrors_inner(map: &Array2<Tile>, pos: &Pos, cache: &mut Array3<bool>) {
    let tile = &map[(pos.y, pos.x)];

    if cache[(pos.y, pos.x, pos.direction.clone() as usize)] {
        return;
    }
    cache[(pos.y, pos.x, pos.direction.clone() as usize)] = true;

    tile.change_direction(&pos.direction)
        .into_iter()
        .flatten()
        .for_each(|new_direction| {
            let pos = Pos {
                x: pos.x,
                y: pos.y,
                direction: new_direction,
            };
            if let Some(new_pos) = pos.move_pos(map.shape()) {
                run_mirrors_inner(map, &new_pos, cache);
            }
        });
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay16 {
    map: Array2<Tile>,
}

impl AocDay<usize, usize> for AocDay16 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.peekable();
        let width = lines.peek().expect("map does not have any rows").len();
        let map: Vec<Tile> = lines
            .flat_map(|row| {
                row.chars()
                    .map(|c| c.try_into().expect("failed to parse character in map"))
                    .collect::<Vec<_>>()
            })
            .collect();

        AocDay16 {
            map: Array2::from_shape_vec((map.len() / width, width), map)
                .expect("failed to build array2"),
        }
    }
    fn part1(&self) -> usize {
        run_mirrors(
            &self.map,
            &Pos {
                x: 0,
                y: 0,
                direction: Direction::East,
            },
        )
    }
    fn part2(&self) -> usize {
        let sides = (0..self.map.nrows())
            .map(|y| {
                let left = run_mirrors(
                    &self.map,
                    &Pos {
                        x: 0,
                        y,
                        direction: Direction::East,
                    },
                );
                let right = run_mirrors(
                    &self.map,
                    &Pos {
                        x: self.map.ncols() - 1,
                        y,
                        direction: Direction::West,
                    },
                );
                left.max(right)
            })
            .max()
            .unwrap();
        let top_bottom = (0..self.map.ncols())
            .map(|x| {
                let top = run_mirrors(
                    &self.map,
                    &Pos {
                        x,
                        y: 0,
                        direction: Direction::South,
                    },
                );
                let bottom = run_mirrors(
                    &self.map,
                    &Pos {
                        x,
                        y: self.map.nrows() - 1,
                        direction: Direction::North,
                    },
                );
                top.max(bottom)
            })
            .max()
            .unwrap();

        sides.max(top_bottom)
    }
}

#[cfg(test)]
mod day16tests {
    use super::*;

    const INPUT: &[&str] = &[
        r".|...\....",
        r"|.-.\.....",
        ".....|-...",
        "........|.",
        "..........",
        r".........\",
        r"..../.\\..",
        ".-.-/..|..",
        r".|....-|.\",
        "..//.|....",
    ];

    #[test]
    fn part1() {
        let day = AocDay16::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 46);
    }

    #[test]
    fn part2() {
        let day = AocDay16::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 51);
    }
}
