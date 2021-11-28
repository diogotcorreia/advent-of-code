use std::io;

fn main() {
    let mut input = String::from("");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: Vec<u32> = input
        .trim()
        .chars()
        .map(|v| String::from(v).parse().expect("Char not a number"))
        .collect();

    let mut sum: u32 = 0;
    let mut sum_halfway: u32 = 0;

    for (i, num) in input.iter().enumerate() {
        let next = match input.get(i + 1) {
            Some(v) => v,
            None => input.get(0).expect("Empty input"),
        };

        let next_halfway = input
            .get((input.len() / 2 + i) % input.len())
            .expect("List not even");

        if next == num {
            sum += num;
        }

        if next_halfway == num {
            sum_halfway += num;
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_halfway);
}
