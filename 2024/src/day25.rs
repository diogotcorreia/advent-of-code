use aoc_common::{AocDay, DayError};
use itertools::Itertools;

fn calculate_heights(lines: &[&str]) -> [u8; 5] {
    [0, 1, 2, 3, 4].map(|i| {
        lines
            .iter()
            .filter(|line| line.as_bytes()[i] == b'#')
            .count() as u8
    })
}

#[derive(PartialEq, Eq)]
enum SchematicType {
    Lock,
    Key,
}

struct Schematic {
    heights: [u8; 5],
    schematic_type: SchematicType,
}

pub struct AocDay25 {
    schematics: Vec<Schematic>,
}

impl AocDay<usize, String> for AocDay25 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let schematics = lines
            .tuple_windows()
            .step_by(8)
            .map(|(a, b, c, d, e, f)| {
                let schematic_type = if a.starts_with('#') {
                    SchematicType::Lock
                } else {
                    SchematicType::Key
                };
                let heights = calculate_heights(&[&b, &c, &d, &e, &f]);

                Schematic {
                    heights,
                    schematic_type,
                }
            })
            .collect_vec();

        Ok(AocDay25 { schematics })
    }
    fn part1(&self) -> usize {
        self.schematics
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.schematic_type != b.schematic_type)
            .filter(|(a, b)| {
                a.heights
                    .iter()
                    .zip(&b.heights)
                    .all(|(h_a, h_b)| h_a + h_b <= 5)
            })
            .count()
    }
    fn part2(&self) -> String {
        // there is no part 2
        "".to_string()
    }
}

#[cfg(test)]
mod day25tests {
    use super::*;

    const INPUT: &[&str] = &[
        "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
        ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
        "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
        "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay25::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 3);
        Ok(())
    }

    // there is no part 2
}
