use std::io;

fn main() {
    let mut increase_count: u32 = 0;
    let mut group_increase_count: u32 = 0;

    let mut prev_prev_number: Option<i32> = None;
    let mut prev_number: Option<i32> = None;
    let mut last_group_sum: Option<i32> = None;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        if input.trim().len() == 0 {
            break;
        }

        let input: i32 = input.trim().parse().expect("Failed to convert to int");

        if let Some(prev) = prev_number {
            if prev < input {
                increase_count += 1;
            }
            if let Some(prev_prev) = prev_prev_number {
                if let Some(last_sum) = last_group_sum {
                    if last_sum < input + prev + prev_prev {
                        group_increase_count += 1;
                    }
                }

                last_group_sum = Some(prev_prev + prev + input);
            }
        }

        prev_prev_number = prev_number;
        prev_number = Some(input);
    }

    println!("Part 1: {}", increase_count);
    println!("Part 2: {}", group_increase_count);
}
