use crate::AocDay;

#[derive(Debug, PartialEq, Eq)]
enum Material {
    Rock,
    Ash,
}

impl TryFrom<char> for Material {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Material::Ash),
            '#' => Ok(Material::Rock),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

fn is_reflection_x<const NUM_DIFF: u8>(map: &Vec<Vec<Material>>, y: usize) -> bool {
    let mut difference_count = 0;
    let mut i = 0;
    loop {
        let upper = y + i;
        let lower = y - i - 1;
        for j in 0..map[0].len() {
            if map[upper][j] != map[lower][j] {
                if difference_count >= NUM_DIFF {
                    return false;
                }
                difference_count += 1;
            }
        }

        if lower == 0 || upper + 1 == map.len() {
            break;
        }
        i += 1;
    }

    difference_count == NUM_DIFF
}

fn get_reflection_x<const NUM_DIFF: u8>(map: &Vec<Vec<Material>>) -> Option<usize> {
    (1..map.len()).find(|&y| is_reflection_x::<NUM_DIFF>(map, y))
}

fn is_reflection_y<const NUM_DIFF: u8>(map: &Vec<Vec<Material>>, x: usize) -> bool {
    let mut differences_count = 0;
    let mut i = 0;
    loop {
        let upper = x + i;
        let lower = x - i - 1;
        for row in map {
            if row[upper] != row[lower] {
                if differences_count >= NUM_DIFF {
                    return false;
                }
                differences_count += 1;
            }
        }

        if lower == 0 || upper + 1 == map[0].len() {
            break;
        }
        i += 1;
    }

    differences_count == NUM_DIFF
}

fn get_reflection_y<const NUM_DIFF: u8>(map: &Vec<Vec<Material>>) -> Option<usize> {
    (1..map[0].len()).find(|&x| is_reflection_y::<NUM_DIFF>(map, x))
}

pub struct AocDay13 {
    patterns: Vec<Vec<Vec<Material>>>,
}

impl AocDay13 {
    fn solve<const NUM_DIFF: u8>(&self) -> usize {
        self.patterns
            .iter()
            .map(|pattern| {
                if let Some(x) = get_reflection_x::<NUM_DIFF>(pattern) {
                    x * 100
                } else if let Some(y) = get_reflection_y::<NUM_DIFF>(pattern) {
                    y
                } else {
                    panic!("could not find reflection");
                }
            })
            .sum()
    }
}

impl AocDay<usize, usize> for AocDay13 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut patterns = vec![];
        let mut pattern = vec![];

        for line in lines {
            if line.trim().is_empty() {
                if !pattern.is_empty() {
                    patterns.push(pattern);
                    pattern = vec![];
                }
                continue;
            }
            pattern.push(
                line.chars()
                    .map(|c| c.try_into().expect("invalid character in map"))
                    .collect(),
            );
        }

        if !pattern.is_empty() {
            patterns.push(pattern);
        }

        AocDay13 { patterns }
    }
    fn part1(&self) -> usize {
        self.solve::<0>()
    }
    fn part2(&self) -> usize {
        self.solve::<1>()
    }
}

#[cfg(test)]
mod day13tests {
    use super::*;

    const INPUT: &[&str] = &[
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    #[test]
    fn part1() {
        let day = AocDay13::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 405);
    }

    #[test]
    fn part2() {
        let day = AocDay13::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 400);
    }
}
