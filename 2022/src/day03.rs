use crate::AocDay;

struct Rucksack {
    first_compartment: String,
    second_compartment: String,
}

impl From<String> for Rucksack {
    fn from(input: String) -> Self {
        assert!(input.len() % 2 == 0);

        let compartments = input.split_at(input.len() / 2);

        Rucksack {
            first_compartment: compartments.0.to_string(),
            second_compartment: compartments.1.to_string(),
        }
    }
}

impl ToString for Rucksack {
    fn to_string(&self) -> String {
        format!("{}{}", self.first_compartment, self.second_compartment)
    }
}

impl Rucksack {
    // Find the common char in both compartments
    fn find_common_item(&self) -> Option<char> {
        // Lines are small, so we can get away with O(n^2)
        for x in self.first_compartment.chars() {
            for y in self.second_compartment.chars() {
                if x == y {
                    return Some(x);
                }
            }
        }

        None
    }

    fn find_common_item_in_group(&self, elf1: &Rucksack, elf2: &Rucksack) -> Option<char> {
        // Lines are small, so we can get away with O(n^3)
        for x in self.to_string().chars() {
            for y in elf1.to_string().chars() {
                if x == y {
                    for z in elf2.to_string().chars() {
                        if x == z {
                            return Some(x);
                        }
                    }
                    break;
                }
            }
        }

        None
    }

    fn get_points_for_item(item: char) -> i32 {
        if item.is_ascii_uppercase() {
            item as i32 - 'A' as i32 + 26 + 1
        } else if item.is_ascii_lowercase() {
            item as i32 - 'a' as i32 + 1
        } else {
            0
        }
    }
}

pub struct AocDay03 {
    rucksacks: Vec<Rucksack>,
}

impl AocDay<i32, i32> for AocDay03 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        AocDay03 {
            rucksacks: lines
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .map(|x| x.into())
                .collect(),
        }
    }
    fn part1(&self) -> i32 {
        self.rucksacks
            .iter()
            .map(|x| x.find_common_item())
            .map(|x| x.expect("rucksack did not have a common item"))
            .map(Rucksack::get_points_for_item)
            .sum()
    }
    fn part2(&self) -> i32 {
        self.rucksacks
            .windows(3)
            .step_by(3)
            .map(|x| x[0].find_common_item_in_group(&x[1], &x[2]))
            .map(|x| x.expect("rucksack group did not have a common item"))
            .map(Rucksack::get_points_for_item)
            .sum()
    }
}

#[cfg(test)]
mod day03tests {
    use super::*;

    const INPUT: &[&str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn rucksack_common() {
        let rucksack = Rucksack::from(INPUT[0].to_string());
        assert_eq!(rucksack.find_common_item(), Some('p'));
    }

    #[test]
    fn rucksack_common_group() {
        let rucksack1 = Rucksack::from(INPUT[0].to_string());
        let rucksack2 = Rucksack::from(INPUT[1].to_string());
        let rucksack3 = Rucksack::from(INPUT[2].to_string());
        assert_eq!(
            rucksack1.find_common_item_in_group(&rucksack2, &rucksack3),
            Some('r')
        );
    }

    #[test]
    fn rucksack_item_points() {
        assert_eq!(Rucksack::get_points_for_item('a'), 1);
        assert_eq!(Rucksack::get_points_for_item('p'), 16);
        assert_eq!(Rucksack::get_points_for_item('v'), 22);
        assert_eq!(Rucksack::get_points_for_item('L'), 38);
    }

    #[test]
    fn part1() {
        let day = AocDay03::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 157);
    }

    #[test]
    fn part2() {
        let day = AocDay03::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 70);
    }
}
