use itertools::Itertools;

use crate::AocDay;

struct Pos {
    x: usize,
    y: usize,
}

pub struct AocDay11 {
    galaxies: Vec<Pos>,
    galaxies_x: Vec<usize>,
    galaxies_y: Vec<usize>,
}

impl AocDay11 {
    fn distance_between_galaxies<const EMPTY_FACTOR: usize>(&self) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                let distance_x = a.x.abs_diff(b.x);
                let a_x = self
                    .galaxies_x
                    .binary_search(&a.x)
                    .expect("galaxy not in list");
                let b_x = self
                    .galaxies_x
                    .binary_search(&b.x)
                    .expect("galaxy not in list");
                let filled_columns = a_x.abs_diff(b_x);

                let distance_y = a.y.abs_diff(b.y);
                let a_y = self
                    .galaxies_y
                    .binary_search(&a.y)
                    .expect("galaxy not in list");
                let b_y = self
                    .galaxies_y
                    .binary_search(&b.y)
                    .expect("galaxy not in list");
                let filled_rows = a_y.abs_diff(b_y);

                filled_columns
                    + (distance_x - filled_columns) * EMPTY_FACTOR
                    + filled_rows
                    + (distance_y - filled_rows) * EMPTY_FACTOR
            })
            .sum()
    }
}

impl AocDay<usize, usize> for AocDay11 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut galaxies = Vec::new();
        let mut galaxies_x = Vec::new();
        let mut galaxies_y = Vec::new();
        lines.enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' => {
                    galaxies.push(Pos { x, y });
                    galaxies_x.push(x);
                    galaxies_y.push(y);
                }
                '.' => {}
                _ => unreachable!(),
            })
        });

        galaxies_x.sort();
        galaxies_x.dedup();
        // galaxies_y is already sorted
        galaxies_y.dedup();

        AocDay11 {
            galaxies,
            galaxies_x,
            galaxies_y,
        }
    }
    fn part1(&self) -> usize {
        self.distance_between_galaxies::<2>()
    }
    fn part2(&self) -> usize {
        self.distance_between_galaxies::<1000000>()
    }
}

#[cfg(test)]
mod day11tests {
    use super::*;

    const INPUT: &[&str] = &[
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn part1() {
        let day = AocDay11::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 374);
    }

    #[test]
    fn part2() {
        let day = AocDay11::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.distance_between_galaxies::<10>(), 1030);
        assert_eq!(day.distance_between_galaxies::<100>(), 8410);
    }
}
