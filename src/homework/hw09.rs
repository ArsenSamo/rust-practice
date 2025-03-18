pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    let shift = (n % (len as isize) + (len as isize) * 2) as usize % len;
    let (left, right) = s.split_at(len - shift);
    right.to_string() + left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0, "abcdefgh"),
            (9, "habcdefg"),
            (-9, "bcdefgha"),
            (3, "fghabcde"),
            (12, "efghabcd"),
            (-3, "defghabc"),
            (-12, "cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string())
        });
    }
}
