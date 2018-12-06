use std::fs;

pub fn read_file(filename: &String) -> Vec<String> {
    println!("Reading file {}", filename);
    let file = fs::read_to_string(filename).expect("File not found");
    file.split_whitespace()
        .map(|it| {
            match it.parse() {
                Ok(id) => id,
                Err(_) => panic!("Cannot parse `{}` into number", it)
            }
        })
        .collect()
}

pub mod first_part;
pub mod second_part;
