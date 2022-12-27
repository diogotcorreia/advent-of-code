use std::str::FromStr;

use crate::AocDay;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost_ore: u32,
    obsidian_robot_cost_clay: u32,
    geode_robot_cost_ore: u32,
    geode_robot_cost_obsidian: u32,
}

impl FromStr for Blueprint {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();

        let id_str = it.nth(1).ok_or(ParseErr)?;

        Ok(Blueprint {
            id: id_str
                .get(0..(id_str.len() - 1))
                .ok_or(ParseErr)?
                .parse()
                .map_err(|_| ParseErr)?,
            ore_robot_cost: it.nth(4).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            clay_robot_cost: it.nth(5).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            obsidian_robot_cost_ore: it.nth(5).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            obsidian_robot_cost_clay: it.nth(2).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            geode_robot_cost_ore: it.nth(5).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            geode_robot_cost_obsidian: it.nth(2).ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
        })
    }
}

#[derive(Debug, Clone)]
enum ItemType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct SimulationState {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,

    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,

    ignored_types: [bool; 3], // ore, clay, obsidian
}

impl SimulationState {
    fn new() -> Self {
        SimulationState {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
            ignored_types: [false; 3],
        }
    }

    fn get_robot_to_build(&self, blueprint: &Blueprint) -> Vec<Option<ItemType>> {
        if blueprint.geode_robot_cost_ore <= self.ore_count
            && blueprint.geode_robot_cost_obsidian <= self.obsidian_count
        {
            // Force to make a Geode robot, it's always the best option
            return vec![Some(ItemType::Geode)];
        }
        let mut options = vec![None];
        if blueprint.obsidian_robot_cost_ore <= self.ore_count
            && blueprint.obsidian_robot_cost_clay <= self.clay_count
            && self.obsidian_robots < blueprint.geode_robot_cost_obsidian
            && !self.ignored_types[2]
        {
            options.push(Some(ItemType::Obsidian));
        }
        if blueprint.clay_robot_cost <= self.ore_count
            && self.clay_robots < blueprint.obsidian_robot_cost_clay
            && !self.ignored_types[1]
        {
            options.push(Some(ItemType::Clay));
        }
        if blueprint.ore_robot_cost <= self.ore_count
            && self.ore_robots
                < blueprint
                    .clay_robot_cost
                    .max(blueprint.obsidian_robot_cost_ore)
                    .max(blueprint.geode_robot_cost_ore)
            && !self.ignored_types[0]
        {
            options.push(Some(ItemType::Ore));
        }
        options
    }
}

fn simulate_blueprint<const T: u32>(blueprint: &Blueprint) -> u32 {
    let state = SimulationState::new();

    simulate_blueprint_iteration(blueprint, state, T, 0)
}

fn simulate_blueprint_iteration(
    blueprint: &Blueprint,
    state: SimulationState,
    time_remaining: u32,
    current_max: u32,
) -> u32 {
    if time_remaining == 0 {
        return state.geode_count;
    }

    // Prune if it isn't possible to reach the max number of geodes
    // sum of integers from m-1 to n = n(n+1)/2 - m(m+1)/2
    let max_geodes_that_can_be_produced = {
        let n = state.geode_robots + time_remaining - 1;
        let m = state.geode_robots.max(1) - 1;

        n * (n + 1) / 2 - m * (m + 1) / 2
    };
    if state.geode_count + max_geodes_that_can_be_produced <= current_max {
        return state.geode_count;
    }

    let options = state.get_robot_to_build(blueprint);

    options
        .iter()
        .scan(current_max, |current_max, robot_to_build| {
            let mut state = state.clone();

            if robot_to_build.is_none() {
                options.iter().flatten().cloned().for_each(|t| match t {
                    ItemType::Ore => state.ignored_types[0] = true,
                    ItemType::Clay => state.ignored_types[1] = true,
                    ItemType::Obsidian => state.ignored_types[2] = true,
                    _ => {}
                })
            } else {
                state.ignored_types = [false; 3];
            }

            match robot_to_build {
                Some(ItemType::Ore) => state.ore_count -= blueprint.ore_robot_cost,
                Some(ItemType::Clay) => state.ore_count -= blueprint.clay_robot_cost,
                Some(ItemType::Obsidian) => {
                    state.ore_count -= blueprint.obsidian_robot_cost_ore;
                    state.clay_count -= blueprint.obsidian_robot_cost_clay;
                }
                Some(ItemType::Geode) => {
                    state.ore_count -= blueprint.geode_robot_cost_ore;
                    state.obsidian_count -= blueprint.geode_robot_cost_obsidian;
                }
                _ => {}
            }

            state.ore_count += state.ore_robots;
            state.clay_count += state.clay_robots;
            state.obsidian_count += state.obsidian_robots;
            state.geode_count += state.geode_robots;

            if let Some(t) = robot_to_build {
                match t {
                    ItemType::Ore => state.ore_robots += 1,
                    ItemType::Clay => state.clay_robots += 1,
                    ItemType::Obsidian => state.obsidian_robots += 1,
                    ItemType::Geode => state.geode_robots += 1,
                }
            }

            let result =
                simulate_blueprint_iteration(blueprint, state, time_remaining - 1, *current_max);
            *current_max = result.max(*current_max);
            Some(result)
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay19 {
    blueprints: Vec<Blueprint>,
}

impl AocDay<u32, u32> for AocDay19 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let blueprints = lines
            .map(|x| x.parse().expect("invalid blueprint"))
            .collect();

        AocDay19 { blueprints }
    }
    fn part1(&self) -> u32 {
        self.blueprints
            .iter()
            .map(|blueprint| blueprint.id * simulate_blueprint::<24>(blueprint))
            .sum()
    }
    fn part2(&self) -> u32 {
        self.blueprints
            .iter()
            .take(3)
            .map(simulate_blueprint::<32>)
            .product()
    }
}

#[cfg(test)]
mod day19tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
    ];

    #[test]
    fn part1() {
        let day = AocDay19::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 33);
    }

    #[test]
    fn part2() {
        let day = AocDay19::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 56 * 62);
    }
}
