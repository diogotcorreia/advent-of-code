use std::str::FromStr;

use itertools::Itertools;

use crate::AocDay;

#[derive(Debug)]
struct ConvertEntry {
    dest_start: u32,
    src_start: u32,
    src_end: u32,
}

impl ConvertEntry {
    fn in_range(&self, from: u32) -> bool {
        self.src_start <= from && self.src_end >= from
    }
    fn get_range(&self, from: &Range) -> Option<Range> {
        // https://scicomp.stackexchange.com/questions/26258/the-easiest-way-to-find-intersection-of-two-intervals
        let start = from.start.max(self.src_start);
        let end = from.end.min(self.src_end);

        if start > end {
            None
        } else {
            Some(Range { start, end })
        }
    }
}

impl FromStr for ConvertEntry {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();

        let dest_start = it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?;
        let src_start = it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?;
        let length: u32 = it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?;

        Ok(ConvertEntry {
            dest_start,
            src_start,
            src_end: length - 1 + src_start,
        })
    }
}

#[derive(Debug)]
struct ConvertMap {
    entries: Vec<ConvertEntry>,
}

impl ConvertMap {
    fn convert(&self, from: u32) -> u32 {
        for entry in &self.entries {
            if entry.in_range(from) {
                let delta = entry.dest_start.abs_diff(entry.src_start);
                if entry.dest_start > entry.src_start {
                    return from + delta;
                }
                return from - delta;
            }
        }
        from
    }
    fn convert_range(&self, from: Range) -> Vec<Range> {
        let mut ranges = vec![from];
        let mut resulting_ranges = vec![];
        for entry in &self.entries {
            ranges = ranges
                .into_iter()
                .flat_map(|range| match entry.get_range(&range) {
                    Some(matched_range) => {
                        let delta = entry.dest_start.abs_diff(entry.src_start);
                        let (start, end) = if entry.dest_start > entry.src_start {
                            (matched_range.start + delta, matched_range.end + delta)
                        } else {
                            (matched_range.start - delta, matched_range.end - delta)
                        };

                        resulting_ranges.push(Range { start, end });

                        let mut new_ranges = vec![];
                        if range.start != matched_range.start {
                            new_ranges.push(Range {
                                start: range.start,
                                end: matched_range.start - 1,
                            });
                        }
                        if range.end != matched_range.end {
                            new_ranges.push(Range {
                                start: matched_range.start + 1,
                                end: range.end,
                            });
                        }
                        new_ranges.into_iter()
                    }
                    None => vec![range].into_iter(),
                })
                .collect();
        }
        ranges.into_iter().for_each(|r| resulting_ranges.push(r));
        resulting_ranges
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay05 {
    seeds: Vec<u32>,
    convert_maps: Vec<ConvertMap>,
}

impl AocDay<u32, u32> for AocDay05 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let seeds = lines.next().expect("failed to parse seeds");
        let (_, seeds) = seeds.split_once(": ").expect("failed to parse seeds");
        let seeds = seeds
            .split_whitespace()
            .map(|n| n.parse().expect("failed to parse seed number"))
            .collect();

        let mut convert_maps = vec![];
        let mut entries = vec![];

        for line in lines {
            if line.trim().is_empty() {
                if !entries.is_empty() {
                    convert_maps.push(ConvertMap { entries });
                    entries = vec![];
                }
                continue;
            }
            if line.ends_with(" map:") {
                continue;
            }
            entries.push(line.parse().expect("failed to parse convertion entry"));
        }

        if !entries.is_empty() {
            convert_maps.push(ConvertMap { entries });
        }

        AocDay05 {
            seeds,
            convert_maps,
        }
    }
    fn part1(&self) -> u32 {
        self.seeds
            .iter()
            .map(|&s| {
                self.convert_maps
                    .iter()
                    .fold(s, |acc, converter| converter.convert(acc))
            })
            .min()
            .expect("no seeds in input")
    }
    fn part2(&self) -> u32 {
        let ranges: Vec<Range> = self
            .seeds
            .iter()
            .tuples()
            .map(|(&start, &len)| Range {
                start,
                end: len - 1 + start,
            })
            .collect();

        self.convert_maps
            .iter()
            // rust does not like accumulating with an iterator :(
            .fold(ranges, |acc, converter| {
                acc.into_iter()
                    .flat_map(|r| converter.convert_range(r))
                    .collect()
            })
            .iter()
            .min()
            .expect("no seeds in input")
            .start
    }
}

#[cfg(test)]
mod day05tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day05_example.txt");

    #[test]
    fn part1() {
        let day = AocDay05::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(day.part1(), 35);
    }

    #[test]
    fn part2() {
        let day = AocDay05::preprocessing(INPUT.lines().map(String::from));
        assert_eq!(day.part2(), 46);
    }
}
