use std::{
    convert::TryInto,
    io::{self, BufRead},
};

fn main() {
    let mut bit_count: u32 = 0;

    let vec: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            bit_count = l.len().try_into().unwrap();
            u64::from_str_radix(&l, 2).expect("must be a number")
        })
        .collect();

    let gamma_rate = calc_gamma_rate(&vec, bit_count);
    let epsilon_rate = 2_u64.pow(bit_count) - 1 - gamma_rate; // binary complement

    let (oxygen_gen_rating, co2_scrubber_rating) = calc_life_support_numbers(&vec, bit_count);

    println!("Part 1: {}", gamma_rate * epsilon_rate);
    println!("Part 2: {}", oxygen_gen_rating * co2_scrubber_rating);
}

fn calc_gamma_rate(nums: &Vec<u64>, bit_count: u32) -> u64 {
    let mut gamma_rate = 0;
    let nums_size = nums.len();

    for bit_index in 0..bit_count {
        let bit = 2_u64.pow(bit_index);

        let count_true = nums.iter().filter(|num| *num & bit > 0).count();

        if count_true > nums_size / 2 {
            gamma_rate |= bit; // set bit at bit_index to 1
        }
    }

    gamma_rate
}

fn calc_life_support_numbers(nums: &Vec<u64>, bit_count: u32) -> (u64, u64) {
    let mut o2_nums: Vec<u64> = nums.clone();
    let mut co2_nums: Vec<u64> = nums.clone();

    for bit_index in (0..bit_count).rev() {
        let bit = 2_u64.pow(bit_index);

        if o2_nums.len() > 1 {
            let count_true = o2_nums.iter().filter(|num| *num & bit > 0).count();
            let most_common_o2 = 2 * count_true >= o2_nums.len();

            o2_nums = o2_nums
                .into_iter()
                .filter(|num| (*num & bit > 0) == most_common_o2)
                .collect();
        }

        if co2_nums.len() > 1 {
            let count_true = co2_nums.iter().filter(|num| *num & bit > 0).count();
            let least_common_co2 = 2 * count_true < co2_nums.len();
            co2_nums = co2_nums
                .into_iter()
                .filter(|num| (*num & bit > 0) == least_common_co2)
                .collect();
        }
    }

    (
        *o2_nums.iter().next().unwrap(),
        *co2_nums.iter().next().unwrap(),
    )
}
