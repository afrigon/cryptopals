use crate::analysis::{ascii_frequency, hamming_distance, is_english};

pub fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    let left_len = left.len();

    if left_len == 0 {
        return right.to_vec();
    }

    let right_len = right.len();

    if right_len == 0 {
        return left.to_vec();
    }

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

pub fn find_single_byte_xor(input: &[u8]) -> (u8, f32) {
    let scores = (0..=0xff)
        .map(|n| xor(input, &[n]))
        .map(|n| ascii_frequency(n.as_slice()))
        .collect::<Vec<[f32; 26]>>()
        .iter()
        .map(is_english)
        .collect::<Vec<f32>>();

    let mut min: f32 = 1.0;
    let mut byte: u8 = 0x00;

    for (i, score) in scores.iter().enumerate() {
        if *score < min {
            min = *score;
            byte = i as u8;
        }
    }

    (byte, min)
}

pub fn find_repeating_xor_key(input: &[u8]) -> Vec<u8> {
    let len = input.len();
    let upper_bound = if len / 4 < 40 { len / 5 } else { 40 };

    let mut min: f32 = 10000000.0;
    let mut keylen = 0;

    for i in 2..upper_bound {
        let value = (hamming_distance(&input[0..i], &input[i..2 * i]) as f32 / i as f32
            + hamming_distance(&input[0..i], &input[2 * i..3 * i]) as f32 / i as f32
            + hamming_distance(&input[0..i], &input[3 * i..4 * i]) as f32 / i as f32
            + hamming_distance(&input[i..2 * i], &input[2 * i..3 * i]) as f32 / i as f32
            + hamming_distance(&input[i..2 * i], &input[3 * i..4 * i]) as f32 / i as f32
            + hamming_distance(&input[2 * i..3 * i], &input[3 * i..4 * i]) as f32 / i as f32)
            / 6.0;

        if value < min {
            min = value;
            keylen = i;
        }
    }

    let mut key = Vec::new();

    for i in 0..keylen {
        let block = input
            .chunks(keylen)
            .take(len / keylen)
            .map(|n| n[i])
            .collect::<Vec<u8>>();

        key.push(find_single_byte_xor(block.as_slice()).0);
    }

    key
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
