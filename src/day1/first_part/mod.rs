pub fn calculate_final_frequency(frequencies: &Vec<i64>) -> i64 {
    frequencies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![1, -2, 3, 1];
        let result = calculate_final_frequency(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let input = vec![1, 1, 1];
        let result = calculate_final_frequency(&input);
        assert_eq!(result, 3, );
    }

    #[test]
    fn test3() {
        let input = vec![1, 1, -2];
        let result = calculate_final_frequency(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test4() {
        let input = vec![-1, -2, -3];
        let result = calculate_final_frequency(&input);
        assert_eq!(result, -6);
    }
}