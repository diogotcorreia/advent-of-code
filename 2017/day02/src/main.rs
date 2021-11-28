use itertools::Itertools;
use std::io;

fn main() {
    let mut sum: u32 = 0;
    let mut sum_division: u32 = 0;

    loop {
        let mut row = String::from("");
        if let Err(_) = io::stdin().read_line(&mut row) {
            break;
        }

        if row.trim().len() == 0 {
            break;
        }

        let row: Vec<u32> = row.split_whitespace().map(|v| v.parse().unwrap()).collect();

        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();

        sum += max - min;

        for vec in row.iter().combinations(2) {
            let upper = vec.iter().max().unwrap();
            let lower = vec.iter().min().unwrap();

            if *upper % *lower == 0 {
                sum_division += *upper / *lower;
                break;
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_division);
}
