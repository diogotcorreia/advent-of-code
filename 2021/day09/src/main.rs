use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    let mut row_length = 0;

    std::io::stdin().lock().lines().for_each(|line| {
        let line = line.expect("failed to read stdin");
        line.trim()
            .chars()
            .map(|v| String::from(v).parse::<i32>().expect("not a number"))
            .for_each(|v| numbers.push(v));
        row_length = line.trim().len();
    });

    let mut total_risk_level = 0;
    let mut lowest_points = Vec::new();

    for (i, v) in numbers.iter().enumerate() {
        if let Some(risk_level) = get_risk_level(&numbers, *v, i, row_length as i32) {
            total_risk_level += risk_level;
            lowest_points.push(i);
        }
    }

    let mut basins_size: Vec<usize> = lowest_points
        .iter()
        .map(|&lowest_point| {
            let mut basin_pos = HashSet::new();
            expand_basin(
                &numbers,
                lowest_point,
                None,
                row_length as i32,
                &mut basin_pos,
            );

            basin_pos.len()
        })
        .collect();

    basins_size.sort();

    let three_max_sizes = basins_size
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, &v| acc * v as i32);

    println!("Part 1: {}", total_risk_level);
    println!("Part 2: {}", three_max_sizes);
}

fn get_risk_level(numbers: &Vec<i32>, value: i32, i: usize, row_length: i32) -> Option<i32> {
    let (x, y) = (i as i32 % row_length, i as i32 / row_length);

    if [
        numbers.get(xy_to_i(x - 1, y, row_length) as usize),
        numbers.get(xy_to_i(x + 1, y, row_length) as usize),
        numbers.get(xy_to_i(x, y - 1, row_length) as usize),
        numbers.get(xy_to_i(x, y + 1, row_length) as usize),
    ]
    .iter()
    .all(|adj| *adj.unwrap_or(&10) > value)
    {
        Some(value + 1)
    } else {
        None
    }
}

fn xy_to_i(x: i32, y: i32, row_length: i32) -> i32 {
    if x < 0 || x >= row_length {
        return -1;
    }
    x + y * row_length
}

fn expand_basin(
    numbers: &Vec<i32>,
    start_i: usize,
    from: Option<i32>,
    row_length: i32,
    basin_pos: &mut HashSet<usize>,
) {
    if !basin_pos.insert(start_i) {
        // already had value
        return;
    }

    let (x, y) = (start_i as i32 % row_length, start_i as i32 / row_length);

    [
        xy_to_i(x - 1, y, row_length) as usize,
        xy_to_i(x + 1, y, row_length) as usize,
        xy_to_i(x, y - 1, row_length) as usize,
        xy_to_i(x, y + 1, row_length) as usize,
    ]
    .iter()
    .filter(|&&i| i != from.unwrap_or(-1) as usize)
    .for_each(|&adj| {
        let val = numbers.get(adj).unwrap_or(&10);
        if *val < 9 {
            expand_basin(numbers, adj, Some(start_i as i32), row_length, basin_pos);
        }
    })
}
