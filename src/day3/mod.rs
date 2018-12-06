use std::fs;

use regex::Regex;

pub use self::claim::Claim;
pub use self::fabric::Fabric;
pub use self::point::Point;
pub use self::rectangle::Rectangle;

mod rectangle;
mod point;
mod claim;
mod fabric;
pub mod first_part;
pub mod second_part;

pub type Area = u32;

pub fn read_file(filename: &String) -> Vec<Claim> {
    println!("Reading file {}", filename);
    let file = fs::read_to_string(filename).expect("File not found");

    let pattern = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$")
        .expect("Invalid line pattern");

    file.split('\n')
        .filter(|it| { it.trim() != "" })
        .map(|it| { it.trim() })
        .map(|line| {
            pattern.captures(line)
                .map(|cap| {
                    Claim::new(
                        cap[1].parse().unwrap(),
                        cap[3].parse().unwrap(),
                        cap[2].parse().unwrap(),
                        cap[4].parse().unwrap(),
                        cap[5].parse().unwrap(),
                    )
                })
                .expect(&format!("Line `{}` is not valid format", line))
        })
        .collect()
}