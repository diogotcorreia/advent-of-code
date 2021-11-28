use std::collections::HashMap;
use std::io;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn center() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn get_adjacent_positions(&self) -> Vec<Pos> {
        let mut vec = Vec::new();
        for delta_x in -1..2 {
            for delta_y in -1..2 {
                if delta_x != 0 || delta_y != 0 {
                    vec.push(Pos {
                        x: self.x + delta_x,
                        y: self.y + delta_y,
                    });
                }
            }
        }
        vec
    }

    fn distance_to_center(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: i32 = input.trim().parse().expect("Input not a number");

    let mut pos = Pos::center();
    let mut direction = Direction::Right;
    let mut square_size = 0;
    let mut current_num = 1;

    while input > current_num {
        direction = calculate_next_direction(&direction, &mut pos, &mut square_size);
        current_num += 1;
    }

    println!("Part 1: {}", pos.distance_to_center());

    pos = Pos::center();
    direction = Direction::Right;
    current_num = 1;
    square_size = 0;

    let mut calculated_values = HashMap::new();
    calculated_values.insert(pos.clone(), 1);

    while input >= current_num {
        direction = calculate_next_direction(&direction, &mut pos, &mut square_size);

        current_num = pos
            .get_adjacent_positions()
            .iter()
            .filter_map(|p| calculated_values.get(p))
            .sum();

        calculated_values.insert(pos.clone(), current_num);
    }

    println!("Part 2: {}", calculated_values.get(&pos).unwrap_or(&0));
}

fn calculate_next_direction(
    direction: &Direction,
    pos: &mut Pos,
    square_size: &mut i32,
) -> Direction {
    match direction {
        Direction::Up => {
            pos.y += 1;
            if pos.y == *square_size {
                Direction::Left
            } else {
                Direction::Up
            }
        }
        Direction::Down => {
            pos.y -= 1;
            if pos.y == -*square_size {
                Direction::Right
            } else {
                Direction::Down
            }
        }
        Direction::Left => {
            pos.x -= 1;
            if pos.x == -*square_size {
                Direction::Down
            } else {
                Direction::Left
            }
        }
        Direction::Right => {
            pos.x += 1;
            if pos.x == *square_size + 1 {
                *square_size += 1;
                Direction::Up
            } else {
                Direction::Right
            }
        }
    }
}
