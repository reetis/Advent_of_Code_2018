use super::*;

pub fn find_non_overlapped_claim(claims: &Vec<Claim>) -> &Claim {
    claims.iter()
        .find(|claim| {
            claims
                .iter()
                .filter(|it| { it.rectangle().intersection(claim.rectangle()).is_some() })
                .nth(1)
                .is_none()
        })
        .expect("All claims overlap")
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
        let result = find_non_overlapped_claim(&input);
        assert_eq!(result.id(), &3);
    }
}