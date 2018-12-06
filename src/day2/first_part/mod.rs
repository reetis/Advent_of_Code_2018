use std::collections::*;

pub fn calculate_checksum(ids: &Vec<String>) -> u32 {
    let mut two_counts: u32 = 0;
    let mut three_counts: u32 = 0;

    for id in ids {
        let counts: HashSet<u64> = id.chars()
            .fold(
                HashMap::new(),
                |mut map: HashMap<char, u64>, chr| {
                    let new_count = map.get(&chr).unwrap_or(&0).clone();
                    map.insert(chr, new_count + 1);
                    map
                },
            )
            .values()
            .map(|it| { it.clone() })
            .collect();

        if counts.contains(&2) { two_counts += 1; }
        if counts.contains(&3) { three_counts += 1; }
    }

    two_counts * three_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab")
        ];
        let result = calculate_checksum(&input);
        assert_eq!(result, 12);
    }
}