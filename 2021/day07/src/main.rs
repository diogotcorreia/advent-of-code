fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let crabs: Vec<i32> = input
        .trim()
        .split(",")
        .map(|v| v.parse().expect("input not a number"))
        .collect();

    let fuel_part1 = find_fuel_to_most_efficient_position(&crabs, |distance| distance);
    let fuel_part2 =
        find_fuel_to_most_efficient_position(&crabs, |distance| (distance * (distance + 1)) / 2);

    println!("Part 1: {}", fuel_part1);
    println!("Part 2: {}", fuel_part2);
}

fn find_fuel_to_most_efficient_position(
    crabs: &Vec<i32>,
    fuel_modifier: impl Fn(i32) -> i32,
) -> i32 {
    let min = crabs.iter().min().unwrap_or(&0);
    let max = crabs.iter().max().unwrap_or(&0);

    let mut min_fuel = None;

    for i in *min..=*max {
        let fuel_for_pos: i32 = crabs.iter().map(|p| fuel_modifier((*p - i).abs())).sum();

        match min_fuel {
            None => {
                min_fuel = Some(fuel_for_pos);
            }
            Some(old_fuel) => {
                if fuel_for_pos < old_fuel {
                    min_fuel = Some(fuel_for_pos);
                }
            }
        }
    }

    min_fuel.expect("could not get puzzle answer")
}
