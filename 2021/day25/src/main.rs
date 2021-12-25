use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Right,
    Down,
    Empty,
}

fn main() {
    let mut ocean_floor = {
        let mut ocean_floor = Vec::new();

        for line in std::io::stdin().lock().lines() {
            if let Ok(line) = line {
                let mut line_vec = Vec::new();
                for char in line.chars() {
                    line_vec.push(match char {
                        '>' => Cell::Right,
                        'v' => Cell::Down,
                        '.' => Cell::Empty,
                        _ => unreachable!("unknown char on input"),
                    });
                }
                ocean_floor.push(line_vec);
            }
        }

        ocean_floor
    };

    let mut step = 0;

    loop {
        let move_right = advance_state(&ocean_floor, Cell::Right);
        let move_down = advance_state(&move_right.1, Cell::Down);
        ocean_floor = move_down.1;

        step += 1;

        if !move_right.0 && !move_down.0 {
            break;
        }
    }

    println!("Part 1: {}", step);
}

fn advance_state(ocean_floor: &Vec<Vec<Cell>>, cell_type: Cell) -> (bool, Vec<Vec<Cell>>) {
    let mut new_floor: Vec<Vec<Cell>> = ocean_floor
        .iter()
        .map(|row| row.iter().map(|_| Cell::Empty).collect())
        .collect();
    let mut has_changed = false;

    for i in 0..ocean_floor.len() {
        let row = ocean_floor.get(i).unwrap();
        for j in 0..row.len() {
            let cell = row.get(j).unwrap();
            if *cell == cell_type {
                match cell {
                    Cell::Right => {
                        let new_pos = (j + 1) % row.len();
                        if *row.get(new_pos).unwrap() == Cell::Empty {
                            *new_floor.get_mut(i).unwrap().get_mut(new_pos).unwrap() = Cell::Right;
                            has_changed = true;
                        } else {
                            *new_floor.get_mut(i).unwrap().get_mut(j).unwrap() = Cell::Right;
                        }
                    }
                    Cell::Down => {
                        let new_pos = (i + 1) % ocean_floor.len();
                        if *ocean_floor.get(new_pos).unwrap().get(j).unwrap() == Cell::Empty {
                            *new_floor.get_mut(new_pos).unwrap().get_mut(j).unwrap() = Cell::Down;
                            has_changed = true;
                        } else {
                            *new_floor.get_mut(i).unwrap().get_mut(j).unwrap() = Cell::Down;
                        }
                    }
                    _ => {}
                }
            } else if *cell != Cell::Empty {
                *new_floor.get_mut(i).unwrap().get_mut(j).unwrap() = *cell;
            }
        }
    }

    (has_changed, new_floor)
}
