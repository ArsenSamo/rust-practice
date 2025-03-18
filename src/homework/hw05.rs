pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ((28, 64), 4),
            ((18, 9), 9),
            ((14, 7), 7),
            ((150, 50), 50),
            ((30, 18), 6),
            ((110, 10), 10),
            ((130, 85), 5),
            ((90, 150), 30),
            ((105, 21), 21),
            ((41, 13), 1),
            ((125, 95), 5),
        ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }
}
