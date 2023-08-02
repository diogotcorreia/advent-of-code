use std::{
    ops::{Add, Neg, Range},
    str::FromStr,
};

use crate::AocDay;

struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn simulate_movement(
        &self,
        position: &Position,
        movement: &Movement,
        wrap_around_stategy: &dyn WrapAroundStrategy,
    ) -> Position {
        match movement {
            Movement::Rotate { direction } => Position {
                row: position.row,
                col: position.col,
                facing: position.facing + *direction,
            },
            Movement::Move { steps } => {
                let mut current_pos = position.clone();
                let mut limits = self.get_limits(&current_pos);

                for _ in 0..*steps {
                    let next_pos = current_pos.next_position(&limits);
                    let recalculate_limits = next_pos.is_none();
                    let next_pos = next_pos
                        .unwrap_or_else(|| wrap_around_stategy.wrap_around(&current_pos, &limits));

                    if self.is_wall(&next_pos) {
                        break;
                    }
                    current_pos = next_pos;

                    if recalculate_limits {
                        limits = self.get_limits(&current_pos);
                    }
                }

                current_pos
            }
        }
    }

    fn get_start_position(&self) -> Position {
        Position {
            row: 0,
            col: self.rows.first().unwrap().limits.start,
            facing: Direction::East,
        }
    }

    fn get_limits(&self, position: &Position) -> Range<u32> {
        match position.facing {
            Direction::East | Direction::West => self.get_row_limits(position.row),
            Direction::North | Direction::South => self.get_col_limits(position.col, position.row),
        }
    }

    fn get_row_limits(&self, row: u32) -> Range<u32> {
        self.rows.get(row as usize).unwrap().limits.clone()
    }

    fn get_col_limits(&self, col: u32, current_row: u32) -> Range<u32> {
        let lower_limit =
            self.rows[..=current_row as usize].partition_point(|row| !row.limits.contains(&col));
        let upper_limit =
            self.rows[lower_limit..].partition_point(|row| row.limits.contains(&col)) + lower_limit;

        lower_limit as u32..upper_limit as u32
    }

    fn is_wall(&self, position: &Position) -> bool {
        self.rows
            .get(position.row as usize)
            .map(|row| row.is_wall(position.col))
            .unwrap_or(false)
    }
}

fn compute_cube_size(rows: &Vec<Row>) -> u32 {
    // 2d representation of cube is always 4:3 or 3:4
    let max_row = rows.len() as u32;
    let max_col = rows.iter().map(|row| row.limits.end).max().unwrap_or(0);

    if max_row > max_col {
        max_row / 4
    } else {
        max_col / 4
    }
}

fn reduce_rows_to_cube_faces_mask(rows: &[Row], cube_size: u32) -> [[bool; 4]; 4] {
    // reduce the map to a simple array (2d version of cube always fits in a 4x4 grid)
    let mut faces_mask = [[false; 4]; 4];
    for (row_index, row) in rows.iter().enumerate().step_by(cube_size as usize) {
        for col_square in 0..4 {
            if row.limits.contains(&(col_square * cube_size)) {
                faces_mask[row_index / cube_size as usize][col_square as usize] = true;
            }
        }
    }

    faces_mask
}

fn compute_cube_faces(rows: &[Row], cube_size: u32) -> Vec<CubeFace> {
    let faces_mask = reduce_rows_to_cube_faces_mask(rows, cube_size);

    let top_face_index = faces_mask[0].iter().position(|&has_face| has_face).unwrap() as u32; // unreachable
    let top_face = CubeFace {
        top_left_corner_2d: (0, top_face_index * cube_size),
        face_side: FaceSide::Top,
        rotation: Rotation::None,
    };

    // recursive
    fn find_adjacent_faces(
        face: &CubeFace,
        cube_size: u32,
        faces_mask: &[[bool; 4]; 4],
        from_side: Option<&Direction>,
    ) -> Vec<CubeFace> {
        let mask_position = (
            (face.top_left_corner_2d.0 / cube_size) as i32,
            (face.top_left_corner_2d.1 / cube_size) as i32,
        );

        let mut new_faces = vec![];

        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if from_side.filter(|&&side| side == direction).is_some() {
                // don't go back to where we came from
                continue;
            }

            let vector = direction.as_vector();
            let adjacent_pos = (mask_position.0 + vector.0, mask_position.1 + vector.1);

            if adjacent_pos.0 < 0 || adjacent_pos.1 < 0 {
                continue;
            }

            let is_face = *faces_mask
                .get(adjacent_pos.0 as usize)
                .and_then(|array| array.get(adjacent_pos.1 as usize))
                .unwrap_or(&false);

            if !is_face {
                continue;
            }

            let canonical_direction = direction + face.rotation;
            let (face_side, rotation) = face
                .face_side
                .get_params_of_adjacent_face(&canonical_direction);

            new_faces.push(CubeFace {
                top_left_corner_2d: (
                    adjacent_pos.0 as u32 * cube_size,
                    adjacent_pos.1 as u32 * cube_size,
                ),
                face_side,
                rotation: rotation + face.rotation,
            });

            // recursively find all adjacent faces
            new_faces.extend(find_adjacent_faces(
                new_faces.last().unwrap(),
                cube_size,
                faces_mask,
                Some(&(direction + Rotation::UpsideDown)),
            ));
        }

        new_faces
    }

    let mut faces = vec![top_face];
    faces.extend(find_adjacent_faces(
        faces.first().unwrap(),
        cube_size,
        &faces_mask,
        None,
    ));

    faces
}

#[derive(Debug)]
struct Row {
    limits: Range<u32>,
    walls: Vec<u32>,
}

impl FromStr for Row {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper_limit = s.len() as u32;
        let lower_limit = s.find(['.', '#']).ok_or(ParseErr)? as u32;

        let walls = s
            .chars()
            .enumerate()
            .filter(|(_i, v)| *v == '#')
            .map(|(i, _v)| i as u32)
            .collect();

        Ok(Self {
            limits: lower_limit..upper_limit,
            walls,
        })
    }
}

impl Row {
    fn is_wall(&self, col: u32) -> bool {
        self.walls.binary_search(&col).is_ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn password_ordinal(&self) -> u32 {
        match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }

    fn as_vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl Add<Rotation> for Direction {
    type Output = Self;

    fn add(self, other: Rotation) -> Self::Output {
        match (self, other) {
            (_, Rotation::None) => self,

            (Direction::North, Rotation::Clockwise) => Direction::East,
            (Direction::North, Rotation::UpsideDown) => Direction::South,
            (Direction::North, Rotation::Counterclockwise) => Direction::West,

            (Direction::East, Rotation::Clockwise) => Direction::South,
            (Direction::East, Rotation::UpsideDown) => Direction::West,
            (Direction::East, Rotation::Counterclockwise) => Direction::North,

            (Direction::South, Rotation::Clockwise) => Direction::West,
            (Direction::South, Rotation::UpsideDown) => Direction::North,
            (Direction::South, Rotation::Counterclockwise) => Direction::East,

            (Direction::West, Rotation::Clockwise) => Direction::North,
            (Direction::West, Rotation::UpsideDown) => Direction::East,
            (Direction::West, Rotation::Counterclockwise) => Direction::South,
        }
    }
}

#[derive(Debug)]
enum Movement {
    Rotate { direction: Rotation },
    Move { steps: u32 },
}

#[derive(Debug, Clone)]
struct Position {
    row: u32,
    col: u32,
    facing: Direction,
}

fn clamp_wrap_around(value: i32, limits: &Range<u32>) -> u32 {
    (((value - limits.start as i32).rem_euclid(limits.len() as i32)) + limits.start as i32) as u32
}
impl Position {
    fn next_position(&self, limits: &Range<u32>) -> Option<Position> {
        fn inside_limits_or_none(value: i32, limits: &Range<u32>) -> Option<u32> {
            Some(value as u32).filter(|v| limits.contains(v))
        }

        let vector = self.facing.as_vector();
        match self.facing {
            Direction::North | Direction::South => {
                inside_limits_or_none(self.row as i32 + vector.0, limits).map(|row| Position {
                    row,
                    col: self.col,
                    facing: self.facing,
                })
            }
            Direction::East | Direction::West => {
                inside_limits_or_none(self.col as i32 + vector.1, limits).map(|col| Position {
                    row: self.row,
                    col,
                    facing: self.facing,
                })
            }
        }
    }

    fn calculate_password(&self) -> u32 {
        1000 * (self.row + 1) + 4 * (self.col + 1) + self.facing.password_ordinal()
    }
}

#[derive(Debug)]
struct ParseErr;

trait WrapAroundStrategy {
    fn wrap_around(&self, current_pos: &Position, limits: &Range<u32>) -> Position;
}

struct WrapAroundLinear;

impl WrapAroundStrategy for WrapAroundLinear {
    fn wrap_around(&self, current_pos: &Position, limits: &Range<u32>) -> Position {
        let (row, col) = match current_pos.facing {
            Direction::East => (current_pos.row, limits.start),
            Direction::West => (current_pos.row, limits.end - 1),
            Direction::North => (limits.end - 1, current_pos.col),
            Direction::South => (limits.start, current_pos.col),
        };

        Position {
            row,
            col,
            facing: current_pos.facing,
        }
    }
}

struct WrapAroundCube {
    cube_faces: Vec<CubeFace>,
    cube_size: u32,
}

impl WrapAroundStrategy for WrapAroundCube {
    fn wrap_around(&self, position: &Position, _limits: &Range<u32>) -> Position {
        let cube_size = self.cube_size;
        let current_face = self
            .cube_faces
            .iter()
            .find(|face| face.contains_position(position, cube_size))
            .expect("position must be in a face");

        let canonical_direction = position.facing + current_face.rotation;
        let (target_face_side, target_face_side_rotation) = current_face
            .face_side
            .get_params_of_adjacent_face(&canonical_direction);

        let target_face = self
            .cube_faces
            .iter()
            .find(|face| face.face_side == target_face_side)
            .expect("target face must exist");

        let direction_vector = canonical_direction.as_vector();
        let local_coordinates = (position.row % cube_size, position.col % cube_size);
        let local_coordinates =
            (current_face.rotation).rotate_local_coordinates(&local_coordinates, cube_size);

        let local_new_coordinates = (
            clamp_wrap_around(
                local_coordinates.0 as i32 + direction_vector.0,
                &(0..cube_size),
            ),
            clamp_wrap_around(
                local_coordinates.1 as i32 + direction_vector.1,
                &(0..cube_size),
            ),
        );
        let local_new_coordinates =
            target_face_side_rotation.rotate_local_coordinates(&local_new_coordinates, cube_size);

        let new_coordinates =
            (-target_face.rotation).rotate_local_coordinates(&local_new_coordinates, cube_size);
        let new_coordinates = (
            target_face.top_left_corner_2d.0 + new_coordinates.0,
            target_face.top_left_corner_2d.1 + new_coordinates.1,
        );

        Position {
            row: new_coordinates.0,
            col: new_coordinates.1,
            facing: position.facing
                + (current_face.rotation + target_face_side_rotation + (-target_face.rotation)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CubeFace {
    top_left_corner_2d: (u32, u32),
    face_side: FaceSide,
    rotation: Rotation,
}

impl CubeFace {
    fn contains_position(self, position: &Position, cube_size: u32) -> bool {
        let row_range = self.top_left_corner_2d.0..(self.top_left_corner_2d.0 + cube_size);
        let col_range = self.top_left_corner_2d.1..(self.top_left_corner_2d.1 + cube_size);

        row_range.contains(&position.row) && col_range.contains(&position.col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaceSide {
    Top,
    Front,
    Right,
    Back,
    Left,
    Bottom,
}

impl FaceSide {
    // Canonic rotation (i.e. how they would appear if none of them were rotated) of faces is as follows:
    // The top face is the anchor and therefore is never rotated
    // The side faces have their top against the top face
    // The bottom face has its top against the front face
    fn get_params_of_adjacent_face(&self, direction: &Direction) -> (FaceSide, Rotation) {
        match (self, direction) {
            (FaceSide::Top, Direction::East) => (FaceSide::Right, Rotation::Clockwise),
            (FaceSide::Top, Direction::South) => (FaceSide::Front, Rotation::None),
            (FaceSide::Top, Direction::West) => (FaceSide::Left, Rotation::Counterclockwise),
            (FaceSide::Top, Direction::North) => (FaceSide::Back, Rotation::UpsideDown),

            (FaceSide::Front, Direction::East) => (FaceSide::Right, Rotation::None),
            (FaceSide::Front, Direction::South) => (FaceSide::Bottom, Rotation::None),
            (FaceSide::Front, Direction::West) => (FaceSide::Left, Rotation::None),
            (FaceSide::Front, Direction::North) => (FaceSide::Top, Rotation::None),

            (FaceSide::Right, Direction::East) => (FaceSide::Back, Rotation::None),
            (FaceSide::Right, Direction::South) => (FaceSide::Bottom, Rotation::Clockwise),
            (FaceSide::Right, Direction::West) => (FaceSide::Front, Rotation::None),
            (FaceSide::Right, Direction::North) => (FaceSide::Top, Rotation::Counterclockwise),

            (FaceSide::Back, Direction::East) => (FaceSide::Left, Rotation::None),
            (FaceSide::Back, Direction::South) => (FaceSide::Bottom, Rotation::UpsideDown),
            (FaceSide::Back, Direction::West) => (FaceSide::Right, Rotation::None),
            (FaceSide::Back, Direction::North) => (FaceSide::Top, Rotation::UpsideDown),

            (FaceSide::Left, Direction::East) => (FaceSide::Front, Rotation::None),
            (FaceSide::Left, Direction::South) => (FaceSide::Bottom, Rotation::Counterclockwise),
            (FaceSide::Left, Direction::West) => (FaceSide::Back, Rotation::None),
            (FaceSide::Left, Direction::North) => (FaceSide::Top, Rotation::Clockwise),

            (FaceSide::Bottom, Direction::East) => (FaceSide::Right, Rotation::Counterclockwise),
            (FaceSide::Bottom, Direction::South) => (FaceSide::Back, Rotation::UpsideDown),
            (FaceSide::Bottom, Direction::West) => (FaceSide::Left, Rotation::Clockwise),
            (FaceSide::Bottom, Direction::North) => (FaceSide::Front, Rotation::None),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    None,
    Clockwise,
    UpsideDown,
    Counterclockwise,
}

impl Add for Rotation {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::None, _) => other,
            (_, Self::None) => self,

            (Self::Clockwise, Self::Clockwise) => Self::UpsideDown,
            (Self::Clockwise, Self::UpsideDown) => Self::Counterclockwise,
            (Self::Clockwise, Self::Counterclockwise) => Self::None,

            (Self::UpsideDown, Self::Clockwise) => Self::Counterclockwise,
            (Self::UpsideDown, Self::UpsideDown) => Self::None,
            (Self::UpsideDown, Self::Counterclockwise) => Self::Clockwise,

            (Self::Counterclockwise, Self::Clockwise) => Self::None,
            (Self::Counterclockwise, Self::UpsideDown) => Self::Clockwise,
            (Self::Counterclockwise, Self::Counterclockwise) => Self::UpsideDown,
        }
    }
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::None | Self::UpsideDown => self,
            Self::Clockwise => Self::Counterclockwise,
            Self::Counterclockwise => Self::Clockwise,
        }
    }
}

impl FromStr for Rotation {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Clockwise),
            "L" => Ok(Self::Counterclockwise),
            _ => Err(ParseErr),
        }
    }
}

impl Rotation {
    fn rotate_local_coordinates(&self, coordinates: &(u32, u32), cube_size: u32) -> (u32, u32) {
        let (sin, cos) = match self {
            Self::None => (0, 1),
            Self::Clockwise => (-1, 0),
            Self::UpsideDown => (0, -1),
            Self::Counterclockwise => (1, 0),
        };

        let cube_size = cube_size as i32 * 3 / 2;

        // new pos = T(-side/2, -side/2) -> Rotation -> T(side/2, side/2)

        // triple coordinates to avoid working with floats and center them as points
        let coordinates = (coordinates.0 as i32 * 3, coordinates.1 as i32 * 3);
        // sum 1 to "center" square
        // move to center
        let coordinates = (coordinates.0 - cube_size + 1, coordinates.1 - cube_size + 1);
        // rotate
        let new_pos = (
            coordinates.0 * cos - coordinates.1 * sin,
            coordinates.0 * sin + coordinates.1 * cos,
        );
        // move to corner
        let new_pos = (new_pos.0 + cube_size - 1, new_pos.1 + cube_size - 1);

        (new_pos.0 as u32 / 3, new_pos.1 as u32 / 3)
    }
}

pub struct AocDay22 {
    map: Map,
    movements: Vec<Movement>,
}

impl AocDay<u32, u32> for AocDay22 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let rows = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.parse().expect("failed to parse line"))
            .collect();

        let movements_str = lines.next().expect("no movement list provided");
        let mut movements = Vec::new();
        let mut last = 0;
        for (index, matched) in movements_str.match_indices(['L', 'R']) {
            if last != index {
                movements.push(Movement::Move {
                    steps: movements_str[last..index]
                        .parse()
                        .expect("failed to parse movement step count"),
                });
            }
            movements.push(Movement::Rotate {
                direction: matched
                    .parse()
                    .expect("failed to parse movement rotation direction"),
            });
            last = index + matched.len();
        }
        if last < movements_str.len() {
            movements.push(Movement::Move {
                steps: movements_str[last..]
                    .parse()
                    .expect("failed to parse movement step count"),
            });
        }

        AocDay22 {
            map: Map { rows },
            movements,
        }
    }
    fn part1(&self) -> u32 {
        let mut position = self.map.get_start_position();

        for movement in &self.movements {
            position = self
                .map
                .simulate_movement(&position, movement, &WrapAroundLinear);
        }

        position.calculate_password()
    }
    fn part2(&self) -> u32 {
        let mut position = self.map.get_start_position();

        let cube_size = compute_cube_size(&self.map.rows);
        let cube_faces = compute_cube_faces(&self.map.rows, cube_size);

        let strategy = WrapAroundCube {
            cube_faces,
            cube_size,
        };

        for movement in &self.movements {
            position = self.map.simulate_movement(&position, movement, &strategy);
        }

        position.calculate_password()
    }
}

#[cfg(test)]
mod day22tests {
    use super::*;

    const INPUT: &[&str] = &[
        "        ...#",
        "        .#..",
        "        #...",
        "        ....",
        "...#.......#",
        "........#...",
        "..#....#....",
        "..........#.",
        "        ...#....",
        "        .....#..",
        "        .#......",
        "        ......#.",
        "",
        "10R5L5R10L4R5L5",
    ];

    #[test]
    fn part1() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 6032);
    }

    #[test]
    fn part2() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 5031);
    }

    #[test]
    fn test_reduce_to_faces_mask() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        let mask = reduce_rows_to_cube_faces_mask(&day.map.rows, 4);

        assert_eq!(mask[0], [false, false, true, false]);
        assert_eq!(mask[1], [true, true, true, false]);
        assert_eq!(mask[2], [false, false, true, true]);
        assert_eq!(mask[3], [false, false, false, false]);
    }

    #[test]
    fn test_compute_cube_size() {
        let day = AocDay22::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(compute_cube_size(&day.map.rows), 4);
    }
}
