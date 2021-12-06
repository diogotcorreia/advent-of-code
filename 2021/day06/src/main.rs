fn main() {
    let mut lanternfish = [0; 9];

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    input
        .split(',')
        .map(|v| v.parse().expect("not a number"))
        .for_each(|age: usize| lanternfish[age] += 1);

    for i in 0..256 {
        lanternfish = simulate_day(&lanternfish);

        if i == 79 {
            println!("Part 1: {}", lanternfish.iter().sum::<i64>());
        }
    }

    println!("Part 2: {}", lanternfish.iter().sum::<i64>());
}

fn simulate_day(lanternfish: &[i64; 9]) -> [i64; 9] {
    let mut new_day = [0; 9];

    for i in 0..9 {
        let day_count = lanternfish[i];
        if i == 0 {
            new_day[6] += day_count;
            new_day[8] += day_count;
        } else {
            new_day[i - 1] += day_count;
        }
    }

    new_day
}
