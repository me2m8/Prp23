
/// Basically, this contains the difference between n and the sum of the digits of 2 * n. for
/// example: n = 6, 2 * n = 12, 6 + 7 = 13 + 3 mod 10, which is also the sum if the digits of 12.
static EVAL_NUMBERS: [usize; 10] = [0, 1, 2, 3, 4, 6, 7, 8, 9, 0];

pub fn validate_pin(pin: &str) -> bool {
    // Oh boy what a doosy. We start by wrapping the pin in a single element iterator.
    std::iter::once(pin)
        // This removes the pin if it contains any non digit characters.
        .filter(|p| p.chars().all(|c| c.is_ascii_digit() || c == ' '))
        // This convers the string into a vector of numbers.
        .map(|p| {
            p.chars()
                .rev()
                .filter_map(|c| c.to_digit(10).map(|n| n as usize))
                .collect::<Vec<usize>>()
        })
        // This removes the pin if it is of the wrong length.
        .filter(|p| p.len() == 10 || p.len() == 12)
        // This flattens the single element iterator containing the pin into an iterator over the
        // numbers of the pin.
        .flatten()
        // We then take the first 10 numbers of the pin. This is equivalent to taking the last 10
        // of the original pin since we reversed it when convertingit to numbers.
        .take(10)
        // We then pair each number with its index in the pin.
        .enumerate()
        // We then fold over the pairs, summing the numbers according to luhns algorithm, with a
        // slight tweak. We add i + 1 for each number. At the end we subtract 55, which is the sum
        // of all number 1-10. This is a safeguard against the case where the pin is filtered out
        // earlier. Without this, filtered out pins would evaluate to 0 which is a valid pin.
        .fold(0, |acc, (i, n)| acc + n + EVAL_NUMBERS[n] * (i % 2) + i + 1)
        // If 55 can not be subtracted, this returns None.
        .checked_sub(55)
        // Map the case where its not none to a boolean.
        .map(|sum| sum % 10 == 0)
        // If the sum is None, the pin is invalid, so false is returned. otherwise the value
        // inside the option is returned, which in this case is a boolean indicating the validity of
        // the pin.
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pin() {
        assert!(validate_pin("0603179276"));
        assert!(validate_pin("0610092454"));
        assert!(validate_pin("7601010205"));
        assert!(!validate_pin("81121898765"));
        assert!(!validate_pin("8a121898765"));
        assert!(!validate_pin("f0610092454"));
        assert!(!validate_pin("12345"));
        assert!(!validate_pin(""));
    }
}
