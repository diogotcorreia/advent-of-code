use std::collections::{HashMap, HashSet};

use crate::AocDay;

#[derive(Debug, Clone)]
enum Push {
    Left,
    Right,
}

impl Push {
    fn apply(&self, pos: &Pos) -> Pos {
        match self {
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
        }
    }
}

impl From<char> for Push {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => unreachable!("invalid push type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
enum RockType {
    Horizontal,
    Cross,
    L,
    Vertical,
    Square,
}

impl RockType {
    fn get_next(&self) -> Self {
        match self {
            Self::Horizontal => Self::Cross,
            Self::Cross => Self::L,
            Self::L => Self::Vertical,
            Self::Vertical => Self::Square,
            Self::Square => Self::Horizontal,
        }
    }
    fn get_height(&self) -> i64 {
        match self {
            Self::Horizontal => 1,
            Self::Cross => 3,
            Self::L => 3,
            Self::Vertical => 4,
            Self::Square => 2,
        }
    }
    fn get_width(&self) -> i64 {
        match self {
            Self::Horizontal => 4,
            Self::Cross => 3,
            Self::L => 3,
            Self::Vertical => 1,
            Self::Square => 2,
        }
    }
    fn get_positions(&self, pos: &Pos) -> Vec<Pos> {
        let rel_pos = match self {
            Self::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Cross => vec![(1, 0), (1, -1), (0, -1), (2, -1), (1, -2)],
            Self::L => vec![(2, 0), (2, -1), (0, -2), (1, -2), (2, -2)],
            Self::Vertical => vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            Self::Square => vec![(0, 0), (1, 0), (0, -1), (1, -1)],
        };

        rel_pos
            .into_iter()
            .map(|p| (pos.0 + p.0, pos.1 + p.1))
            .collect()
    }
}

type Pos = (i64, i64); // x, y

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rock {
    pos: Pos, // pos of highest, left-most position
    rock_type: RockType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CycleState {
    heights: [i64; 7],
    gas_rules_i: usize,
    rock_type: RockType,
}

fn simulate_game(gas_rules: &[Push], mut n: i64) -> i64 {
    let mut current_rock_type = RockType::Horizontal;
    let mut heights = [0i64; 7];
    let mut taken_pos = HashSet::new();
    let mut cycle_mem = HashMap::new();

    let mut gas_rules = gas_rules.iter().cloned().enumerate().cycle().peekable();

    let mut height_modifier: i64 = 0;

    while n > 0 {
        simulate_rock(
            &mut gas_rules,
            current_rock_type.clone(),
            &mut heights,
            &mut taken_pos,
        );

        current_rock_type = current_rock_type.get_next();

        let max_height = *heights.iter().max().unwrap();
        let state = CycleState {
            heights: heights.map(|h| max_height - h),
            gas_rules_i: gas_rules.peek().unwrap().0,
            rock_type: current_rock_type.clone(),
        };

        n -= 1;

        // Cycle detector
        if let Some(old_state) = cycle_mem.insert(state, (max_height, n)) {
            let height_period = max_height - old_state.0;
            let rock_count_period = old_state.1 - n;

            let rounds_to_skip = n / rock_count_period;
            let rounds_left_to_sim = n % rock_count_period;

            height_modifier = height_period * rounds_to_skip;
            n = rounds_left_to_sim;

            cycle_mem.clear();
        }
    }

    *heights.iter().max().unwrap() + height_modifier
}

fn is_valid_position(pos: &Pos, rock_type: &RockType, taken_pos: &HashSet<Pos>) -> bool {
    pos.0 >= 0
        && pos.0 + rock_type.get_width() <= 7
        && pos.1 - rock_type.get_height() >= 0
        && rock_type
            .get_positions(pos)
            .iter()
            .all(|p| !taken_pos.contains(p))
}

fn simulate_rock(
    gas_rules: &mut impl Iterator<Item = (usize, Push)>,
    rock_type: RockType,
    heights: &mut [i64; 7],
    taken_pos: &mut HashSet<Pos>,
) {
    let start_y = heights.iter().max().unwrap_or(&0) + 3;
    let mut rock = Rock {
        pos: (2, start_y + rock_type.get_height()),
        rock_type,
    };

    loop {
        // Simulate gas stream
        let next_pos: Pos = gas_rules.next().unwrap().1.apply(&rock.pos);
        if is_valid_position(&next_pos, &rock.rock_type, taken_pos) {
            rock.pos = next_pos;
        }

        // Simulate downwards movement
        let next_pos: Pos = (rock.pos.0, rock.pos.1 - 1);
        if is_valid_position(&next_pos, &rock.rock_type, taken_pos) {
            rock.pos = next_pos;
        } else {
            // solidify
            rock.rock_type
                .get_positions(&rock.pos)
                .into_iter()
                .for_each(|p| {
                    let col_height = heights.get_mut(p.0 as usize).unwrap();
                    *col_height = p.1.max(*col_height);
                    taken_pos.insert(p);
                });
            break;
        }
    }
}

pub struct AocDay17 {
    gas_rules: Vec<Push>,
}

impl AocDay<i64, i64> for AocDay17 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let gas_rules: Vec<Push> = lines
            .next()
            .expect("Input must have one line")
            .chars()
            .map(|x| x.into())
            .collect();

        AocDay17 { gas_rules }
    }
    fn part1(&self) -> i64 {
        simulate_game(&self.gas_rules, 2022)
    }
    fn part2(&self) -> i64 {
        simulate_game(&self.gas_rules, 1_000_000_000_000)
    }
}

#[cfg(test)]
mod day17tests {
    use super::*;

    const INPUT: &[&str] = &[">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"];

    #[test]
    fn part1() {
        let day = AocDay17::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 3068);
    }

    #[test]
    fn part2() {
        let day = AocDay17::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 1_514_285_714_288);
    }
}
