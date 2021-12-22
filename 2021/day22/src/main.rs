use std::io::BufRead;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Area {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Area {
    fn overlaps(&self, area: &Area) -> bool {
        (Area::overlaps_coord(self.x, area.x) || Area::overlaps_coord(area.x, self.x))
            && (Area::overlaps_coord(self.y, area.y) || Area::overlaps_coord(area.y, self.y))
            && (Area::overlaps_coord(self.z, area.z) || Area::overlaps_coord(area.z, self.z))
    }

    fn overlaps_coord(x1: (i64, i64), x2: (i64, i64)) -> bool {
        (x1.0 <= x2.0 && x2.0 <= x1.1) || (x1.0 <= x2.1 && x2.1 <= x1.1)
    }

    fn exclude_area(&self, area: &Area) -> Vec<Area> {
        if !self.overlaps(area) {
            return vec![self.clone()];
        }
        let mut new_self = self.clone();
        let mut new_sections = Vec::new();
        // sections on Z axis
        if new_self.z.0 < area.z.0 {
            // chop off bottom
            new_sections.push(Area {
                x: new_self.x,
                y: new_self.y,
                z: (new_self.z.0, area.z.0 - 1),
            });
            new_self.z.0 = area.z.0;
        }
        if new_self.z.1 > area.z.1 {
            // chop off top
            new_sections.push(Area {
                x: new_self.x,
                y: new_self.y,
                z: (area.z.1 + 1, new_self.z.1),
            });
            new_self.z.1 = area.z.1;
        }

        // sections on Y axis
        if new_self.y.0 < area.y.0 {
            new_sections.push(Area {
                x: new_self.x,
                y: (new_self.y.0, area.y.0 - 1),
                z: new_self.z,
            });
            new_self.y.0 = area.y.0;
        }
        if new_self.y.1 > area.y.1 {
            new_sections.push(Area {
                x: new_self.x,
                y: (area.y.1 + 1, new_self.y.1),
                z: new_self.z,
            });
            new_self.y.1 = area.y.1;
        }

        // sections on X axis
        if new_self.x.0 < area.x.0 {
            new_sections.push(Area {
                x: (new_self.x.0, area.x.0 - 1),
                y: new_self.y,
                z: new_self.z,
            });
            new_self.x.0 = area.x.0;
        }
        if new_self.x.1 > area.x.1 {
            new_sections.push(Area {
                x: (area.x.1 + 1, new_self.x.1),
                y: new_self.y,
                z: new_self.z,
            });
            new_self.x.1 = area.x.1;
        }

        new_sections
    }

    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn is_inside_initialization_area(&self) -> bool {
        self.x.0 >= -50
            && self.x.1 <= 50
            && self.y.0 >= -50
            && self.y.1 <= 50
            && self.z.0 >= -50
            && self.z.1 <= 50
    }
}

fn main() {
    let re = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
        .unwrap();
    let instructions: Vec<(bool, Area)> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let cap = re.captures(line.trim()).unwrap();

            (
                &cap[1] == "on",
                Area {
                    x: (cap[2].parse().unwrap(), cap[3].parse().unwrap()),
                    y: (cap[4].parse().unwrap(), cap[5].parse().unwrap()),
                    z: (cap[6].parse().unwrap(), cap[7].parse().unwrap()),
                },
            )
        })
        .collect();

    println!(
        "Part 1: {}",
        run_simulations(
            &instructions
                .iter()
                .filter(|(_, a)| a.is_inside_initialization_area())
                .map(|(b, a)| (*b, a.clone()))
                .collect()
        )
    );
    println!("Part 2: {}", run_simulations(&instructions));
}

fn run_simulations(instructions: &Vec<(bool, Area)>) -> i64 {
    let mut on_areas = Vec::new();

    for (power, area) in instructions.iter() {
        on_areas = on_areas
            .iter()
            .flat_map(|a: &Area| a.exclude_area(area))
            .collect();

        if *power {
            on_areas.push(area.clone());
        }
    }

    on_areas.iter().map(|a| a.volume()).sum()
}
