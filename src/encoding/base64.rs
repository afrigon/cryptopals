use crate::encoding::Encode;

const CHAR_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

/// https://tools.ietf.org/html/rfc4648
pub struct Base64;

impl Encode for Base64 {
    fn encode(input: &[u8]) -> String {
        let len = input.len();
        let remainder = len % 3;

        let mut output: String = String::with_capacity(len / 3 * 4 + 3);

        for i in (0..len - remainder).step_by(3) {
            encode_part(&input[i..i + 3], &mut output);
        }

        if remainder != 0 {
            if remainder == 1 {
                output.push(CHAR_TABLE[first_subpart(input[len - 1]) as usize]);
                output.push(CHAR_TABLE[second_subpart(input[len - 1], 0) as usize]);
                output.push(PADDING);
            } else if remainder == 2 {
                output.push(CHAR_TABLE[first_subpart(input[len - 2]) as usize]);
                output.push(CHAR_TABLE[second_subpart(input[len - 2], input[len - 1]) as usize]);
                output.push(CHAR_TABLE[third_subpart(input[len - 1], 0) as usize]);
            }

            output.push(PADDING);
        }

        output
    }
}

fn encode_part(input: &[u8], output: &mut String) {
    output.push(CHAR_TABLE[first_subpart(input[0]) as usize]);
    output.push(CHAR_TABLE[second_subpart(input[0], input[1]) as usize]);
    output.push(CHAR_TABLE[third_subpart(input[1], input[2]) as usize]);
    output.push(CHAR_TABLE[fourth_subpart(input[2]) as usize]);
}

/// 11111100_00000000_00000000
fn first_subpart(a: u8) -> u8 {
    a >> 2
}

/// 00000011_11110000_00000000
fn second_subpart(a: u8, b: u8) -> u8 {
    (a & 0b11) << 4 | b >> 4
}

/// 00000000_00001111_11000000
fn third_subpart(b: u8, c: u8) -> u8 {
    (b & 0b1111) << 2 | c >> 6
}

/// 00000000_00000000_00111111
fn fourth_subpart(c: u8) -> u8 {
    c & 0b111111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encodes() {
        let input = b"I find your lack of faith disturbing";
        let expected = String::from("SSBmaW5kIHlvdXIgbGFjayBvZiBmYWl0aCBkaXN0dXJiaW5n");

        assert_eq!(Base64::encode(input), expected);
    }

    #[test]
    fn base64_encodes_with_padding() {
        let input = b"Hello there";
        let expected = String::from("SGVsbG8gdGhlcmU=");

        assert_eq!(Base64::encode(input), expected);
    }

    #[test]
    fn base64_encodes_with_double_padding() {
        let input = b"It's the ship that made the Kessel run in less than twelve parsecs.";
        let expected = String::from("SXQncyB0aGUgc2hpcCB0aGF0IG1hZGUgdGhlIEtlc3NlbCBydW4gaW4gbGVzcyB0aGFuIHR3ZWx2ZSBwYXJzZWNzLg==");

        assert_eq!(Base64::encode(input), expected);
    }
}
