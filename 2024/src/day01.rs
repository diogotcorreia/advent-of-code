use aoc_common::AocDay;

pub struct AocDay01 {
    // TODO
}

impl AocDay<u32, u32> for AocDay01 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        // TODO

        AocDay01 { /* TODO */ }
    }
    fn part1(&self) -> u32 {
        todo!()
    }
    fn part2(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod day01tests {
    use super::*;

    const INPUT: &[&str] = &[""];

    #[test]
    fn part1() {
        let day = AocDay01::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), todo!());
    }

    #[test]
    fn part2() {
        let day = AocDay01::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), todo!());
    }
}
