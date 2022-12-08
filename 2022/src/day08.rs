use std::collections::HashSet;

use crate::AocDay;

#[derive(Debug)]
struct Forest(Vec<u8>, usize);

impl Forest {
    fn new(side: usize) -> Forest {
        Forest(Vec::new(), side)
    }

    fn push(&mut self, height: u8) {
        self.0.push(height);
    }

    fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = (usize, &u8)> {
        self.0.iter().enumerate().skip(row * self.1).take(self.1)
    }

    fn iter_col(&self, col: usize) -> impl DoubleEndedIterator<Item = (usize, &u8)> {
        self.0.iter().enumerate().skip(col).step_by(self.1)
    }

    fn viewing_distance<'a>(
        &self,
        height: u8,
        iter: impl Iterator<Item = (usize, &'a u8)>,
    ) -> usize {
        // need this because take_while is non-inclusive
        let mut last_tree: Option<u8> = None;
        iter.take_while(|(_, h)| match last_tree {
            Some(x) if x >= height => false,
            _ => {
                last_tree = Some(**h);
                true
            }
        })
        .count()
    }

    fn scenic_score(&self, pos: usize) -> usize {
        let row = pos / self.1;
        let col = pos % self.1;
        let height = *self.0.get(pos).expect("tree index out of bounds");

        self.viewing_distance(height, self.iter_row(row).skip(col + 1))
            * self.viewing_distance(height, self.iter_row(row).rev().skip(self.1 - col))
            * self.viewing_distance(height, self.iter_col(col).skip(row + 1))
            * self.viewing_distance(height, self.iter_col(col).rev().skip(self.1 - row))
    }
}

fn append_visible_indexes<'a>(
    visible_trees: &mut HashSet<usize>,
    iter: impl Iterator<Item = (usize, &'a u8)>,
) {
    let mut max_height: Option<u8> = None;

    for (index, height) in iter {
        match max_height {
            None => {
                visible_trees.insert(index);
                max_height = Some(*height);
            }
            Some(9) => break,
            Some(x) if x < *height => {
                visible_trees.insert(index);
                max_height = Some(*height);
            }
            _ => {}
        }
    }
}

pub struct AocDay08 {
    forest: Forest,
}

impl AocDay<usize, usize> for AocDay08 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.peekable();
        let side = lines.peek().expect("input must have one line").len();
        let mut forest = Forest::new(side);

        lines
            .flat_map(|x| x.as_bytes().iter().map(|c| *c - b'0').collect::<Vec<_>>())
            .for_each(|x| forest.push(x));

        assert_eq!(forest.0.len(), forest.1 * forest.1);

        return AocDay08 { forest };
    }
    fn part1(&self) -> usize {
        let mut visible_trees = HashSet::new();
        for i in 0..self.forest.1 {
            append_visible_indexes(&mut visible_trees, self.forest.iter_row(i));
            append_visible_indexes(&mut visible_trees, self.forest.iter_row(i).rev());
            append_visible_indexes(&mut visible_trees, self.forest.iter_col(i));
            append_visible_indexes(&mut visible_trees, self.forest.iter_col(i).rev());
        }

        visible_trees.len()
    }
    fn part2(&self) -> usize {
        (0..self.forest.0.len())
            .into_iter()
            .map(|i| self.forest.scenic_score(i))
            .max()
            .expect("forest must not be empty")
    }
}

#[cfg(test)]
mod day08tests {
    use super::*;

    const INPUT: &'static [&'static str] = &["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn part1() {
        let day = AocDay08::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 21);
    }

    #[test]
    fn part2() {
        let day = AocDay08::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 8);
    }
}
