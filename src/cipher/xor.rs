pub fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    let left_len = left.len();
    let right_len = right.len();

    let mut ptr = 0;

    let len = if left_len > right_len {
        left_len
    } else {
        right_len
    };
    let short_len = if left_len > right_len {
        right_len
    } else {
        left_len
    };

    let long = if left_len > right_len { left } else { right };
    let short = if left_len > right_len { right } else { left };

    let mut output = Vec::with_capacity(len);

    for i in 0..len {
        output.push(long[i] ^ short[ptr]);
        ptr = (ptr + 1) % short_len;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_same_length_xors_every_char() {
        let left = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let right = b"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20";

        let expected = b"abcdefghijklmnopqrstuvwxyz".to_vec();

        assert_eq!(xor(left, right), expected);
    }

    #[test]
    fn xor_with_smaller_right_wraps_around() {
        let left = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let right = b"\x20";

        let expected = b"abcdefghijklmnopqrstuvwxyz".to_vec();

        assert_eq!(xor(left, right), expected);
    }

    #[test]
    fn xor_with_smaller_left_wraps_around() {
        let left = b"\x20";
        let right = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let expected = b"abcdefghijklmnopqrstuvwxyz".to_vec();

        assert_eq!(xor(left, right), expected);
    }
}
