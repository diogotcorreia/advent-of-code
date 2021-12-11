use std::io::BufRead;

#[derive(Debug)]
struct Grid {
    values: Vec<u8>,
    row_length: i32,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<&u8> {
        if x < 0 || x >= self.row_length {
            None
        } else {
            self.values.get((x + y * self.row_length) as usize)
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        if x < 0 || x >= self.row_length {
            None
        } else {
            self.values.get_mut((x + y * self.row_length) as usize)
        }
    }

    fn row_count(&self) -> i32 {
        self.values.len() as i32 / self.row_length
    }
}

fn main() {
    let mut grid = {
        let mut row_length = 0;
        let vec: Vec<u8> = std::io::stdin()
            .lock()
            .lines()
            .flat_map(|line| {
                let line = line.expect("failed to read stdin");
                let line = line.trim();
                row_length = line.len() as i32;

                line.chars()
                    .map(|c| String::from(c).parse::<u8>().expect("not a number"))
                    .collect::<Vec<u8>>()
            })
            .collect();
        Grid {
            values: vec,
            row_length,
        }
    };

    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += execute_step(&mut grid);
    }

    let mut step = 100;

    loop {
        step += 1;
        if execute_step(&mut grid) == grid.values.len() as i32 {
            break;
        }
    }

    println!("Part 1: {}", flash_count);
    println!("Part 2: {}", step);
}

fn execute_step(grid: &mut Grid) -> i32 {
    // increase level of all by 1
    grid.values.iter_mut().for_each(|energy| *energy += 1);

    // find energies > 9
    for x in 0..grid.row_length {
        for y in 0..grid.row_count() {
            if let Some(&energy) = grid.get(x, y) {
                if energy == 10 {
                    flash_octupus(grid, x, y);
                }
            }
        }
    }

    // reset energies of octupus that have flashed and count them
    grid.values.iter_mut().fold(0, |flash_count, energy| {
        if *energy > 9 {
            *energy = 0;
            flash_count + 1
        } else {
            flash_count
        }
    })
}

fn flash_octupus(grid: &mut Grid, x: i32, y: i32) {
    // set energy of octupus to 11, so it doesn't flash again this round
    if let Some(energy) = grid.get_mut(x, y) {
        *energy += 1;
    }

    for x_delta in -1..=1 {
        for y_delta in -1..=1 {
            if x_delta == 0 && y_delta == 0 {
                continue;
            }

            if let Some(energy_adjacent) = grid.get_mut(x + x_delta, y + y_delta) {
                if *energy_adjacent <= 9 {
                    *energy_adjacent += 1;
                }
                if *energy_adjacent == 10 {
                    flash_octupus(grid, x + x_delta, y + y_delta);
                }
            }
        }
    }
}
