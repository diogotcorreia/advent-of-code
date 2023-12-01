use crate::AocDay;

const DIGIT_PATTERNS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

pub struct AocDay01 {
    lines: Vec<String>,
}

impl AocDay<u32, usize> for AocDay01 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        AocDay01 {
            lines: lines.collect(),
        }
    }
    fn part1(&self) -> u32 {
        self.lines
            .iter()
            .map(|line| {
                let (first, last) = line
                    .chars()
                    .fold(None, |acc, c| {
                        if c.is_ascii_digit() {
                            let digit = c.to_digit(10).expect("cannot convert char to digit");
                            Some((acc.map(|(first, _)| first).unwrap_or(digit), digit))
                        } else {
                            acc
                        }
                    })
                    .expect("cannot find digit in line");
                first * 10 + last
            })
            .sum()
    }
    fn part2(&self) -> usize {
        let reversed_digit_patterns: Vec<String> = DIGIT_PATTERNS
            .iter()
            .map(|s| s.chars().rev().collect())
            .collect();
        self.lines
            .iter()
            .map(|line| {
                let first = DIGIT_PATTERNS
                    .iter()
                    .enumerate()
                    .filter_map(|(number, pattern)| line.find(pattern).map(|i| (i, number % 10)))
                    .min_by_key(|(i, _)| *i)
                    .expect("cannot find digit in line")
                    .1;

                let reversed_line: String = line.chars().rev().collect();
                let last = reversed_digit_patterns
                    .iter()
                    .enumerate()
                    .filter_map(|(number, pattern)| {
                        reversed_line.find(pattern).map(|i| (i, number % 10))
                    })
                    .min_by_key(|(i, _)| *i)
                    .expect("cannot find digit in line")
                    .1;

                first * 10 + last
            })
            .sum()
    }
}

#[cfg(test)]
mod day01tests {
    use super::*;

    const INPUT: &[&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    const INPUT2: &[&str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    #[test]
    fn part1() {
        let day = AocDay01::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 142);
    }

    #[test]
    fn part2() {
        let day = AocDay01::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 281);
    }
}
