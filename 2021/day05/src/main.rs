use std::{collections::HashMap, io};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn from_str(pos: &str) -> Pos {
        let (x, y) = pos.split_once(",").expect("invalid input");
        let x: i32 = x.parse().expect("not a number");
        let y: i32 = y.parse().expect("not a number");
        Pos { x, y }
    }

    fn get_pos_inbetween(&self, pos: &Pos, ignore_diag: bool) -> Vec<Pos> {
        if self.x == pos.x {
            return get_range(self.y, pos.y)
                .into_iter()
                .map(|y| Pos { x: self.x, y })
                .collect();
        } else if self.y == pos.y {
            return get_range(self.x, pos.x)
                .into_iter()
                .map(|x| Pos { x, y: self.y })
                .collect();
        } else if !ignore_diag {
            let range_x = get_range(self.x, pos.x);
            let range_y = get_range(self.y, pos.y);
            return range_x
                .into_iter()
                .zip(range_y)
                .map(|(x, y)| Pos { x, y })
                .collect();
        }
        // ignore diagonals for now ig
        Vec::new()
    }
}

fn main() {
    let mut all_pos: HashMap<Pos, i32> = HashMap::new();
    let mut all_pos_diag: HashMap<Pos, i32> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        if input.trim().len() == 0 {
            break;
        }

        let (start_pos, end_pos) = input.trim().split_once(" -> ").expect("invalid input");
        let (start_pos, end_pos) = (Pos::from_str(start_pos), Pos::from_str(end_pos));

        start_pos
            .get_pos_inbetween(&end_pos, true)
            .iter()
            .for_each(|pos| {
                *all_pos.entry(*pos).or_insert(0) += 1;
            });

        start_pos
            .get_pos_inbetween(&end_pos, false)
            .iter()
            .for_each(|pos| {
                *all_pos_diag.entry(*pos).or_insert(0) += 1;
            });
    }

    let pos_over_one = all_pos.values().filter(|v| **v > 1).count();
    let pos_over_one_diag = all_pos_diag.values().filter(|v| **v > 1).count();

    println!("Part 1: {}", pos_over_one);
    println!("Part 2: {}", pos_over_one_diag);
}

fn get_range(n1: i32, n2: i32) -> Box<dyn Iterator<Item = i32>> {
    if n1 > n2 {
        Box::new((n2..=n1).rev())
    } else {
        Box::new(n1..=n2)
    }
}
