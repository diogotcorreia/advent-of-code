use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
};

use crate::AocDay;

type Edges = Vec<String>;
type AllEdges = HashMap<String, Edges>;
type ValveDistances = HashMap<String, u32>;
type AllValveDistances = HashMap<String, ValveDistances>;
type AllFlowRates = HashMap<String, u32>;

fn calculate_distances_to_nodes(
    edges: &AllEdges,
    valves_with_flow: &AllFlowRates,
) -> AllValveDistances {
    let mut nodes: Vec<String> = valves_with_flow.keys().cloned().collect();
    nodes.push("AA".to_string());

    nodes
        .iter()
        .map(|start| {
            (
                start.clone(),
                bfs(edges, start)
                    .into_iter()
                    .filter(|(valve, _)| valves_with_flow.contains_key(valve))
                    .map(|(valve, dist)| (valve, dist + 1))
                    .collect(),
            )
        })
        .collect()
}

fn bfs(edges: &AllEdges, start_node: &String) -> ValveDistances {
    let mut distances: ValveDistances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_front(start_node.to_string());
    distances.insert(start_node.to_string(), 0);

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();

        let distance = *distances.get(&node).expect("distance for node not found");

        for neighbour in edges.get(&node).expect("node does not exist") {
            if !distances.contains_key(neighbour) {
                distances.insert(neighbour.clone(), distance + 1);
                queue.push_front(neighbour.to_string());
            }
        }
    }

    distances
}

fn calc_preasure_release(
    valve_distances: &ValveDistances,
    flow_rates: &AllFlowRates,
    current_minute: i32,
) -> HashMap<String, u32> {
    valve_distances
        .iter()
        .map(|(valve, &distance)| {
            (
                valve.clone(),
                flow_rates
                    .get(valve)
                    .expect("could not find flow rate for valve")
                    * (current_minute - distance as i32).max(0) as u32,
            )
        })
        .collect()
}

fn dfs_greater_pressure(
    day: &AocDay16,
    current_valves: (&String, &String),
    current_minute: (i32, i32),
    already_visited: Vec<String>,
    current_pressure: u32,
    mut max_pressure: u32,
) -> u32 {
    let (distances_me, distances_elephant) = (
        day.valves_distances
            .get(current_valves.0)
            .expect("unknown distance from valve"),
        day.valves_distances
            .get(current_valves.1)
            .expect("unknown distance from valve"),
    );
    let (pressure_released_me, pressure_released_elephant) = (
        calc_preasure_release(distances_me, &day.flow_rates, current_minute.0)
            .into_iter()
            .filter(|(valve, i)| *i > 0 && !already_visited.contains(valve))
            .collect::<HashMap<_, _>>(),
        calc_preasure_release(distances_elephant, &day.flow_rates, current_minute.1)
            .into_iter()
            .filter(|(valve, i)| *i > 0 && !already_visited.contains(valve))
            .collect::<HashMap<_, _>>(),
    );

    let available_pressure: u32 = day
        .flow_rates
        .keys()
        .map(|key| {
            pressure_released_me
                .get(key)
                .unwrap_or(&0)
                .max(pressure_released_elephant.get(key).unwrap_or(&0))
        })
        .sum();

    if current_pressure + available_pressure < max_pressure {
        return max_pressure;
    }

    if current_minute.0 >= current_minute.1 {
        let mut options = pressure_released_me.into_iter().collect::<Vec<_>>();
        options.sort_by_key(|(_, p)| Reverse(*p));
        for (valve, pressure) in options {
            max_pressure = dfs_greater_pressure(
                day,
                (&valve, current_valves.1),
                (
                    current_minute.0 - *distances_me.get(&valve).unwrap() as i32,
                    current_minute.1,
                ),
                {
                    let mut vec_clone = already_visited.clone();
                    vec_clone.push(valve.clone());

                    vec_clone
                },
                current_pressure + pressure,
                max_pressure.max(current_pressure + pressure),
            );
        }
    }

    if current_minute.1 >= current_minute.0 && current_valves.0 != current_valves.1 {
        let mut options = pressure_released_elephant.into_iter().collect::<Vec<_>>();
        options.sort_by_key(|(_, p)| Reverse(*p));
        for (valve, pressure) in options {
            max_pressure = dfs_greater_pressure(
                day,
                (current_valves.0, &valve),
                (
                    current_minute.0,
                    current_minute.1 - *distances_elephant.get(&valve).unwrap() as i32,
                ),
                {
                    let mut vec_clone = already_visited.clone();
                    vec_clone.push(valve.clone());

                    vec_clone
                },
                current_pressure + pressure,
                max_pressure.max(current_pressure + pressure),
            );
        }
    }

    max_pressure
}

pub struct AocDay16 {
    flow_rates: AllFlowRates,
    valves_distances: AllValveDistances,
}

impl AocDay<u32, u32> for AocDay16 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut edges = HashMap::new();
        let mut flow_rates = HashMap::new();

        lines.for_each(|line| {
            let mut parts = line.split_ascii_whitespace();
            let valve_name = parts.nth(1).expect("expected valve name on input");

            let flow_rate = parts.nth(2).expect("expected flow rate on input");
            let flow_rate: u32 = flow_rate
                .get(5..(flow_rate.len() - 1))
                .expect("flow rate has unexpected size")
                .parse()
                .expect("flow rate is not an integer");

            let valve_edges: Vec<String> = parts
                .skip(4)
                .map(|x| {
                    x.get(0..2)
                        .map(String::from)
                        .expect("destination valve too small")
                })
                .collect();

            edges.insert(String::from(valve_name), valve_edges);
            if flow_rate > 0 {
                flow_rates.insert(String::from(valve_name), flow_rate);
            }
        });

        let valves_distances: AllValveDistances = calculate_distances_to_nodes(&edges, &flow_rates);

        AocDay16 {
            flow_rates,
            valves_distances,
        }
    }

    fn part1(&self) -> u32 {
        dfs_greater_pressure(
            self,
            (&"AA".to_string(), &"AA".to_string()),
            (30, 0),
            Vec::new(),
            0,
            0,
        )
    }

    fn part2(&self) -> u32 {
        dfs_greater_pressure(
            self,
            (&"AA".to_string(), &"AA".to_string()),
            (26, 26),
            Vec::new(),
            0,
            0,
        )
    }
}

#[cfg(test)]
mod day16tests {
    use super::*;

    const INPUT: &[&str] = &[
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ];

    #[test]
    fn part1() {
        let day = AocDay16::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 1651);
    }

    #[test]
    fn part2() {
        let day = AocDay16::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 1707);
    }
}
