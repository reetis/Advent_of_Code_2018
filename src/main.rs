extern crate advent_of_code;

use std::env;

use advent_of_code::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Missing filename");

    day2(filename);
}

#[allow(dead_code)]
fn day1(filename: &String) {
    let input = day1::read_file(filename);
    let result1 = day1::first_part::calculate_final_frequency(&input);
    let result2 = day1::second_part::calculate_first_repeating_freq(&input);

    println!("First part answer is {}", result1);
    println!("Second part answer is {}", result2);
}

#[allow(dead_code)]
fn day2(filename: &String) {
    let input = day2::read_file(filename);
    let result1 = day2::first_part::calculate_checksum(&input);
    let result2 = day2::second_part::find_common_letters(&input);

    println!("First part answer is {}", result1);
    println!("Second part answer is {:?}", result2);
}