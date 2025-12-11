use std::{ops::Add, str::FromStr};

use aoc_common::{AocDay, DayError};
use itertools::Itertools;
use z3::{ast::Int, Optimize, SatResult};

#[derive(Debug, Default)]
struct Machine {
    lights: Vec<bool>,
    reverse_buttons: Vec<Vec<usize>>,
    button_count: usize,
    joltage: Vec<u32>,
}

impl FromStr for Machine {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut machine = Machine::default();
        let mut buttons: Vec<Vec<usize>> = vec![];

        for directive in s.split_whitespace() {
            if directive.starts_with('[') {
                machine.lights = directive
                    .chars()
                    .skip(1)
                    .take_while(|&c| c != ']')
                    .map(|c| match c {
                        '.' => Ok(false),
                        '#' => Ok(true),
                        _ => Err(DayError::GenericParseErr("invalid char in light")),
                    })
                    .process_results(|it| it.collect_vec())?;
            } else if directive.starts_with('(') {
                let button = directive[1..directive.len() - 1]
                    .split(',')
                    .map(|i| i.parse())
                    .process_results(|it| it.collect_vec())?;
                buttons.push(button);
            } else if directive.starts_with('{') {
                machine.joltage = directive[1..directive.len() - 1]
                    .split(',')
                    .map(|i| i.parse())
                    .process_results(|it| it.collect_vec())?;
            } else {
                return Err(DayError::GenericParseErr(
                    "unknown directive in machine input",
                ));
            }
        }

        machine.reverse_buttons = Self::light_button_mapping(machine.lights.len(), &buttons);
        machine.button_count = buttons.len();

        if machine.lights.len() != machine.joltage.len() {
            return Err(DayError::GenericParseErr(
                "number of lights and joltage does not match",
            ));
        }

        Ok(machine)
    }
}

impl Machine {
    /// Return the buttons that affect each respective light
    fn light_button_mapping(lights_length: usize, buttons: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut lights = vec![Vec::default(); lights_length];
        for (i, button) in buttons.iter().enumerate() {
            for &light in button {
                lights[light].push(i);
            }
        }
        lights
    }

    fn fewest_presses_generic<SC>(&self, solver_callback: SC) -> u64
    where
        SC: FnOnce(&Optimize, &[Int]),
    {
        let buttons = vec![(); self.button_count]
            .iter()
            .map(|_| Int::fresh_const("btn"))
            .collect_vec();
        let solver = Optimize::new();
        for btn in &buttons {
            solver.assert(&btn.ge(0));
        }

        solver_callback(&solver, &buttons);

        let sum = buttons
            .iter()
            .cloned()
            .reduce(|acc, btn| acc.add(btn))
            .unwrap();

        solver.minimize(&sum);

        assert_eq!(solver.check(&[]), SatResult::Sat);
        let model = solver.get_model().unwrap();
        let result = model.eval(&sum, true).unwrap();

        result.as_u64().unwrap()
    }

    fn fewest_presses_lights(&self) -> u64 {
        self.fewest_presses_generic(|solver, buttons| {
            for (&light, controlling_buttons) in self.lights.iter().zip(&self.reverse_buttons) {
                solver.assert(
                    &controlling_buttons
                        .iter()
                        .map(|&i| buttons[i].clone())
                        .reduce(|acc, btn| acc.add(btn))
                        .unwrap()
                        .rem(2)
                        .eq(light as u32),
                );
            }
        })
    }

    fn fewest_presses_joltage(&self) -> u64 {
        self.fewest_presses_generic(|solver, buttons| {
            for (&joltage, controlling_buttons) in self.joltage.iter().zip(&self.reverse_buttons) {
                solver.assert(
                    &controlling_buttons
                        .iter()
                        .map(|&i| buttons[i].clone())
                        .reduce(|acc, btn| acc.add(btn))
                        .unwrap()
                        .eq(joltage),
                );
            }
        })
    }
}

pub struct AocDay10 {
    machines: Vec<Machine>,
}

impl AocDay<u64, u64> for AocDay10 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Result<Self, DayError> {
        let machines = lines
            .map(|line| line.parse())
            .process_results(|it| it.collect_vec())?;

        Ok(AocDay10 { machines })
    }
    fn part1(&self) -> u64 {
        self.machines
            .iter()
            .map(|machine| machine.fewest_presses_lights())
            .sum()
    }
    fn part2(&self) -> u64 {
        self.machines
            .iter()
            .map(|machine| machine.fewest_presses_joltage())
            .sum()
    }
}

#[cfg(test)]
mod day10tests {
    use super::*;

    const INPUT: &[&str] = &[
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[test]
    fn part1() -> Result<(), DayError> {
        let day = AocDay10::preprocessing_tests(INPUT)?;
        assert_eq!(day.part1(), 7);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), DayError> {
        let day = AocDay10::preprocessing_tests(INPUT)?;
        assert_eq!(day.part2(), 33);
        Ok(())
    }
}
