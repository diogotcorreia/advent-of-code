use std::{convert::TryInto, io::BufRead};

#[derive(Debug, Clone, Copy)]
struct Digit {
    // 0, g, f, e, d, c, b, a
    mask: u8,
}

impl Digit {
    fn from(input: &str) -> Digit {
        Digit {
            mask: input
                .chars()
                .fold(0, |acc, c| acc | 2_u8.pow(c as u32 - 'a' as u32)),
        }
    }

    fn segment_count(&self) -> i32 {
        let mut count = 0;
        let mut mask: u8 = 1;

        for _ in 0..7 {
            if mask & self.mask > 0 {
                count += 1;
            }

            mask *= 2;
        }
        count
    }
}

#[derive(Debug)]
struct NotesEntry {
    signal_patterns: [Digit; 10],
    output_value: [Digit; 4],
}

impl NotesEntry {
    fn from(input: &str) -> NotesEntry {
        let (signal_patterns, output_value) = input.split_once('|').expect("split notes entry");

        NotesEntry {
            signal_patterns: signal_patterns
                .split_whitespace()
                .map(|s| s.trim())
                .filter(|s| s.len() != 0)
                .map(|s| Digit::from(s))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            output_value: output_value
                .split_whitespace()
                .map(|s| s.trim())
                .filter(|s| s.len() != 0)
                .map(|s| Digit::from(s))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

fn main() {
    let entries: Vec<NotesEntry> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| NotesEntry::from(&line.expect("read from stdin")))
        .collect();

    let easy_digits_count: usize = entries
        .iter()
        .map(|entry| {
            entry
                .output_value
                .iter()
                .filter(|value| {
                    value.segment_count() == 2 // ONE
                        || value.segment_count() == 4 // FOUR
                        || value.segment_count() == 3 // SEVEN
                        || value.segment_count() == 7 // EIGHT
                })
                .count()
        })
        .sum();

    let total_sum: i32 = entries.iter().map(|entry| guess_numbers(entry)).sum();

    println!("Part 1: {}", easy_digits_count);
    println!("Part 2: {}", total_sum);
}

fn guess_numbers(entry: &NotesEntry) -> i32 {
    let mut numbers: [Option<Digit>; 10] =
        [None, None, None, None, None, None, None, None, None, None];
    //let mut possibilities: [Option<Digit>; 7] = [None, None, None, None, None, None, None];

    let leftover_digits: Vec<&Digit> = entry
        .signal_patterns
        .iter()
        .filter(|digit| {
            match digit.segment_count() {
                2 => numbers[1] = Some(**digit), // ONE
                3 => numbers[7] = Some(**digit), // SEVEN
                4 => numbers[4] = Some(**digit), // FOUR
                7 => numbers[8] = Some(**digit), // EIGHT
                _ => return true,
            }
            false
        })
        .collect();

    let mut five_segment_numbers: Vec<&&Digit> = leftover_digits
        .iter()
        .filter(|digit| digit.segment_count() == 5)
        .collect();

    // find three: intercept all five-segment numbers with 1 and see which still represents 1
    let pos = five_segment_numbers
        .iter()
        .position(|digit| digit.mask & numbers[1].unwrap().mask == numbers[1].unwrap().mask)
        .unwrap();
    numbers[3] = Some(***five_segment_numbers.get(pos).unwrap());
    five_segment_numbers.remove(pos);

    // find five: intercept remaining five-segment numbers with 4 and see which has 3 bits
    let pos = five_segment_numbers
        .iter()
        .position(|digit| {
            Digit {
                mask: digit.mask & numbers[4].unwrap().mask,
            }
            .segment_count()
                == 3
        })
        .unwrap();
    numbers[5] = Some(***five_segment_numbers.get(pos).unwrap());
    five_segment_numbers.remove(pos);

    // find two: remaining five-segment number
    numbers[2] = Some(***five_segment_numbers.get(0).unwrap());

    let mut six_segment_numbers: Vec<&&Digit> = leftover_digits
        .iter()
        .filter(|digit| digit.segment_count() == 6)
        .collect();

    // find six: intercept six-segment numbers numbers with 1 and see which does not represent 1
    let pos = six_segment_numbers
        .iter()
        .position(|digit| digit.mask & numbers[1].unwrap().mask != numbers[1].unwrap().mask)
        .unwrap();
    numbers[6] = Some(***six_segment_numbers.get(pos).unwrap());
    six_segment_numbers.remove(pos);

    // find nine: intercept remaining six-segment numbers numbers with 4 and see which still represents 4
    let pos = six_segment_numbers
        .iter()
        .position(|digit| digit.mask & numbers[4].unwrap().mask == numbers[4].unwrap().mask)
        .unwrap();
    numbers[9] = Some(***six_segment_numbers.get(pos).unwrap());
    six_segment_numbers.remove(pos);

    // find zero: remaining six-segment number
    numbers[0] = Some(***six_segment_numbers.get(0).unwrap());

    entry.output_value.iter().fold(0, |acc, digit| {
        let n = numbers
            .iter()
            .position(|d| d.unwrap().mask == digit.mask)
            .unwrap();

        acc * 10 + n as i32
    })
}
