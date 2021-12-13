#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    y: i32,
    x: i32,
}

#[derive(Debug, Clone, Copy)]
enum Fold {
    AlongX(i32),
    AlongY(i32),
}

fn main() {
    let mut positions = Vec::new();

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");

        if input.trim().len() == 0 {
            break;
        }

        let (x, y) = input
            .trim()
            .split_once(',')
            .expect("invalid position format");
        let (x, y): (i32, i32) = (
            x.parse().expect("x not a number"),
            y.parse().expect("y not a number"),
        );

        positions.push(Pos { x, y });
    }

    let mut folds = Vec::new();

    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read stdin");

        if input.trim().len() == 0 {
            break;
        }

        let input = input.trim().replace("fold along ", "");
        let (axis, amount) = input.split_once('=').expect("invalid fold format");
        let amount: i32 = amount.parse().expect("fold amount not a number");

        folds.push(match axis {
            "x" => Fold::AlongX(amount),
            "y" => Fold::AlongY(amount),
            _ => unreachable!("unknown axis"),
        });
    }

    // do one fold for part 1
    folds.iter().take(1).for_each(|&f| {
        fold(&mut positions, f);
    });

    positions.sort();
    positions.dedup();

    println!("Part 1: {}", positions.len());

    folds.iter().skip(1).for_each(|&f| {
        fold(&mut positions, f);
    });

    println!("Part 2:");
    print_dots(&positions);
}

fn fold(positions: &mut Vec<Pos>, fold: Fold) {
    positions.iter_mut().for_each(|pos| match fold {
        Fold::AlongX(x) => {
            if pos.x > x {
                pos.x = 2 * x - pos.x
            }
        }
        Fold::AlongY(y) => {
            if pos.y > y {
                pos.y = 2 * y - pos.y
            }
        }
    });
}

fn print_dots(positions: &Vec<Pos>) {
    let (max_x, max_y) = positions.iter().fold((0, 0), |(max_x, max_y), pos| {
        (max_x.max(pos.x), max_y.max(pos.y))
    });
    let (max_x, max_y) = (max_x + 1, max_y + 1);

    let mut pos_vec: Vec<bool> = Vec::with_capacity((max_x * max_y) as usize);

    for _ in 0..max_x * max_y {
        pos_vec.push(false);
    }

    positions
        .iter()
        .for_each(|pos| *pos_vec.get_mut((pos.x + pos.y * max_x) as usize).unwrap() = true);

    for y in 0..max_y {
        for x in 0..max_x {
            match pos_vec.get((x + y * max_x) as usize).unwrap() {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}
