use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use ndarray::Array2;

use crate::AocDay;

#[derive(PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = ParseErr;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Forest),
            '.' => Ok(Self::Path),
            '^' => Ok(Self::Slope(Direction::North)),
            '>' => Ok(Self::Slope(Direction::East)),
            'v' => Ok(Self::Slope(Direction::South)),
            '<' => Ok(Self::Slope(Direction::West)),
            _ => Err(ParseErr),
        }
    }
}

#[derive(Debug)]
struct ParseErr;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn move_pos(&self, direction: &Direction, limits: &[usize]) -> Option<Pos> {
        let (x, y) = (self.x, self.y);
        match direction {
            Direction::North => y.checked_sub(1).map(|y| (x, y)),
            Direction::East => x.checked_add(1).map(|x| (x, y)),
            Direction::South => y.checked_add(1).map(|y| (x, y)),
            Direction::West => x.checked_sub(1).map(|x| (x, y)),
        }
        .filter(|(x, y)| *x < limits[1] && *y < limits[0])
        .map(|(x, y)| Pos { x, y })
    }
}

type EdgeMap = HashMap<Pos, Vec<(Pos, usize)>>;
type EdgeVec = Vec<Vec<(usize, usize)>>;

fn simplify_labirinth<const PART2: bool>(map: &Array2<Tile>, start: Pos, end: Pos) -> EdgeMap {
    let mut visited: Array2<bool> = Array2::default(map.raw_dim());
    let mut edges: EdgeMap = HashMap::new();
    let mut intersections = HashSet::new();
    let mut intersections_to_explore = VecDeque::new();
    intersections_to_explore.push_back((start, start));

    while let Some((intersection_start, pos)) = intersections_to_explore.pop_back() {
        let mut prev_pos = intersection_start;
        let mut curr_pos = pos;
        let mut len = 0;
        loop {
            if visited[(curr_pos.y, curr_pos.x)] {
                if len > 0 && intersections.contains(&curr_pos) {
                    edges
                        .entry(intersection_start)
                        .or_default()
                        .push((curr_pos, len));
                    if PART2 {
                        edges
                            .entry(curr_pos)
                            .or_default()
                            .push((intersection_start, len));
                    }
                }
                break;
            }
            visited[(curr_pos.y, curr_pos.x)] = true;

            if curr_pos == end {
                edges
                    .entry(intersection_start)
                    .or_default()
                    .push((curr_pos, len));
                if PART2 {
                    edges
                        .entry(curr_pos)
                        .or_default()
                        .push((intersection_start, len));
                }
                break;
            }

            let next_pos: Vec<_> = Direction::get_all()
                .into_iter()
                .filter_map(|direction| {
                    curr_pos
                        .move_pos(&direction, visited.shape())
                        .map(|pos| (pos, direction))
                })
                .filter(|(pos, _)| map[(pos.y, pos.x)] != Tile::Forest)
                .map(|(pos, from_dir)| match &map[(pos.y, pos.x)] {
                    Tile::Slope(dir) => (pos, *dir == from_dir),
                    _ => (pos, true),
                })
                .filter(|(pos, _)| *pos != prev_pos)
                .filter(|(pos, _)| !visited[(pos.y, pos.x)] || intersections.contains(pos))
                .collect();

            len += 1;
            if next_pos.len() == 1 {
                prev_pos = curr_pos;
                curr_pos = next_pos.first().unwrap().0;
            } else {
                for pos in next_pos
                    .into_iter()
                    .filter(|(_, can_follow)| *can_follow)
                    .map(|(pos, _)| pos)
                {
                    intersections_to_explore.push_back((curr_pos, pos));
                    intersections.insert(curr_pos);
                }
                visited[(curr_pos.y, curr_pos.x)] = true;
                edges
                    .entry(intersection_start)
                    .or_default()
                    .push((curr_pos, len - 1));
                if PART2 {
                    edges
                        .entry(curr_pos)
                        .or_default()
                        .push((intersection_start, len - 1));
                }
                break;
            }
        }
    }

    edges
}

fn convert_edge_map(edges: &EdgeMap) -> (EdgeVec, HashMap<Pos, usize>) {
    let mut mapping: HashMap<Pos, usize> =
        edges.keys().enumerate().map(|(i, pos)| (*pos, i)).collect();

    for pos in edges
        .values()
        .flat_map(|vec| vec.iter().map(|(pos, _)| pos))
    {
        let next_i = mapping.len();
        mapping.entry(*pos).or_insert(next_i);
    }

    let mut simple_edges = vec![vec![]; mapping.len()];
    for (pos, neighbours) in edges {
        simple_edges[mapping[pos]] = neighbours
            .iter()
            .map(|(pos, cost)| (mapping[pos], *cost))
            .collect();
    }

    (simple_edges, mapping)
}

fn find_longest_path<const PART2: bool>(
    start: usize,
    end: usize,
    graph: &EdgeVec,
    visited: &mut Vec<bool>,
) -> Option<usize> {
    if start == end {
        return Some(0);
    }
    let mut max: Option<usize> = None;
    visited[start] = true;
    for (neighbour, cost) in &graph[start] {
        if visited[*neighbour] {
            continue;
        }
        if let Some(new_cost) = find_longest_path::<PART2>(*neighbour, end, graph, visited) {
            let cost = new_cost + cost + 1;
            max = Some(max.map_or(cost, |prev| prev.max(cost)));
        }
    }
    visited[start] = false;

    max
}

pub struct AocDay23 {
    map: Array2<Tile>,
    start: Pos,
    end: Pos,
}

impl AocDay23 {
    fn solve<const PART2: bool>(&self) -> usize {
        let graph = simplify_labirinth::<PART2>(&self.map, self.start, self.end);
        let (graph, mapping) = convert_edge_map(&graph);
        find_longest_path::<PART2>(
            mapping[&self.start],
            mapping[&self.end],
            &graph,
            &mut vec![false; graph.len()],
        )
        .and_then(|v| v.checked_sub(1))
        .expect("could not find any path")
    }
}

impl AocDay<usize, usize> for AocDay23 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.peekable();
        let width = lines.peek().expect("map does not have any rows").len();
        let map: Vec<Tile> = lines
            .flat_map(|row| {
                row.chars()
                    .map(|c| c.try_into().expect("unknown tile type"))
                    .collect::<Vec<_>>()
            })
            .collect();

        let map = Array2::from_shape_vec((map.len() / width, width), map)
            .expect("failed to build array2");

        let start = map
            .indexed_iter()
            .find(|(_, tile)| **tile != Tile::Forest)
            .map(|((y, x), _)| Pos { x, y })
            .expect("cannot find start position");
        let end = Pos {
            x: map
                .row(map.nrows() - 1)
                .iter()
                .find_position(|x| **x != Tile::Forest)
                .expect("cannot find end position")
                .0,
            y: map.nrows() - 1,
        };

        AocDay23 { map, start, end }
    }
    fn part1(&self) -> usize {
        self.solve::<false>()
    }
    fn part2(&self) -> usize {
        self.solve::<true>()
    }
}

#[cfg(test)]
mod day23tests {
    use super::*;

    const INPUT: &[&str] = &[
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ];

    #[test]
    fn part1() {
        let day = AocDay23::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 94);
    }

    #[test]
    fn part2() {
        let day = AocDay23::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 154);
    }
}
