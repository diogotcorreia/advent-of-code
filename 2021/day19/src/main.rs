use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn vec(from: &Pos, to: &Pos) -> Pos {
        Pos {
            x: to.x - from.x,
            y: to.y - from.y,
            z: to.z - from.z,
        }
    }

    fn add(&self, pos2: &Pos) -> Pos {
        Pos {
            x: self.x + pos2.x,
            y: self.y + pos2.y,
            z: self.z + pos2.z,
        }
    }

    fn manhattan_distance(&self, pos2: &Pos) -> i32 {
        (self.x - pos2.x).abs() + (self.y - pos2.y).abs() + (self.z - pos2.z).abs()
    }

    fn has_repeated_abs_coords(&self) -> bool {
        let mut vec = vec![self.x.abs(), self.y.abs(), self.z.abs()];
        vec.sort();
        vec.dedup();
        vec.len() < 3
    }

    fn apply_transformations(&mut self, transformations: &Vec<Transformation>) {
        for t in transformations {
            match t {
                Transformation::SwapXY => {
                    let y = self.y;
                    self.y = self.x;
                    self.x = y;
                }
                Transformation::SwapXZ => {
                    let z = self.z;
                    self.z = self.x;
                    self.x = z;
                }
                Transformation::SwapYZ => {
                    let z = self.z;
                    self.z = self.y;
                    self.y = z;
                }
                Transformation::SimetricX => {
                    self.x *= -1;
                }
                Transformation::SimetricY => {
                    self.y *= -1;
                }
                Transformation::SimetricZ => {
                    self.z *= -1;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Beacon {
    id: u16,
    pos: Pos,
    distance_to_nearby_beacons: Vec<i32>,
}

#[derive(Debug)]
struct Scanner {
    position: Option<Pos>,
    beacons_relative: Vec<Beacon>,
}

impl Scanner {
    fn get_beacon_absolute_positions(&self) -> Vec<Pos> {
        if self.position.is_none() {
            panic!("unknown scanner position");
        }

        self.beacons_relative
            .iter()
            .map(|b| b.pos.add(&self.position.unwrap()))
            .collect()
    }
}

#[derive(Debug)]
enum Transformation {
    SwapXY,
    SwapXZ,
    SwapYZ,
    SimetricX,
    SimetricY,
    SimetricZ,
}

fn main() {
    let mut scanners = {
        let mut current_id: u16 = 0;
        let mut scanners = Vec::new();
        loop {
            let mut input = String::new();
            // Discard first line
            std::io::stdin()
                .read_line(&mut input)
                .expect("failed to read from stdin");
            if input.trim().len() == 0 {
                break;
            }

            let mut positions = Vec::new();

            loop {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read from stdin");
                if input.trim().len() == 0 {
                    break;
                }

                let mut pos_split = input.trim().split(',');
                let x: i32 = pos_split.next().unwrap().parse().unwrap();
                let y: i32 = pos_split.next().unwrap().parse().unwrap();
                let z: i32 = pos_split.next().unwrap().parse().unwrap();

                positions.push(Beacon {
                    id: current_id,
                    pos: Pos { x, y, z },
                    distance_to_nearby_beacons: Vec::new(),
                });
                current_id += 1;
            }

            let mut distances = HashMap::new();

            for beacon in positions.iter() {
                let d = positions
                    .iter()
                    .map(|b| beacon.pos.manhattan_distance(&b.pos))
                    .sorted()
                    .skip(1) // skip distance to itself (0)
                    .collect::<Vec<i32>>();
                distances.insert(beacon.id, d);
            }

            for beacon in positions.iter_mut() {
                beacon.distance_to_nearby_beacons = distances.get(&beacon.id).unwrap().clone();
            }

            scanners.push(Scanner {
                position: None,
                beacons_relative: positions,
            });
        }
        scanners.get_mut(0).unwrap().position = Some(Pos { x: 0, y: 0, z: 0 });

        scanners
    };

    let scanner_count = scanners.len();
    while scanners_without_positions(&scanners) {
        for fixed_scanner in 0..scanner_count {
            if scanners.get(fixed_scanner).unwrap().position.is_none() {
                continue;
            }

            for unknown_scanner in 0..scanner_count {
                if fixed_scanner == unknown_scanner
                    || scanners.get(unknown_scanner).unwrap().position.is_some()
                {
                    continue;
                }

                if let Some(transformations) = find_overlapping_beacons(
                    scanners.get(fixed_scanner).unwrap(),
                    scanners.get(unknown_scanner).unwrap(),
                ) {
                    scanners
                        .get_mut(unknown_scanner)
                        .unwrap()
                        .beacons_relative
                        .iter_mut()
                        .for_each(|b| b.pos.apply_transformations(&transformations));

                    let scanner2_position = get_relative_scanner_position(
                        scanners.get(fixed_scanner).unwrap(),
                        scanners.get(unknown_scanner).unwrap(),
                    );

                    let scanner1_position = scanners
                        .get(fixed_scanner)
                        .unwrap()
                        .position
                        .unwrap()
                        .clone();

                    scanners.get_mut(unknown_scanner).unwrap().position =
                        Some(scanner1_position.add(&scanner2_position));
                }
            }
        }
    }

    let mut beacons: Vec<Pos> = scanners
        .iter()
        .flat_map(|s| s.get_beacon_absolute_positions())
        .collect();

    beacons.sort();
    beacons.dedup();

    let max_manhattan_distance = scanners
        .iter()
        .combinations(2)
        .map(|combs| {
            let (s1, s2) = (combs.get(0).unwrap(), combs.get(1).unwrap());
            s1.position
                .unwrap()
                .manhattan_distance(&s2.position.unwrap())
        })
        .max()
        .unwrap();

    println!("Part 1: {}", beacons.len());
    println!("Part 2: {}", max_manhattan_distance);
}

fn find_overlapping_beacons(scanner1: &Scanner, scanner2: &Scanner) -> Option<Vec<Transformation>> {
    let mut overlapping_beacons = Vec::new();
    let mut i = 0;
    for beacon1 in scanner1.beacons_relative.iter() {
        for beacon2 in scanner2.beacons_relative.iter() {
            let common_distances = get_common_values(
                &beacon1.distance_to_nearby_beacons,
                &beacon2.distance_to_nearby_beacons,
            );
            // itself is already a common beacon, so only 11 need to be common
            if common_distances.len() >= 11 {
                i += 1;
                overlapping_beacons.push((beacon1, beacon2));
            }
        }
    }
    if i < 12 {
        return None;
    }
    // found a match! now let's rotate scanner2's orientation to match scanner1's

    // start by finding a vector between two beacons that doesn't have repeated (absolute) coordinates
    let vecs = overlapping_beacons
        .iter()
        .tuple_combinations()
        .map(|(pair1, pair2)| {
            return (
                Pos::vec(&pair1.0.pos, &pair2.0.pos),
                Pos::vec(&pair1.1.pos, &pair2.1.pos),
            );
        })
        .find(|(vec1, vec2)| !vec1.has_repeated_abs_coords() && !vec2.has_repeated_abs_coords())
        .unwrap();

    let transformations = find_transformations(vecs.0, vecs.1);

    Some(transformations)
}

fn get_common_values(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    let iter1 = vec1.iter();
    let mut iter2 = vec2.iter();
    let mut common = Vec::new();

    if let Some(mut current2) = iter2.next() {
        for current1 in iter1 {
            while *current2 < *current1 {
                current2 = match iter2.next() {
                    Some(current2) => current2,
                    None => return common,
                };
            }
            if *current1 == *current2 {
                common.push(*current1);
            }
        }
    }
    common
}

fn find_transformations(vec1: Pos, mut vec2: Pos) -> Vec<Transformation> {
    let mut transformations = Vec::new();
    if vec1.x.abs() != vec2.x.abs() {
        if vec1.x.abs() == vec2.y.abs() {
            transformations.push(Transformation::SwapXY);
            let y = vec2.y;
            vec2.y = vec2.x;
            vec2.x = y;
        }
        if vec1.x.abs() == vec2.z.abs() {
            transformations.push(Transformation::SwapXZ);
            let z = vec2.z;
            vec2.z = vec2.x;
            vec2.x = z;
        }
    }
    if vec1.y.abs() != vec2.y.abs() {
        if vec1.y.abs() == vec2.x.abs() {
            transformations.push(Transformation::SwapXY);
            let y = vec2.y;
            vec2.y = vec2.x;
            vec2.x = y;
        }
        if vec1.y.abs() == vec2.z.abs() {
            transformations.push(Transformation::SwapYZ);
            let z = vec2.z;
            vec2.z = vec2.y;
            vec2.y = z;
        }
    }
    if vec1.x != 0 && vec1.x == -vec2.x {
        transformations.push(Transformation::SimetricX);
        vec2.x *= -1;
    }
    if vec1.y != 0 && vec1.y == -vec2.y {
        transformations.push(Transformation::SimetricY);
        vec2.y *= -1;
    }
    if vec1.z != 0 && vec1.z == -vec2.z {
        transformations.push(Transformation::SimetricZ);
        vec2.z *= -1;
    }

    if vec1 != vec2 {
        panic!("failed to transform!");
    }

    transformations
}

// Position of scanner2 relative to scanner1, assuming they're correctly oriented
fn get_relative_scanner_position(scanner1: &Scanner, scanner2: &Scanner) -> Pos {
    for beacon1 in scanner1.beacons_relative.iter() {
        for beacon2 in scanner2.beacons_relative.iter() {
            let common_distances = get_common_values(
                &beacon1.distance_to_nearby_beacons,
                &beacon2.distance_to_nearby_beacons,
            );
            // itself is already a common beacon, so only 11 need to be common
            if common_distances.len() >= 11 {
                return Pos::vec(&beacon2.pos, &beacon1.pos);
            }
        }
    }
    panic!("no beacons in common");
}

fn scanners_without_positions(scanners: &Vec<Scanner>) -> bool {
    scanners.iter().any(|s| s.position == None)
}
