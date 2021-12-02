use std::io;

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Movement {
    fn from_line(str: String) -> Movement {
        let splits = Vec::from_iter(str.split_whitespace());
        let direction = splits.get(0).expect("Could not get direction");
        let value = String::from(*splits.get(1).expect("Could not get value"))
            .trim()
            .parse()
            .expect("Could not convert to int");

        if *direction == "forward" {
            return Movement::Forward(value);
        } else if *direction == "down" {
            return Movement::Down(value);
        } else if *direction == "up" {
            return Movement::Up(value);
        } else {
            panic!("Unknown direction");
        }
    }
}

fn main() {
    let mut horiz_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut depth_by_aim = 0;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        if input.trim().len() == 0 {
            break;
        }

        let mov = Movement::from_line(input);

        match mov {
            Movement::Up(v) => {
                depth -= v;
                aim -= v;
            }
            Movement::Down(v) => {
                depth += v;
                aim += v;
            }
            Movement::Forward(v) => {
                horiz_pos += v;
                depth_by_aim += aim * v;
            }
        }
    }

    println!("Part 1: {}", horiz_pos * depth);
    println!("Part 2: {}", horiz_pos * depth_by_aim);
}
