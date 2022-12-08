use crate::AocDay;

pub struct AocDay01 {
    calories_by_elves: Vec<i32>,
}

impl AocDay<i32, i32> for AocDay01 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut vec: Vec<i32> = Vec::new();

        let mut calories_sum: Option<i32> = None;

        for input in lines {
            if input.trim().is_empty() {
                if let Some(v) = calories_sum {
                    vec.push(v);
                    calories_sum = None;
                }
                continue;
            }

            calories_sum = Some(
                calories_sum.unwrap_or(0)
                    + input
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to convert to int"),
            );
        }

        if let Some(v) = calories_sum {
            vec.push(v);
        }

        vec.sort();

        AocDay01 {
            calories_by_elves: vec,
        }
    }
    fn part1(&self) -> i32 {
        *self.calories_by_elves.last().unwrap()
    }
    fn part2(&self) -> i32 {
        self.calories_by_elves.iter().rev().take(3).sum()
    }
}
