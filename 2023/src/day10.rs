use std::collections::HashSet;

use crate::AocDay;

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn get_directions(&self) -> Vec<Direction> {
        match *self {
            Self::Vertical => vec![Direction::North, Direction::South],
            Self::Horizontal => vec![Direction::East, Direction::West],
            Self::NorthEast => vec![Direction::North, Direction::East],
            Self::NorthWest => vec![Direction::North, Direction::West],
            Self::SouthWest => vec![Direction::West, Direction::South],
            Self::SouthEast => vec![Direction::East, Direction::South],
            Self::Ground => vec![],
            Self::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }

    /// Convert pipe locations to a tile type.
    /// Used to convert the start tile into an actual tile
    fn from_directions(dirs: &[Direction]) -> Self {
        [
            Self::Vertical,
            Self::Horizontal,
            Self::NorthEast,
            Self::NorthWest,
            Self::SouthWest,
            Self::SouthEast,
        ]
        .into_iter()
        .find(|tile| {
            let tile_set: HashSet<Direction> = HashSet::from_iter(tile.get_directions());
            let other_set = HashSet::from_iter(dirs.iter().cloned());

            tile_set.eq(&other_set)
        })
        .expect("cannot find tile from list of directions")
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Vertical),
            '-' => Ok(Tile::Horizontal),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err(ParseErr),
        }
    }
}

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
    fn get_opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn move_pos(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        match self {
            Self::North => {
                if y != 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Self::East => Some((x + 1, y)),
            Self::South => Some((x, y + 1)),
            Self::West => {
                if x != 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        }
    }
}

fn get_tile(tiles: &[Vec<Tile>], pos: (usize, usize)) -> &Tile {
    tiles
        .get(pos.1)
        .and_then(|row| row.get(pos.0))
        .unwrap_or(&Tile::Ground)
}

fn get_start_directions(tiles: &[Vec<Tile>], start_pos: (usize, usize)) -> Vec<Direction> {
    Direction::get_all()
        .into_iter()
        .filter_map(|dir| {
            if let Some(next_pos) = dir.move_pos(start_pos) {
                let next_tile = get_tile(tiles, next_pos);

                let opposite_dir = dir.get_opposite();
                if next_tile
                    .get_directions()
                    .into_iter()
                    .any(|from_dir| from_dir == opposite_dir)
                {
                    return Some(dir);
                }
            }
            None
        })
        .collect()
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay10 {
    tiles: Vec<Vec<Tile>>,
    pipe_positions: HashSet<(usize, usize)>,
    start_directions: Vec<Direction>,
}

impl AocDay<usize, u32> for AocDay10 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut start_pos = None;
        let tiles: Vec<_> = lines
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = c.try_into().expect("invalid character in map");
                        if tile == Tile::Start {
                            start_pos = Some((x, y));
                        }
                        tile
                    })
                    .collect()
            })
            .collect();
        let start_pos = start_pos.expect("input does not contain a start position");

        // this is the same for part 1 and part 2, so do it here
        let start_directions = get_start_directions(&tiles, start_pos);
        let move_to = start_directions
            .first()
            .expect("start tile is not connected to other pipes");

        let mut pipe_positions = HashSet::new();
        pipe_positions.insert(start_pos);
        let mut curr_pos = move_to.move_pos(start_pos).expect("pipe outside map range");
        let mut from_dir = move_to.get_opposite();
        while curr_pos != start_pos {
            let curr_tile = get_tile(&tiles, curr_pos);
            let next_dir = curr_tile
                .get_directions()
                .into_iter()
                .find(|d| *d != from_dir)
                .expect("cannot find next direction to move to");

            pipe_positions.insert(curr_pos);
            curr_pos = next_dir.move_pos(curr_pos).expect("pipe outside map range");
            from_dir = next_dir.get_opposite();
        }

        AocDay10 {
            tiles,
            pipe_positions,
            start_directions,
        }
    }
    fn part1(&self) -> usize {
        self.pipe_positions.len() / 2
    }
    fn part2(&self) -> u32 {
        // figure out stuff
        let mut inside_count: u32 = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            let mut inside = false;
            let mut elbow_from = None;
            for (x, tile) in row.iter().enumerate() {
                if !self.pipe_positions.contains(&(x, y)) {
                    if inside {
                        inside_count += 1;
                    }
                    continue;
                }

                let tile = if *tile == Tile::Start {
                    Tile::from_directions(&self.start_directions)
                } else {
                    tile.clone()
                };

                match tile {
                    Tile::Vertical => inside = !inside,
                    Tile::NorthEast => elbow_from = Some(Direction::North),
                    Tile::SouthEast => elbow_from = Some(Direction::South),
                    Tile::NorthWest => {
                        if let Some(d) = &elbow_from {
                            if *d != Direction::North {
                                inside = !inside;
                            }
                        }
                    }
                    Tile::SouthWest => {
                        if let Some(d) = &elbow_from {
                            if *d != Direction::South {
                                inside = !inside;
                            }
                        }
                    }
                    Tile::Horizontal => {}
                    _ => unreachable!(),
                };
            }
        }

        inside_count
    }
}

#[cfg(test)]

mod day10tests {
    use super::*;

    const INPUT: &[&str] = &[".....", ".S-7.", ".|.|.", ".L-J.", "....."];
    const INPUT2: &[&str] = &["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];
    const INPUT3: &[&str] = &[
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ];
    const INPUT4: &[&str] = &[
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ];
    const INPUT5: &[&str] = &[
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];

    #[test]
    fn part1() {
        let day = AocDay10::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 4);

        let day = AocDay10::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 8);
    }

    #[test]
    fn part2() {
        let day = AocDay10::preprocessing(INPUT3.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 4);

        let day = AocDay10::preprocessing(INPUT4.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 8);

        let day = AocDay10::preprocessing(INPUT5.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 10);
    }
}
