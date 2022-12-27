use std::{collections::HashSet, str::FromStr};

use pathfinding::directed::bfs;

use crate::AocDay;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn get_adjacent(&self) -> [Pos; 6] {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .map(|p| Pos {
            x: self.x + p.0,
            y: self.y + p.1,
            z: self.z + p.2,
        })
    }
    fn is_inside_bounds(&self, max_bound: &Pos) -> bool {
        self.x <= max_bound.x && self.y <= max_bound.y && self.z <= max_bound.z
    }
}

impl FromStr for Pos {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');

        Ok(Pos {
            x: it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            y: it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
            z: it.next().ok_or(ParseErr)?.parse().map_err(|_| ParseErr)?,
        })
    }
}

#[derive(Debug)]
struct ParseErr;

fn is_outside(
    pos: &Pos,
    droplet_pos: &HashSet<Pos>,
    outside_pos: &mut HashSet<Pos>,
    inside_pos: &mut HashSet<Pos>,
    max_pos: &Pos,
) -> bool {
    let res = bfs::bfs(
        pos,
        |p| {
            p.get_adjacent()
                .into_iter()
                .filter(|x| !droplet_pos.contains(x))
                .filter(|x| x.is_inside_bounds(max_pos)) // not needed, but improves speed
                .collect::<Vec<_>>()
        },
        |p| outside_pos.contains(p) || inside_pos.contains(p),
    );

    match res {
        None => {
            inside_pos.insert(pos.clone());
            false
        }
        Some(path) => {
            if outside_pos.contains(path.last().unwrap()) {
                path.iter().for_each(|p| {
                    outside_pos.insert(p.clone());
                });
                true
            } else {
                path.iter().for_each(|p| {
                    inside_pos.insert(p.clone());
                });
                false
            }
        }
    }
}

pub struct AocDay18 {
    droplet: HashSet<Pos>,
}

impl AocDay<usize, usize> for AocDay18 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let droplet = lines
            .map(|x| x.parse().expect("failed to parse coordinate"))
            .collect();

        AocDay18 { droplet }
    }
    fn part1(&self) -> usize {
        self.droplet
            .iter()
            .map(|p| {
                p.get_adjacent()
                    .iter()
                    .filter(|adj| !self.droplet.contains(adj))
                    .count()
            })
            .sum()
    }
    fn part2(&self) -> usize {
        let mut outside_pos = HashSet::new();
        let mut inside_pos = HashSet::new();

        outside_pos.insert(Pos { x: 0, y: 0, z: 0 });
        let max_pos = self.droplet.iter().fold((0, 0, 0), |max_bound, p| {
            (
                max_bound.0.max(p.x),
                max_bound.1.max(p.y),
                max_bound.2.max(p.z),
            )
        });
        let max_pos = Pos {
            x: max_pos.0 + 1,
            y: max_pos.1 + 1,
            z: max_pos.2 + 1,
        };
        outside_pos.insert(max_pos.clone());

        self.droplet
            .iter()
            .map(|p| {
                p.get_adjacent()
                    .iter()
                    .filter(|adj| !self.droplet.contains(adj))
                    .filter(|adj| {
                        is_outside(
                            adj,
                            &self.droplet,
                            &mut outside_pos,
                            &mut inside_pos,
                            &max_pos,
                        )
                    })
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod day18tests {
    use super::*;

    const INPUT: &[&str] = &[
        "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6", "1,2,5",
        "3,2,5", "2,1,5", "2,3,5",
    ];

    #[test]
    fn part1() {
        let day = AocDay18::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 64);
    }

    #[test]
    fn part2() {
        let day = AocDay18::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 58);
    }
}
