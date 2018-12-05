use std::collections::HashSet;

pub fn calculate_first_repeating_freq(frequencies: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;
    let mut existing_sums = HashSet::new();
    existing_sums.insert(sum);

    loop {
        for frequency in frequencies.iter() {
            sum += frequency;

            if existing_sums.contains(&sum) {
                return sum;
            }

            existing_sums.insert(sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![1, -1];
        let result = calculate_first_repeating_freq(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test2() {
        let input = vec![3, 3, 4, -2, -4];
        let result = calculate_first_repeating_freq(&input);
        assert_eq!(result, 10);
    }

    #[test]
    fn test3() {
        let input = vec![-6, 3, 8, 5, -6];
        let result = calculate_first_repeating_freq(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test4() {
        let input = vec![7, 7, -2, -7, -4];
        let result = calculate_first_repeating_freq(&input);
        assert_eq!(result, 14);
    }
}