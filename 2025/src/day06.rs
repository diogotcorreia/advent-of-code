use aoc_common::{AocDay, DayError};
use aoc_common_macros::TryFromChar;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, TryFromChar)]
enum Operator {
    #[char_repr = '+']
    Sum,
    #[char_repr = '*']
    Product,
}

pub struct AocDay06 {
    rows: Array2<u32>,
    columns: Vec<u32>,
    operations: Vec<Operator>,
}

impl AocDay<u64, u64> for AocDay06 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let mut entries = vec![];
        let mut operations = vec![];
        let mut lines = lines.peekable();
        let first_line = lines
            .peek()
            .ok_or(DayError::GenericParseErr("input is empty"))?;
        let columns = first_line.split_whitespace().collect_vec().len();
        let mut rtl_entries = vec![0u32; first_line.len()];
        for line in lines {
            let mut iter = line.split_whitespace().peekable();
            let is_number_row = iter
                .peek()
                .filter(|entry| entry.parse::<u32>().is_ok())
                .is_some();
            if is_number_row {
                let mut row = iter
                    .map(|entry| entry.parse::<u32>())
                    .process_results(|it| it.collect_vec())?;
                entries.append(&mut row);
                // part2
                line.chars()
                    .enumerate()
                    .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
                    .for_each(|(i, digit)| {
                        rtl_entries[i] *= 10;
                        rtl_entries[i] += digit
                    });
            } else {
                operations = iter
                    .flat_map(|entry| entry.chars())
                    .map(|c| c.try_into())
                    .process_results(|it| it.collect_vec())?;
            }
        }

        let rows =
            Array2::from_shape_vec((entries.len() / columns, columns), entries).map_err(|_| {
                DayError::GenericParseErr(
                    "lines in input have different lengths, while a rectangle input was expected",
                )
            })?;

        Ok(AocDay06 {
            rows,
            columns: rtl_entries,
            operations,
        })
    }
    fn part1(&self) -> u64 {
        self.rows
            .columns()
            .into_iter()
            .zip(self.operations.iter())
            .map(|(row, operator)| {
                row.iter()
                    .map(|&v| v as u64)
                    .reduce(|acc, v| match operator {
                        Operator::Sum => acc + v,
                        Operator::Product => acc * v,
                    })
                    .unwrap_or_default()
            })
            .sum()
    }
    fn part2(&self) -> u64 {
        let mut operations = self.operations.iter();
        let mut total = 0;
        let mut curr = 0;
        let mut curr_op = operations.next().expect("too few operations");
        for n in &self.columns {
            let n = *n as u64;
            if n == 0 {
                total += curr;
                curr = 0;
                curr_op = operations.next().expect("too few operations");
            }
            if curr == 0 {
                curr = n;
            } else {
                match curr_op {
                    Operator::Sum => curr += n,
                    Operator::Product => curr *= n,
                }
            }
        }
        total + curr
    }
}

#[cfg(test)]
mod day06tests {
    use super::*;

    const INPUT: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay06::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 4277556);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay06::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 3263827);
        Ok(())
    }
}
