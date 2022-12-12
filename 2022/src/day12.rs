#[cfg(debug_assertions)]
use colored::Colorize;
use pathfinding::prelude::astar;

use crate::AocDay;

type Pos = (usize, usize);
type Map = Vec<Vec<u8>>;

// we're finding the path backwards, so the rules are inverted
fn get_possible_next_positions(map: &Map, pos: &Pos) -> Vec<(Pos, i32)> {
    let height = map.get(pos.1).and_then(|row| row.get(pos.0));
    if height.is_none() {
        return Vec::new();
    }
    let height = *height.unwrap();

    [(1i16, 0), (-1i16, 0), (0, 1i16), (0, -1i16)]
        .iter()
        .map(|vec| (pos.0 as i16 + vec.0, pos.1 as i16 + vec.1))
        .map(|vec| (vec.0 as usize, vec.1 as usize))
        .filter_map(|new_pos| {
            map.get(new_pos.1)
                .and_then(|row| row.get(new_pos.0))
                .filter(|x| **x + 1 >= height)
                .map(|_| (new_pos, 1))
        })
        .collect()
}

fn heuristic_fun(pos: &Pos, target_pos: &Pos) -> i32 {
    (pos.0 as i32 - target_pos.0 as i32).abs() + (pos.1 as i32 - target_pos.1 as i32).abs()
}

#[derive(Debug)]
pub struct AocDay12 {
    map: Map,
    start: Pos,
    end: Pos,
}

impl AocDay12 {
    fn get_height(&self, pos: &Pos) -> Option<u8> {
        self.map.get(pos.1).and_then(|row| row.get(pos.0)).cloned()
    }
    #[cfg(debug_assertions)]
    fn print_map(&self, path: &[Pos]) {
        println!();
        for y in 0..self.map.len() {
            for x in 0..self.map.get(y).map(|x| x.len()).unwrap_or(0) {
                print!("{}", {
                    let char = self
                        .get_height(&(x, y))
                        .map(|x| (x + b'a') as char)
                        .unwrap_or('.');
                    match path.contains(&(x, y)) {
                        true => String::from(char).yellow().bold(),
                        false => String::from(char).black(),
                    }
                });
            }
            println!();
        }
        println!();
    }
}

impl AocDay<i32, i32> for AocDay12 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut start = None;
        let mut end = None;
        let map = lines
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = Some((x, y));
                            0
                        }
                        'E' => {
                            end = Some((x, y));
                            b'z' - b'a'
                        }
                        _ => c as u8 - b'a',
                    })
                    .collect()
            })
            .collect();

        AocDay12 {
            map,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
    fn part1(&self) -> i32 {
        let (_path, steps) = astar(
            &self.end,
            |pos| get_possible_next_positions(&self.map, pos),
            |pos| heuristic_fun(pos, &self.start),
            |pos| *pos == self.start,
        )
        .expect("no goal reached");

        #[cfg(debug_assertions)]
        self.print_map(&_path);

        steps
    }
    fn part2(&self) -> i32 {
        let (_path, steps) = astar(
            &self.end,
            |pos| get_possible_next_positions(&self.map, pos),
            |pos| self.get_height(pos).unwrap_or(0) as i32,
            |pos| self.get_height(pos).filter(|x| *x == 0).is_some(),
        )
        .expect("no goal reached");

        #[cfg(debug_assertions)]
        self.print_map(&_path);

        steps
    }
}

#[cfg(test)]
mod day12tests {
    use super::*;

    const INPUT: &[&str] = &["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn part1() {
        let day = AocDay12::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 31);
    }

    #[test]
    fn part2() {
        let day = AocDay12::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 29);
    }
}
