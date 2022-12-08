use std::{collections::LinkedList, str::FromStr};

use crate::AocDay;

pub struct AocDay05 {
    cranes: Vec<Crane>,
    moves: Vec<Move>,
}

type Crane = LinkedList<u8>;

#[derive(Debug)]
struct Move {
    qnt: i32,
    from: usize,
    to: usize,
}
struct MoveParseError(());
impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace().skip(1).step_by(2);

        Ok(Move {
            qnt: split
                .next()
                .ok_or(MoveParseError(()))?
                .parse()
                .map_err(|_| MoveParseError(()))?,
            from: split
                .next()
                .ok_or(MoveParseError(()))?
                .parse::<usize>()
                .map_err(|_| MoveParseError(()))?
                - 1,
            to: split
                .next()
                .ok_or(MoveParseError(()))?
                .parse::<usize>()
                .map_err(|_| MoveParseError(()))?
                - 1,
        })
    }
}

fn move_item(cranes: &mut [Crane], mov: &Move) {
    for _ in 0..mov.qnt {
        let crane_from = cranes.get_mut(mov.from).expect("from crane must exist");
        let element = crane_from.pop_front().expect("crane is empty");
        let crane_to = cranes.get_mut(mov.to).expect("crane to must exist");
        crane_to.push_front(element);
    }
}

fn move_item_keep_order(cranes: &mut [Crane], mov: &Move) {
    let mut tmp_crane = Vec::new();

    let crane_from = cranes.get_mut(mov.from).expect("from crane must exist");
    for _ in 0..mov.qnt {
        let element = crane_from.pop_front().expect("crane is empty");
        tmp_crane.push(element);
    }

    let crane_to = cranes.get_mut(mov.to).expect("crane to must exist");
    tmp_crane
        .into_iter()
        .rev()
        .for_each(|x| crane_to.push_front(x));
}

fn top_level_crates_to_str(cranes: Vec<Crane>) -> String {
    let top_level = cranes.iter().filter_map(|x| x.front().map(|x| *x as char));

    String::from_iter(top_level)
}

impl AocDay<String, String> for AocDay05 {
    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let mut cranes: Vec<Crane> = Vec::new();

        loop {
            let line = lines.next().expect("crane not finished");
            let crane_len = (line.len() + 1) / 4;
            if !line.contains('[') {
                // finished cranes
                break;
            }
            if cranes.is_empty() {
                // first line
                for _ in 0..crane_len {
                    cranes.push(LinkedList::new());
                }
            }
            let line_bytes = line.as_bytes();
            for i in 0..crane_len {
                if let Some(c) = line_bytes.get(i * 4 + 1).filter(|x| **x != b' ') {
                    cranes.get_mut(i).expect("crane should exist").push_back(*c);
                }
            }
        }

        let moves: Vec<Move> = lines.filter_map(|x| x.parse().ok()).collect();

        AocDay05 { cranes, moves }
    }
    fn part1(&self) -> String {
        let mut cranes = self.cranes.clone();
        self.moves
            .iter()
            .for_each(|mov| move_item(&mut cranes, mov));

        top_level_crates_to_str(cranes)
    }
    fn part2(&self) -> String {
        let mut cranes = self.cranes.clone();
        self.moves
            .iter()
            .for_each(|mov| move_item_keep_order(&mut cranes, mov));

        top_level_crates_to_str(cranes)
    }
}

#[cfg(test)]
mod day05tests {
    use super::*;

    const INPUT: &[&str] = &[
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn part1() {
        let day = AocDay05::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), "CMZ");
    }

    #[test]
    fn part2() {
        let day = AocDay05::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), "MCD");
    }
}
