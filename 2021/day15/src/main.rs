use priority_queue::PriorityQueue;
use std::{cmp::Reverse, io::BufRead};

#[derive(Debug)]
struct Grid<T> {
    values: Vec<T>,
    row_length: i32,
}

impl<T> Grid<T> {
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || x >= self.row_length {
            None
        } else {
            self.values.get((x + y * self.row_length) as usize)
        }
    }

    fn row_count(&self) -> i32 {
        self.values.len() as i32 / self.row_length
    }
}

fn main() {
    let grid = {
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

    println!("Part 1: {}", dijkstra(&grid));
    println!("Part 2: {}", dijkstra(&expand_map(&grid)));
}

fn dijkstra(grid: &Grid<u8>) -> u32 {
    let mut cost: Vec<Option<u32>> = grid.values.iter().map(|_| None).collect();
    let mut queue = PriorityQueue::new();
    grid.values.iter().enumerate().for_each(|(i, _)| {
        queue.push(i, Reverse(u32::MAX));
    });
    let mut visited: Vec<bool> = grid.values.iter().map(|_| false).collect();

    *cost.get_mut(0).unwrap() = Some(0);
    queue.push_increase(0, Reverse(0));

    while !visited.last().unwrap() {
        let (min_index, priority) = queue.pop().unwrap();
        *visited.get_mut(min_index).unwrap() = true;
        let (x, y) = (
            min_index as i32 % grid.row_length,
            min_index as i32 / grid.row_length,
        );
        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let cmp = (x + offset.0, y + offset.1);
            let cmp_index: usize = (cmp.0 + cmp.1 * grid.row_length) as usize;
            if should_visit(cmp.0, cmp.1, grid.row_length, &visited) {
                let alt = priority.0 + *grid.get(cmp.0, cmp.1).unwrap() as u32;
                if let Some(val) = cost.get_mut(cmp_index) {
                    *val = match val {
                        None => Some(alt as u32),
                        Some(v) => Some((*v).min(alt as u32)),
                    };
                    queue.push_increase(cmp_index, Reverse(val.unwrap()));
                }
            }
        }
    }

    cost.last().unwrap().unwrap()
}

fn should_visit(x: i32, y: i32, row_length: i32, visited: &Vec<bool>) -> bool {
    if x < 0 || x >= row_length || y < 0 {
        false
    } else {
        !visited.get((y * row_length + x) as usize).unwrap_or(&true)
    }
}

fn expand_map(grid: &Grid<u8>) -> Grid<u8> {
    let size_x = grid.row_length * 5;
    let size_y = grid.row_count() * 5;
    let mut new_grid = Grid {
        values: Vec::new(),
        row_length: size_x,
    };

    for y in 0..size_y {
        for x in 0..size_x {
            let source_x = x % grid.row_length;
            let source_y = y % grid.row_count();
            let inc = (x / grid.row_length) + (y / grid.row_count());
            let source = grid.get(source_x, source_y).unwrap();

            new_grid
                .values
                .push(((*source as i32 + inc - 1) % 9 + 1) as u8);
        }
    }

    new_grid
}
