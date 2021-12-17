use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct TargetArea {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

fn main() {
    let target_area = {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        let re = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();

        let cap = re.captures(input.trim()).expect("input has invalid format");

        TargetArea {
            x: cap[1].parse().unwrap()..=cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap()..=cap[4].parse().unwrap(),
        }
    };

    let start_vx = get_vx_values_from_above(&target_area);
    let min_vx = *start_vx.iter().min().unwrap();
    let max_vy = get_max_vy(&target_area);

    let mut max_y = 0;
    let mut count_possible = 0;
    for vx in min_vx..=*target_area.x.end() {
        for vy in -max_vy..=max_vy {
            match simulate(&target_area, (vx, vy)) {
                Some(y) => {
                    if y > max_y {
                        max_y = y;
                    }
                    count_possible += 1;
                }
                _ => {}
            }
        }
    }

    println!("Part 1: {}", max_y);
    println!("Part 2: {}", count_possible);
}

fn simulate(target: &TargetArea, initial_velocity: (i32, i32)) -> Option<i32> {
    let mut velocity = (initial_velocity.0, initial_velocity.1);
    let mut pos = (0, 0);
    let mut max_y = 0;

    while pos.1 >= *target.y.start() {
        pos.0 += velocity.0;
        pos.1 += velocity.1;

        if pos.1 > max_y {
            max_y = pos.1;
        }

        if target.x.contains(&pos.0) && target.y.contains(&pos.1) {
            return Some(max_y);
        }

        velocity.0 = advance_x_velocity(velocity.0);
        velocity.1 -= 1;
    }

    None
}

fn advance_x_velocity(vel_x: i32) -> i32 {
    if vel_x > 0 {
        vel_x - 1
    } else if vel_x < 0 {
        vel_x + 1
    } else {
        vel_x
    }
}

// Max height will always enter target from above, just get the start velocities
// where vx reaches zero inside the area.
fn get_vx_values_from_above(target: &TargetArea) -> Vec<i32> {
    (0..=target.x.end() / 2)
        .filter(|x| target.x.contains(&(x * (x + 1) / 2)))
        .collect()
}

// The probe always takes the same y values above zero while ascending and descending,
// so we know it's impossible for it to step in the area if the velocity are y=0
// is greater than the distance to the target from y=0
fn get_max_vy(target: &TargetArea) -> i32 {
    target.y.start().abs() + 1
}
