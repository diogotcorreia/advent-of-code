use std::{cell::RefCell, collections::VecDeque, io::BufRead, rc::Rc};

use itertools::Itertools;

#[derive(Debug)]
enum Number {
    Pair(Box<Number>, Box<Number>),
    Value(Rc<RefCell<i32>>),
}

impl Number {
    fn from(input: &Vec<char>) -> Number {
        Number::from_internal(input, 0, input.len())
    }

    fn from_internal(input: &Vec<char>, start: usize, end: usize) -> Number {
        if *input.get(start).unwrap() != '[' {
            // is regular value
            return Number::Value(Rc::new(RefCell::new(
                input[start..end]
                    .into_iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )));
        }

        let mut last_start = 1;
        let mut i = 1;
        let mut depth = 0;
        let mut children: Vec<Number> = Vec::new();
        loop {
            let char_at = input.get(start + i).unwrap();
            match char_at {
                '[' => depth += 1,
                ',' => {
                    if depth == 0 {
                        children.push(Number::from_internal(input, start + last_start, start + i));
                        last_start = i + 1;
                    }
                }
                ']' => {
                    if depth == 0 {
                        children.push(Number::from_internal(input, start + last_start, start + i));
                        break;
                    } else {
                        depth -= 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let (last, first) = (children.pop().unwrap(), children.pop().unwrap());

        Number::Pair(Box::new(first), Box::new(last))
    }

    fn magnitude(&self) -> i64 {
        match self {
            Number::Pair(left, right) => {
                3 * left.as_ref().magnitude() + 2 * right.as_ref().magnitude()
            }
            Number::Value(val) => *val.borrow() as i64,
        }
    }
}

impl Clone for Number {
    fn clone(&self) -> Number {
        match self {
            Number::Pair(left, right) => Number::Pair(
                Box::new(left.as_ref().clone()),
                Box::new(right.as_ref().clone()),
            ),
            Number::Value(v) => Number::Value(Rc::new(RefCell::new(*v.borrow()))),
        }
    }
}

fn main() {
    let mut numbers: VecDeque<Number> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| Number::from(&line.unwrap().trim().chars().collect()))
        .collect();

    let numbers_p2: Vec<Number> = numbers.iter().map(|v| v.clone()).collect();

    let mut result = numbers.pop_front().unwrap();
    while explode(&mut result) || split(&mut result) {}
    while numbers.len() > 0 {
        let next_number = numbers.pop_front().unwrap();

        result = Number::Pair(Box::new(result), Box::new(next_number));

        while explode(&mut result) || split(&mut result) {}
    }

    let mut max_magnitude = 0;

    for mut combs in numbers_p2.into_iter().permutations(2) {
        let (b, mut a) = (combs.pop().unwrap(), combs.pop().unwrap());
        while explode(&mut a) || split(&mut a) {}

        a = Number::Pair(Box::new(a), Box::new(b));

        while explode(&mut a) || split(&mut a) {}

        let mag = a.magnitude();
        if mag > max_magnitude {
            max_magnitude = mag;
        }
    }

    println!("Part 1: {}", result.magnitude());
    println!("Part 2: {}", max_magnitude);
}

fn explode(number: &mut Number) -> bool {
    explode_aux(number, 0, &mut None, &mut None)
}

fn explode_aux(
    number: &mut Number,
    depth: i32,
    left_rn: &mut Option<Rc<RefCell<i32>>>,
    right_rn: &mut Option<i32>,
) -> bool {
    match number {
        Number::Pair(left, right) => {
            if depth >= 4 && right_rn.is_none() {
                let left = match left.as_ref() {
                    Number::Value(i) => *i.borrow(),
                    _ => unreachable!(),
                };
                let right = match right.as_ref() {
                    Number::Value(i) => *i.borrow(),
                    _ => unreachable!(),
                };
                if let Some(left_rn) = left_rn {
                    *left_rn.borrow_mut() += left;
                }
                *right_rn = Some(right);
            } else {
                let result_left = explode_aux(left.as_mut(), depth + 1, left_rn, right_rn);
                if result_left && right_rn.is_none() {
                    return true;
                }

                let result_right = explode_aux(right.as_mut(), depth + 1, left_rn, right_rn);

                return result_left || result_right;
            }
        }
        Number::Value(value) => {
            if let Some(v) = right_rn {
                *value.borrow_mut() += *v;
                *right_rn = None;
                return true;
            } else {
                *left_rn = Some(Rc::clone(value));
                return false;
            }
        }
    };
    *number = Number::Value(Rc::new(RefCell::new(0)));
    true
}

fn split(number: &mut Number) -> bool {
    match number {
        Number::Pair(left, right) => {
            if split(left.as_mut()) {
                true
            } else {
                split(right.as_mut())
            }
        }
        Number::Value(value) => {
            let v = *value.borrow();
            if v >= 10 {
                let (left, right) = (v / 2, (v + 1) / 2);
                let (left, right) = (
                    Number::Value(Rc::new(RefCell::new(left))),
                    Number::Value(Rc::new(RefCell::new(right))),
                );
                *number = Number::Pair(Box::new(left), Box::new(right));
                true
            } else {
                false
            }
        }
    }
}
