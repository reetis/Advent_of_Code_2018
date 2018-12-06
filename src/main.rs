extern crate advent_of_code;

use std::env;

use advent_of_code::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args.get(1)
        .map(|it| it.parse().expect("First argument must be a number"))
        .expect("Missing day number");
    let filename = args.get(2).expect("Missing filename");

    match day {
        1 => day1(filename),
        2 => day2(filename),
        3 => day3(filename),
        4 => unimplemented!(),
        5 => unimplemented!(),
        6 => unimplemented!(),
        7 => unimplemented!(),
        8 => unimplemented!(),
        9 => unimplemented!(),
        10 => unimplemented!(),
        11 => unimplemented!(),
        12 => unimplemented!(),
        13 => unimplemented!(),
        14 => unimplemented!(),
        15 => unimplemented!(),
        16 => unimplemented!(),
        17 => unimplemented!(),
        18 => unimplemented!(),
        19 => unimplemented!(),
        20 => unimplemented!(),
        21 => unimplemented!(),
        22 => unimplemented!(),
        23 => unimplemented!(),
        24 => unimplemented!(),
        25 => unimplemented!(),
        _ => panic!("Advent has only 25 days!")
    };
}

fn day1(filename: &String) {
    let input = day1::read_file(filename);
    let result1 = day1::first_part::calculate_final_frequency(&input);
    let result2 = day1::second_part::calculate_first_repeating_freq(&input);

    println!("First part answer is {}", result1);
    println!("Second part answer is {}", result2);
}

fn day2(filename: &String) {
    let input = day2::read_file(filename);
    let result1 = day2::first_part::calculate_checksum(&input);
    let result2 = day2::second_part::find_common_letters(&input)
        .iter()
        .fold(String::new(), |curr, chr| curr + &format!("{}", chr));

    println!("First part answer is {}", result1);
    println!("Second part answer is {}", result2);
}

fn day3(filename: &String) {
    let input = day3::read_file(filename);
    let result1 = day3::first_part::calculate_overlapped_area(&input);
    let result2 = day3::second_part::find_non_overlapped_claim(&input);

    println!("First part answer is {}", result1);
    println!("Second part answer is {}", result2.id());
}