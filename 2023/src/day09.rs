use crate::AocDay;

fn find_all_non_zero_differences(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs = vec![sequence.to_vec()];
    loop {
        let new_diff: Vec<i32> = diffs
            .last()
            .expect("unreachable: no previous difference found")
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();

        if new_diff.iter().all(|&i| i == 0) {
            break;
        }

        diffs.push(new_diff);
    }

    diffs
}

fn find_next_numbers(sequence: &[i32]) -> i32 {
    find_all_non_zero_differences(sequence)
        .iter()
        .rev()
        .fold(0, |acc, row| row.last().unwrap_or(&0) + acc)
}

fn find_prev_numbers(sequence: &[i32]) -> i32 {
    find_all_non_zero_differences(sequence)
        .iter()
        .rev()
        .fold(0, |acc, row| row.first().unwrap_or(&0) - acc)
}

pub struct AocDay09 {
    numbers: Vec<Vec<i32>>,
}

impl AocDay<i32, i32> for AocDay09 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let numbers = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().expect("found non-integer in input"))
                    .collect()
            })
            .collect();

        AocDay09 { numbers }
    }
    fn part1(&self) -> i32 {
        self.numbers.iter().map(|seq| find_next_numbers(seq)).sum()
    }
    fn part2(&self) -> i32 {
        self.numbers.iter().map(|seq| find_prev_numbers(seq)).sum()
    }
}

#[cfg(test)]
mod day09tests {
    use super::*;

    const INPUT: &[&str] = &["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    #[test]
    fn part1() {
        let day = AocDay09::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 114);
    }

    #[test]
    fn part2() {
        let day = AocDay09::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 2);
    }
}
