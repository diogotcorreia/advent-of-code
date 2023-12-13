use ndarray::Array3;

use crate::AocDay;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

struct Row {
    springs: Vec<Spring>,
    contiguous: Vec<u32>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DpState {
    spring_i: usize,
    contiguous_i: usize,
    curr_block_len: u32,
}

// [spring_i][contiguous_i][curr_block_len]
type DpCache = Array3<Option<u64>>;

impl Row {
    fn count_possibilities(&self) -> u64 {
        let max_block = self.contiguous.iter().max().cloned().unwrap_or(0) as usize;
        self.count_possibilities_inner(
            &DpState {
                spring_i: 0,
                contiguous_i: 0,
                curr_block_len: 0,
            },
            &mut Array3::default((
                self.springs.len() + 1,
                self.contiguous.len() + 1,
                max_block + 1,
            )),
        )
    }

    fn count_possibilities_inner(&self, state: &DpState, cache: &mut DpCache) -> u64 {
        let spring = self.springs.get(state.spring_i);
        let block_size = self.contiguous.get(state.contiguous_i);
        if spring.is_none() {
            if block_size
                .map(|&size| size == state.curr_block_len)
                .unwrap_or(true)
                && state.contiguous_i + 1 >= self.contiguous.len()
            {
                // if finished all blocks, valid path
                return 1;
            } else {
                return 0;
            }
        }

        let spring = spring.unwrap();
        let block_size = block_size.cloned().unwrap_or(0);

        let mut count = 0;

        if *spring != Spring::Operational {
            // damaged or unknown
            if block_size > state.curr_block_len {
                let new_state = DpState {
                    spring_i: state.spring_i + 1,
                    contiguous_i: state.contiguous_i,
                    curr_block_len: state.curr_block_len + 1,
                };
                if let Some(entry) = cache[(
                    new_state.spring_i,
                    new_state.contiguous_i,
                    new_state.curr_block_len as usize,
                )] {
                    count += entry;
                } else {
                    let count_inner = self.count_possibilities_inner(&new_state, cache);
                    cache[(
                        new_state.spring_i,
                        new_state.contiguous_i,
                        new_state.curr_block_len as usize,
                    )] = Some(count_inner);
                    count += count_inner;
                }
            }
        }
        if *spring != Spring::Damaged {
            // operational or unknown
            if state.curr_block_len == 0 || block_size == state.curr_block_len {
                let new_state = DpState {
                    spring_i: state.spring_i + 1,
                    // advance block iff curr_block_len > 0
                    contiguous_i: state.contiguous_i + 1.min(state.curr_block_len as usize),
                    curr_block_len: 0,
                };
                if let Some(entry) = cache[(
                    new_state.spring_i,
                    new_state.contiguous_i,
                    new_state.curr_block_len as usize,
                )] {
                    count += entry;
                } else {
                    let count_inner = self.count_possibilities_inner(&new_state, cache);
                    cache[(
                        new_state.spring_i,
                        new_state.contiguous_i,
                        new_state.curr_block_len as usize,
                    )] = Some(count_inner);
                    count += count_inner;
                }
            }
        }

        count
    }
}

pub struct AocDay12 {
    rows: Vec<Row>,
}

impl AocDay<u64, u64> for AocDay12 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let rows = lines
            .map(|line| {
                let (springs, cont) = line.split_once(' ').unwrap();

                Row {
                    springs: springs
                        .chars()
                        .map(|c| match c {
                            '.' => Spring::Operational,
                            '#' => Spring::Damaged,
                            '?' => Spring::Unknown,
                            _ => unreachable!("unknown spring"),
                        })
                        .collect(),
                    contiguous: cont.split(',').map(|i| i.parse().unwrap()).collect(),
                }
            })
            .collect();

        AocDay12 { rows }
    }
    fn part1(&self) -> u64 {
        self.rows.iter().map(|row| row.count_possibilities()).sum()
    }
    fn part2(&self) -> u64 {
        self.rows
            .iter()
            .map(|row| Row {
                springs: (0..9)
                    .flat_map(|i| {
                        if i % 2 == 0 {
                            row.springs.clone()
                        } else {
                            vec![Spring::Unknown]
                        }
                    })
                    .collect(),
                contiguous: row.contiguous.repeat(5),
            })
            .map(|row| row.count_possibilities())
            .sum()
    }
}

#[cfg(test)]
mod day12tests {
    use super::*;

    const INPUT: &[&str] = &[
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    #[test]
    fn part1() {
        let day = AocDay12::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 21);
    }

    #[test]
    fn part2() {
        let day = AocDay12::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 525152);
    }
}
