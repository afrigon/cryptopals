pub fn hamming_distance(left: &[u8], right: &[u8]) -> usize {
    let left_len = left.len();
    let right_len = right.len();

    let len = if left_len < right_len {
        left_len
    } else {
        right_len
    };

    let mut distance = ((left_len as i64 - right_len as i64).abs() * 8) as usize;

    for i in 0..len {
        let mut value = left[i] ^ right[i];

        while value != 0 {
            distance += value as usize & 1;
            value >>= 1;
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_empty_strings_is_zero() {
        let left = b"";
        let right = b"";
        let expected = 0;

        assert_eq!(hamming_distance(left, right), expected);
    }

    #[test]
    fn hamming_distance_with_longer_left_returns_more_distance() {
        let left = b"AAAA";
        let right = b"";
        let expected = 32;

        assert_eq!(hamming_distance(left, right), expected);
    }

    #[test]
    fn hamming_distance_with_longer_right_returns_more_distance() {
        let left = b"";
        let right = b"BBBB";
        let expected = 32;

        assert_eq!(hamming_distance(left, right), expected);
    }

    #[test]
    fn hamming_distance_returns_different_bit_count() {
        let left = b"this is a test";
        let right = b"wokka wokka!!!";
        let expected = 37;

        assert_eq!(hamming_distance(left, right), expected);
    }
}
