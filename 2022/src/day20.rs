use std::{cmp::Ordering, collections::LinkedList};

use crate::AocDay;

const DECRYPTION_KEY: i64 = 811589153;
const DECRYPTION_ROUNDS: usize = 10;

struct CircularList(LinkedList<(usize, i64)>, usize);

impl CircularList {
    fn new(numbers: &[i64]) -> CircularList {
        CircularList(numbers.iter().cloned().enumerate().collect(), numbers.len())
    }
    fn move_number(&mut self, index: usize) {
        let mut cursor = self.0.cursor_front_mut();
        while let Some(el) = cursor.as_cursor().current() {
            if el.0 == index {
                break;
            }
            cursor.move_next();
        }
        if let Some(el) = cursor.as_cursor().current() {
            let el = *el;
            let to_move = Self::calculate_delta(self.1, cursor.index().unwrap(), el.1);
            match to_move.cmp(&0) {
                Ordering::Greater => {
                    cursor.remove_current();

                    for _ in 0..to_move {
                        cursor.move_next();
                    }
                    cursor.insert_before(el);
                }
                Ordering::Less => {
                    cursor.remove_current();

                    for _ in to_move..0 {
                        cursor.move_prev();
                    }
                    cursor.insert_before(el);
                }
                _ => {}
            }
        }
    }
    fn calculate_delta(length: usize, start_index: usize, to_move: i64) -> i64 {
        let destination = match to_move.cmp(&0) {
            Ordering::Greater | Ordering::Less => {
                let to_move = to_move % (length as i64 - 1);
                (start_index as i64 + to_move).rem_euclid(length as i64 - 1)
            }
            Ordering::Equal => start_index as i64,
        };

        destination - start_index as i64
    }
    fn get_groove_coordinates(&self) -> [i64; 3] {
        let vec: Vec<_> = self.0.iter().map(|x| x.1).collect();

        let zero_index = vec
            .iter()
            .enumerate()
            .find(|x| *x.1 == 0)
            .expect("could not find zero")
            .0;

        [1000, 2000, 3000]
            .map(|x| (zero_index + x) % vec.len())
            .map(|x| *vec.get(x).expect("out of bounds index"))
    }
}

pub struct AocDay20 {
    list: Vec<i64>,
}

impl AocDay<i64, i64> for AocDay20 {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let list = lines
            .map(|l| l.parse().expect("invalid number on input"))
            .collect();

        AocDay20 { list }
    }
    fn part1(&self) -> i64 {
        let mut circ_list = CircularList::new(&self.list);

        for i in 0..self.list.len() {
            circ_list.move_number(i);
        }

        circ_list.get_groove_coordinates().iter().sum()
    }
    fn part2(&self) -> i64 {
        let mut circ_list = CircularList::new(
            &self
                .list
                .iter()
                .map(|x| x * DECRYPTION_KEY)
                .collect::<Vec<_>>(),
        );

        for _ in 0..DECRYPTION_ROUNDS {
            for i in 0..self.list.len() {
                circ_list.move_number(i);
            }
        }

        circ_list.get_groove_coordinates().iter().sum()
    }
}

#[cfg(test)]
mod day20tests {
    use super::*;

    const INPUT: &[&str] = &["1", "2", "-3", "3", "-2", "0", "4"];

    #[test]
    fn move_number() {
        let mut circ_list = CircularList::new(&[-1, 2, 10, 0]);
        circ_list.move_number(0);
        assert_eq!(
            circ_list.0.iter().map(|x| x.1).collect::<Vec<_>>(),
            vec![2, 10, -1, 0]
        );
        circ_list.move_number(1);
        assert_eq!(
            circ_list.0.iter().map(|x| x.1).collect::<Vec<_>>(),
            vec![10, -1, 2, 0]
        );
        circ_list.move_number(2);
        assert_eq!(
            circ_list.0.iter().map(|x| x.1).collect::<Vec<_>>(),
            vec![-1, 10, 2, 0]
        );
    }

    #[test]
    fn calculate_delta() {
        assert_eq!(CircularList::calculate_delta(5, 2, 2), -2);
        assert_eq!(CircularList::calculate_delta(5, 2, 6), -2);
        assert_eq!(CircularList::calculate_delta(5, 2, 10), -2);
        assert_eq!(CircularList::calculate_delta(5, 2, 1), 1);
        assert_eq!(CircularList::calculate_delta(5, 2, -2), -2);
        assert_eq!(CircularList::calculate_delta(5, 2, -3), 1);
        assert_eq!(CircularList::calculate_delta(5, 2, -7), 1);
        assert_eq!(CircularList::calculate_delta(5, 0, 0), 0);
        assert_eq!(CircularList::calculate_delta(5, 2, 0), 0);
        assert_eq!(CircularList::calculate_delta(5, 4, 0), 0);
    }

    #[test]
    fn part1() {
        let day = AocDay20::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part1(), 3);
    }

    #[test]
    fn part2() {
        let day = AocDay20::preprocessing(INPUT.iter().map(|x| String::from(*x)));
        assert_eq!(day.part2(), 1623178306);
    }
}
