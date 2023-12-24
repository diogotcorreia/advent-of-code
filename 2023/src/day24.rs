use std::str::FromStr;

use itertools::Itertools;
use z3::{
    ast::{Ast, Int, Real},
    Config, Context, SatResult, Solver,
};

use crate::AocDay;

struct Hailstone {
    position: Pos3D,
    velocity: Pos3D,
}

impl FromStr for Hailstone {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").ok_or(ParseErr)?;
        Ok(Hailstone {
            position: pos.parse()?,
            velocity: vel.parse()?,
        })
    }
}

struct Pos3D {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Pos3D {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|c| c.trim().parse().map_err(|_| ParseErr));

        Ok(Pos3D {
            x: coords.next().ok_or(ParseErr)??,
            y: coords.next().ok_or(ParseErr)??,
            z: coords.next().ok_or(ParseErr)??,
        })
    }
}

#[derive(Debug)]
struct ParseErr;

pub struct AocDay24 {
    hailstones: Vec<Hailstone>,
}

impl AocDay24 {
    fn count_intersections(&self, range: (i64, i64)) -> usize {
        self.hailstones
            .iter()
            .tuple_combinations()
            .filter(|(h1, h2)| {
                // Math behind this:
                // x = a + bt
                // y = c + dt
                //
                // x = e + fs
                // y = g + hs
                //
                // t = (fs - a + e)/b
                // s = ((a - e)/b - c + g)/(df / b - h)

                let a = h1.position.x as f64;
                let b = h1.velocity.x as f64;
                let c = h1.position.y as f64;
                let d = h1.velocity.y as f64;
                let e = h2.position.x as f64;
                let f = h2.velocity.x as f64;
                let g = h2.position.y as f64;
                let h = h2.velocity.y as f64;

                let s = (d * (a - e) / b - c + g) / (d * f / b - h);
                let t = (f * s - a + e) / b;

                let x = a + b * t;
                let y = c + d * t;

                s > 0.
                    && t > 0.
                    && x >= range.0 as f64
                    && x <= range.1 as f64
                    && y >= range.0 as f64
                    && y <= range.1 as f64
            })
            .count()
    }
}

impl AocDay<usize, i64> for AocDay24 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let hailstones = lines
            .map(|line| line.parse().expect("invalid hailstone"))
            .collect();

        AocDay24 { hailstones }
    }
    fn part1(&self) -> usize {
        self.count_intersections((200000000000000, 400000000000000))
    }
    fn part2(&self) -> i64 {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let rock_px = Real::new_const(&ctx, "rpx");
        let rock_py = Real::new_const(&ctx, "rpy");
        let rock_pz = Real::new_const(&ctx, "rpz");
        let rock_vx = Real::new_const(&ctx, "rvx");
        let rock_vy = Real::new_const(&ctx, "rvy");
        let rock_vz = Real::new_const(&ctx, "rvz");

        let solver = Solver::new(&ctx);

        let zero = Real::from_int(&Int::from_i64(&ctx, 0));
        for (i, hailstone) in self.hailstones.iter().enumerate() {
            let hailstone_px = Real::from_int(&Int::from_i64(&ctx, hailstone.position.x));
            let hailstone_py = Real::from_int(&Int::from_i64(&ctx, hailstone.position.y));
            let hailstone_pz = Real::from_int(&Int::from_i64(&ctx, hailstone.position.z));
            let hailstone_vx = Real::from_int(&Int::from_i64(&ctx, hailstone.velocity.x));
            let hailstone_vy = Real::from_int(&Int::from_i64(&ctx, hailstone.velocity.y));
            let hailstone_vz = Real::from_int(&Int::from_i64(&ctx, hailstone.velocity.z));
            let impact_t = Real::new_const(&ctx, format!("t{}", i));
            solver.assert(&impact_t.ge(&zero));
            solver.assert(
                &(&hailstone_px + &hailstone_vx * &impact_t)
                    ._eq(&(&rock_px + &rock_vx * &impact_t)),
            );
            solver.assert(
                &(&hailstone_py + &hailstone_vy * &impact_t)
                    ._eq(&(&rock_py + &rock_vy * &impact_t)),
            );
            solver.assert(
                &(&hailstone_pz + &hailstone_vz * &impact_t)
                    ._eq(&(&rock_pz + &rock_vz * &impact_t)),
            );
        }

        assert_eq!(solver.check(), SatResult::Sat);
        let model = solver.get_model().unwrap();
        let result = model.eval(&(&rock_px + &rock_py + &rock_pz), true).unwrap();
        result.as_real().unwrap().0
    }
}

#[cfg(test)]
mod day24tests {
    use super::*;

    const INPUT: &[&str] = &[
        "19, 13, 30 @ -2,  1, -2",
        "18, 19, 22 @ -1, -1, -2",
        "20, 25, 34 @ -2, -2, -4",
        "12, 31, 28 @ -1, -2, -1",
        "20, 19, 15 @  1, -5, -3",
    ];

    #[test]
    fn part1() {
        let day = AocDay24::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.count_intersections((7, 27)), 2);
    }

    #[test]
    fn part2() {
        let day = AocDay24::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 47);
    }
}
