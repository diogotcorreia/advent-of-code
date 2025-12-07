use aoc_common::{parsing::try_parse_2d_array, AocDay, DayError};
use aoc_common_macros::TryFromChar;
use ndarray::Array2;

#[derive(Debug, PartialEq, Eq, TryFromChar)]
enum Tile {
    #[char_repr = '.']
    Air,
    #[char_repr = 'S']
    Start,
    #[char_repr = '^']
    Splitter,
}

pub struct AocDay07 {
    split_count: usize,
    timeline_count: usize,
}

impl AocDay<usize, usize> for AocDay07 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let map: Array2<Tile> = try_parse_2d_array(lines)?;
        let beams = {
            let x = map
                .row(0)
                .indexed_iter()
                .find(|(_, tile)| **tile == Tile::Start)
                .ok_or(DayError::GenericParseErr(
                    "input does not contain a start position",
                ))?
                .0;
            let mut beams = vec![0usize; map.ncols()];
            beams[x] = 1;
            beams
        };

        let (split_count, beams) =
            map.rows()
                .into_iter()
                .fold((0usize, beams), |(mut split_count, last_beams), row| {
                    let mut new_beams = vec![0; map.ncols()];
                    for (x, count) in last_beams.iter().enumerate().filter(|(_, &x)| x > 0) {
                        if row[x] == Tile::Splitter {
                            new_beams[x - 1] += count;
                            new_beams[x + 1] += count;
                            split_count += 1
                        } else {
                            new_beams[x] += count;
                        }
                    }

                    (split_count, new_beams)
                });

        Ok(AocDay07 {
            split_count,
            timeline_count: beams.iter().sum(),
        })
    }
    fn part1(&self) -> usize {
        self.split_count
    }
    fn part2(&self) -> usize {
        self.timeline_count
    }
}

#[cfg(test)]
mod day07tests {
    use super::*;

    const INPUT: &[&str] = &[
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay07::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 21);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay07::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 40);
        Ok(())
    }
}
