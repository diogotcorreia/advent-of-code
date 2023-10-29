use crate::AocDay;

fn parse_snafu(number: &str) -> i64 {
    number
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown character"),
        })
        .rev()
        .enumerate()
        .map(|(i, v)| 5i64.pow(i as u32) * v)
        .sum()
}

fn to_snafu(mut number: i64) -> String {
    let mut digits: Vec<i8> = Vec::new();

    while number > 0 {
        digits.push((number % 5) as i8);
        number /= 5;
    }

    let mut carry = 0;
    for digit in digits.iter_mut() {
        *digit += carry;
        carry = 0;
        if *digit >= 3 {
            *digit -= 5;
            carry += 1;
        }
    }
    if carry > 0 {
        digits.push(carry);
    }

    digits
        .iter()
        .rev()
        .map(|digit| match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("invalid digit"),
        })
        .collect()
}

pub struct AocDay25 {
    fuel: i64,
}

impl AocDay<String, String> for AocDay25 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let fuel = lines.map(|line| parse_snafu(&line)).sum();

        AocDay25 { fuel }
    }
    fn part1(&self) -> String {
        to_snafu(self.fuel)
    }
    fn part2(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod day25tests {
    use super::*;

    const INPUT: &[&str] = &[
        "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
        "1=", "122",
    ];

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn part1() {
        let day = AocDay25::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), "2=-1=0");
    }
}
