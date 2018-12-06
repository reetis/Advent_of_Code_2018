use super::*;

pub fn calculate_overlapped_area(claims: &Vec<Claim>) -> Area {
    let mut fabric = Fabric::new();

    claims.iter()
        .for_each(|claim| { fabric.take(claim); });


    fabric.count_used_area(|it| { it > &1u32 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![
            Claim::new(1, 3, 1, 4, 4),
            Claim::new(2, 1, 3, 4, 4),
            Claim::new(3, 5, 5, 2, 2),
        ];
        let result = calculate_overlapped_area(&input);
        assert_eq!(result, 4);
    }
}