extern crate advent_of_code;

use std::env;

use advent_of_code::day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Missing filename");

    day1(filename);
}

fn day1(filename: &String) {
    let input = day1::read_file(filename);
    let result1 = day1::first_part::calculate_final_frequency(&input);
    let result2 = day1::second_part::calculate_first_repeating_freq(&input);

    println!("First part answer is {}", result1);
    println!("Second part answer is {}", result2);
}