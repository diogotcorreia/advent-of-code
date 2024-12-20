use aoc_common::{
    navigation::{Direction, Vec2D, VecSum},
    parsing::try_parse_2d_array,
    AocDay, DayError,
};
use aoc_common_macros::TryFromChar;
use itertools::Itertools;
use ndarray::Array2;

type Pos = Vec2D<usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromChar)]
enum Tile {
    #[char_repr = '#']
    Wall,
    #[char_repr = '.']
    Air,
    #[char_repr = 'O']
    Box,
    BoxLeft,
    BoxRight,
    #[char_repr = '@']
    Robot,
}

struct Move {
    direction: Direction,
}

impl TryFrom<char> for Move {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self {
                direction: Direction::North,
            }),
            '>' => Ok(Self {
                direction: Direction::East,
            }),
            'v' => Ok(Self {
                direction: Direction::South,
            }),
            '<' => Ok(Self {
                direction: Direction::West,
            }),
            _ => Err(DayError::GenericParseErr("unknown character for move")),
        }
    }
}

fn get_robot_pos(map: &Array2<Tile>) -> Option<Pos> {
    map.indexed_iter()
        .find(|(_, tile)| **tile == Tile::Robot)
        .map(|((y, x), _)| Pos::new(x, y))
}

fn maybe_move<const DRY_RUN: bool>(
    from: &Pos,
    direction: &Direction,
    map: &mut Array2<Tile>,
) -> Option<Pos> {
    let vec: Vec2D<isize> = Vec2D::from(direction.clone());
    let destination = from
        .vec_sum(&vec)
        .expect("overflow on vec sum (impossible due to walls)");

    let can_move = match map[&destination] {
        Tile::Wall => false,
        Tile::Air => true,
        Tile::Box => maybe_move::<DRY_RUN>(&destination, direction, map).is_some(),
        tile @ (Tile::BoxLeft | Tile::BoxRight) => match direction {
            Direction::North | Direction::South => {
                let matching_box_part_direction = match tile {
                    Tile::BoxLeft => Direction::East,
                    Tile::BoxRight => Direction::West,
                    _ => unreachable!(),
                };
                let matching_box_part = destination
                    .vec_sum(&Vec2D::<isize>::from(matching_box_part_direction))
                    .expect("overflow on vec sum (impossible due to walls)");

                maybe_move::<DRY_RUN>(&destination, direction, map).is_some()
                    && maybe_move::<DRY_RUN>(&matching_box_part, direction, map).is_some()
            }
            Direction::East | Direction::West => {
                maybe_move::<DRY_RUN>(&destination, direction, map).is_some()
            }
            _ => unreachable!(),
        },
        Tile::Robot => unreachable!(),
    };

    if can_move {
        if !DRY_RUN {
            map[&destination] = map[from];
            map[from] = Tile::Air;
        }
        Some(destination)
    } else {
        None
    }
}

fn calc_map_score(map: &Array2<Tile>) -> usize {
    map.indexed_iter()
        .filter(|(_, tile)| **tile == Tile::Box || **tile == Tile::BoxLeft)
        .map(|((y, x), _)| 100 * y + x)
        .sum()
}

fn expand_map(map: &Array2<Tile>) -> Array2<Tile> {
    Array2::from_shape_vec(
        (map.nrows(), map.ncols() * 2),
        map.iter()
            .flat_map(|t| match t {
                Tile::Wall => [Tile::Wall; 2],
                Tile::Air => [Tile::Air; 2],
                Tile::Box => [Tile::BoxLeft, Tile::BoxRight],
                Tile::Robot => [Tile::Robot, Tile::Air],
                Tile::BoxLeft | Tile::BoxRight => unreachable!("can't extend already extended map"),
            })
            .collect_vec(),
    )
    .expect("can't generate new expanded map")
}

pub struct AocDay15 {
    map: Array2<Tile>,
    moves: Vec<Move>,
    robot_pos: Pos,
}

impl AocDay<usize, usize> for AocDay15 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map = try_parse_2d_array(lines.by_ref().take_while(|l| !l.is_empty()))?;
        let moves = lines
            .flat_map(|line| line.chars().map(|c| c.try_into()).collect_vec())
            .process_results(|it| it.collect_vec())?;

        let robot_pos = get_robot_pos(&map).ok_or(DayError::GenericParseErr("no robot on map"))?;

        Ok(AocDay15 {
            map,
            moves,
            robot_pos,
        })
    }
    fn part1(&self) -> usize {
        let (map, _) = self.moves.iter().fold(
            (self.map.clone(), self.robot_pos.clone()),
            |(mut map, robot_pos), mov| {
                let new_pos =
                    maybe_move::<false>(&robot_pos, &mov.direction, &mut map).unwrap_or(robot_pos);

                (map, new_pos)
            },
        );

        calc_map_score(&map)
    }
    fn part2(&self) -> usize {
        let map = expand_map(&self.map);
        let robot_pos = get_robot_pos(&map).expect("can't find robot pos in extended map");

        let (map, _) = self
            .moves
            .iter()
            .fold((map, robot_pos), |(mut map, robot_pos), mov| {
                let can_move = maybe_move::<true>(&robot_pos, &mov.direction, &mut map).is_some();
                let new_pos = can_move
                    .then(|| maybe_move::<false>(&robot_pos, &mov.direction, &mut map))
                    .flatten()
                    .unwrap_or(robot_pos);

                (map, new_pos)
            });

        calc_map_score(&map)
    }
}

#[cfg(test)]
mod day15tests {
    use super::*;

    const INPUT_SMALL: &[&str] = &[
        "########",
        "#..O.O.#",
        "##@.O..#",
        "#...O..#",
        "#.#.O..#",
        "#...O..#",
        "#......#",
        "########",
        "",
        "<^^>>>vv<v>>v<<",
    ];
    const INPUT: &[&str] = &[
        "##########",
        "#..O..O.O#",
        "#......O.#",
        "#.OO..O.O#",
        "#..O@..O.#",
        "#O#..O...#",
        "#O..O..O.#",
        "#.OO.O.OO#",
        "#....O...#",
        "##########",
        "",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ];

    #[test]
    fn part1_small() -> Result<(), DayError> {
        let day = AocDay15::preprocessing_tests(INPUT_SMALL)?;
        assert_eq!(day.part1(), 2028);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay15::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 10092);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay15::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 9021);
        Ok(())
    }
}
