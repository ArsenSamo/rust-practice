pub fn is_prime(n: &u32) -> bool {
    if *n <= 2 {
        return *n == 2;
    }
    if *n % 2 == 0 || *n % 5 == 0 {
        return false;
    }
    let mut i = 7;
    while i * i <= *n {
        if *n % i == 0 || *n % (i + 4) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (7, true),
            (101, true),
            (10009, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
    }
}
