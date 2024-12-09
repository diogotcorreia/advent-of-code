use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_common::{AocDay, DayError};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum FsBlock {
    Empty,
    File(usize),
}

struct FsGroupedBlock {
    index: usize,
    size: usize,
    id: usize,
}

struct FreeSpaceTable {
    // empty space has size 0-9, but we don't care about zeros
    table: [BinaryHeap<Reverse<usize>>; 9],
}

impl FreeSpaceTable {
    fn init(fs: &[FsGroupedBlock]) -> Self {
        let mut table = [0u8; 9].map(|_| BinaryHeap::new());
        for (prev, next) in fs.iter().tuple_windows() {
            let i = prev.index + prev.size;
            let size = next.index - i;
            if size != 0 {
                table[size - 1].push(Reverse(i));
            }
        }

        Self { table }
    }

    fn alloc_space(&mut self, max_i: usize, size: usize) -> Option<usize> {
        let (i, alloc_size) = self
            .table
            .iter()
            .enumerate()
            .skip(size - 1)
            .flat_map(|(table_i, row)| row.peek().map(|j| (j.0, table_i + 1)))
            .sorted()
            .next()
            .filter(|(i, _)| *i < max_i)?;

        let remaining_space = alloc_size - size;
        if remaining_space > 0 {
            self.table[remaining_space - 1].push(Reverse(i + size));
        }
        self.table[alloc_size - 1].pop();

        Some(i)
    }
}

pub struct AocDay09 {
    fs: Vec<FsBlock>,
    fs_grouped: Vec<FsGroupedBlock>,
}

impl AocDay<usize, usize> for AocDay09 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let mut fs = Vec::new();
        let mut fs_grouped = Vec::new();
        let mut is_empty = false;
        let mut file_id = 0;
        for c in lines.next().unwrap().chars() {
            let size = c.to_digit(10).unwrap() as usize;
            let block = if is_empty {
                is_empty = false;
                FsBlock::Empty
            } else {
                let block = FsBlock::File(file_id);
                fs_grouped.push(FsGroupedBlock {
                    index: fs.len(),
                    size,
                    id: file_id,
                });
                is_empty = true;
                file_id += 1;
                block
            };

            (0..size).for_each(|_| fs.push(block.clone()));
        }

        Ok(AocDay09 { fs, fs_grouped })
    }
    fn part1(&self) -> usize {
        let mut i = 0;
        let mut j = self.fs.len();
        let mut result = 0;

        while i < j {
            if let FsBlock::File(id) = self.fs[i] {
                result += id * i;
            } else {
                j -= 1;
                while let FsBlock::Empty = self.fs[j]
                    && i < j
                {
                    j -= 1;
                }

                if let FsBlock::File(id) = self.fs[j] {
                    result += id * i;
                }
            }
            i += 1;
        }

        result
    }
    fn part2(&self) -> usize {
        let mut table = FreeSpaceTable::init(&self.fs_grouped);

        self.fs_grouped
            .iter()
            .rev()
            .flat_map(|block| {
                let new_i = table
                    .alloc_space(block.index, block.size)
                    .unwrap_or(block.index);

                (new_i..).take(block.size).map(|i| i * block.id)
            })
            .sum()
    }
}

#[cfg(test)]
mod day09tests {
    use super::*;

    const INPUT: &[&str] = &["2333133121414131402"];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay09::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 1928);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay09::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 2858);
        Ok(())
    }
}
