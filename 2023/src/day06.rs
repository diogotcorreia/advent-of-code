use crate::AocDay;

fn ways_to_win(time: u64, distance: u64) -> u64 {
    let held_t = time / 2;

    // binary search!
    let mut left = 0u64;
    let mut right = held_t;
    while left < right {
        let middle = (left + right) / 2;

        if (middle * (time - middle)) > distance {
            // win! the answer is to the left
            right = middle;
        } else {
            // lose :( the answer is to the right
            left = middle + 1;
        }
    }

    let count_half = held_t - left + 1;
    if time % 2 == 0 {
        count_half * 2 - 1
    } else {
        count_half * 2
    }
}

#[derive(Debug)]
pub struct AocDay06 {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl AocDay<u64, u64> for AocDay06 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let time = lines
            .next()
            .expect("times not in input")
            .split_whitespace()
            .skip(1)
            .map(|i| i.parse().expect("time is not a number"))
            .collect();
        let distance = lines
            .next()
            .expect("distances not in input")
            .split_whitespace()
            .skip(1)
            .map(|i| i.parse().expect("distance is not a number"))
            .collect();

        AocDay06 { time, distance }
    }
    fn part1(&self) -> u64 {
        self.time
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let record = *self.distance.get(i).unwrap();
                ways_to_win(t, record)
            })
            .product()
    }
    fn part2(&self) -> u64 {
        let time: u64 = self
            .time
            .iter()
            .cloned()
            .reduce(|acc, i| acc * 10u64.pow(i.ilog10() + 1) + i)
            .expect("no times were given");
        let distance: u64 = self
            .distance
            .iter()
            .cloned()
            .reduce(|acc, i| acc * 10u64.pow(i.ilog10() + 1) + i)
            .expect("no distances were given");

        ways_to_win(time, distance)
    }
}

#[cfg(test)]
mod day06tests {
    use super::*;

    const INPUT: &[&str] = &["Time:      7  15   30", "Distance:  9  40  200"];

    #[test]
    fn part1() {
        let day = AocDay06::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 288);
    }

    #[test]
    fn part2() {
        let day = AocDay06::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 71503);
    }
}
