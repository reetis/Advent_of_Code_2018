use std::fs;

pub fn read_file(filename: &String) -> Vec<i64> {
    println!("Reading file {}", filename);
    let file = fs::read_to_string(filename).expect("File not found");
    file.split_whitespace()
        .map(|it| -> i64 {
            match it.parse() {
                Ok(number) => number,
                Err(_) => panic!("Cannot parse `{}` into number", it)
            }
        })
        .collect()
}

pub mod first_part;
pub mod second_part;