use std::collections::{HashMap, VecDeque};

use num::integer::lcm;

use crate::AocDay;

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    Start,
    FlipFlop,
    Conjunction,
}

struct Module {
    mtype: ModuleType,
    connections: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

impl From<bool> for Signal {
    fn from(value: bool) -> Self {
        match value {
            true => Signal::High,
            false => Signal::Low,
        }
    }
}

#[derive(Debug)]
struct Pulse {
    from: String,
    signal: Signal,
    to: String,
}

struct State {
    flip_flops: HashMap<String, bool>,
    conjunction: HashMap<String, HashMap<String, Signal>>,
}

pub struct AocDay20 {
    modules: HashMap<String, Module>,
    backlink_connections: HashMap<String, Vec<String>>,
}

impl AocDay20 {
    fn init_state(&self) -> State {
        let mut state = State {
            conjunction: HashMap::new(),
            flip_flops: HashMap::new(),
        };

        for (name, module) in &self.modules {
            match module.mtype {
                ModuleType::FlipFlop => {
                    state.flip_flops.insert(name.clone(), false);
                }
                ModuleType::Conjunction => {
                    state.conjunction.insert(
                        name.clone(),
                        self.backlink_connections
                            .get(name)
                            .unwrap()
                            .iter()
                            .map(|n| (n.clone(), Signal::Low))
                            .collect(),
                    );
                }
                _ => {}
            }
        }

        state
    }

    fn handle_signal(&self, pulse: &Pulse, state: &mut State) -> Vec<Pulse> {
        let module = self.modules.get(&pulse.to);
        if module.is_none() {
            return Vec::new();
        }
        let module = module.unwrap();
        let my_signal = match module.mtype {
            ModuleType::Start => pulse.signal.clone(),
            ModuleType::FlipFlop => {
                if pulse.signal == Signal::High {
                    return Vec::new();
                }
                let flip_flop_state = state.flip_flops.get_mut(&pulse.to).unwrap();
                *flip_flop_state = !*flip_flop_state;

                (*flip_flop_state).into()
            }
            ModuleType::Conjunction => {
                let from_signal = state
                    .conjunction
                    .get_mut(&pulse.to)
                    .unwrap()
                    .get_mut(&pulse.from)
                    .unwrap();
                *from_signal = pulse.signal.clone();

                if state
                    .conjunction
                    .get(&pulse.to)
                    .unwrap()
                    .values()
                    .all(|s| *s == Signal::High)
                {
                    Signal::Low
                } else {
                    Signal::High
                }
            }
        };

        module
            .connections
            .iter()
            .map(|dest| Pulse {
                from: pulse.to.clone(),
                signal: my_signal.clone(),
                to: dest.clone(),
            })
            .collect()
    }

    fn simulate_button_press(&self, state: &mut State, mut pulse_callback: impl FnMut(&Pulse)) {
        let mut pending_signals: VecDeque<Pulse> = VecDeque::from(vec![Pulse {
            from: "button".to_string(),
            signal: Signal::Low,
            to: "broadcaster".to_string(),
        }]);

        while !pending_signals.is_empty() {
            let first_signal = pending_signals.pop_front().unwrap();
            pulse_callback(&first_signal);
            let new_signals = self.handle_signal(&first_signal, state);

            new_signals
                .into_iter()
                .for_each(|s| pending_signals.push_back(s));
        }
    }
}

impl AocDay<u32, u64> for AocDay20 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut backlink_connections: HashMap<String, Vec<String>> = HashMap::new();
        let modules = lines
            .map(|line| {
                let (name, connections) = line.split_once(" -> ").unwrap();
                let mtype = if name == "broadcaster" {
                    ModuleType::Start
                } else if name.starts_with('%') {
                    ModuleType::FlipFlop
                } else if name.starts_with('&') {
                    ModuleType::Conjunction
                } else {
                    unreachable!("unknown module");
                };

                let name = name.replace(['%', '&'], "");
                let connections: Vec<String> = connections
                    .split(", ")
                    .map(|connection| connection.to_string())
                    .collect();

                for connection in &connections {
                    backlink_connections
                        .entry(connection.clone())
                        .or_default()
                        .push(name.clone());
                }

                (name, Module { mtype, connections })
            })
            .collect();

        AocDay20 {
            modules,
            backlink_connections,
        }
    }
    fn part1(&self) -> u32 {
        let mut low_count = 0;
        let mut high_count = 0;
        let mut state = self.init_state();
        for _ in 0..1000 {
            self.simulate_button_press(&mut state, |pulse| match pulse.signal {
                Signal::Low => low_count += 1,
                Signal::High => high_count += 1,
            })
        }

        low_count * high_count
    }
    fn part2(&self) -> u64 {
        let mut state = self.init_state();
        let module_containing_rx = self
            .modules
            .iter()
            .find(|(_, module)| module.connections.contains(&"rx".to_string()))
            .unwrap();
        assert_eq!(module_containing_rx.1.mtype, ModuleType::Conjunction);

        let mut high_pulses: HashMap<String, Vec<u32>> = HashMap::new();
        for i in 0..10000 {
            self.simulate_button_press(&mut state, |pulse| {
                if pulse.to == *module_containing_rx.0 && pulse.signal == Signal::High {
                    high_pulses
                        .entry(pulse.from.clone())
                        .or_default()
                        .push(i + 1);
                }
            });
        }

        // (offset, period)
        let periods: Vec<u64> = high_pulses
            .values()
            .map(|module| {
                assert!(module.len() >= 2);
                let i = module.get((module.len() - 2)..).unwrap();
                let period = i[1].abs_diff(i[0]);
                let offset = *module.first().unwrap() - period;
                // assert cycles all start at 0
                assert_eq!(offset, 0);

                period.into()
            })
            .collect();

        periods.into_iter().reduce(lcm).unwrap_or(0)
    }
}

#[cfg(test)]
mod day20tests {
    use super::*;

    const INPUT: &[&str] = &[
        "broadcaster -> a, b, c",
        "%a -> b",
        "%b -> c",
        "%c -> inv",
        "&inv -> a",
    ];
    const INPUT2: &[&str] = &[
        "broadcaster -> a",
        "%a -> inv, con",
        "&inv -> b",
        "%b -> con",
        "&con -> output",
    ];

    #[test]
    fn part1() {
        let day = AocDay20::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 32000000);
        let day = AocDay20::preprocessing(INPUT2.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 11687500);
    }
}
