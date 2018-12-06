pub fn find_common_letters(ids: &[String]) -> Vec<char> {
    ids.iter()
        .enumerate()
        .map(|(i, id)| { (id, &ids[(i + 1)..]) })
        .filter_map(|(id, check_set)| {
            check_set.iter()
                .find_map(|it| { find_common_string_letters(id, it) })
        })
        .nth(0)
        .expect("No matches found")
}

fn find_common_string_letters(string1: &str, string2: &str) -> Option<Vec<char>> {
    if string1.len() != string2.len() {
        return None;
    }

    let common_part: Vec<char> = string1
        .chars()
        .zip(string2.chars())
        .filter(|(a, b)| { a == b })
        .map(|(a, _)| { a })
        .collect();

    if common_part.len() == string1.len() - 1 {
        return Some(common_part);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz")
        ];
        let result = find_common_letters(&input);
        assert_eq!(result, vec!['f', 'g', 'i', 'j']);
    }
}