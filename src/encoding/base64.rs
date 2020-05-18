use crate::encoding::Encode;

/// https://en.wikipedia.org/wiki/Base64#Base64_table
const CHAR_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

pub struct Base64;

impl Encode for Base64 {
    fn encode(input: &[u8]) -> String {
        let mut output: String = String::new();
        let len = input.len();
        let chunk_count = len / 3;
        let last_len = len % 3;

        for i in 0..chunk_count {
            encode(
                (input[i * 3] as u32) << 16
                    | (input[i * 3 + 1] as u32) << 8
                    | input[i * 3 + 2] as u32,
                &mut output,
            );
        }

        // trash dumpster
        if last_len == 1 {
            output.push(CHAR_TABLE[(input[len - 1] >> 2) as usize]);
            output.push(CHAR_TABLE[((input[len - 1] & 0b11) << 4) as usize]);
            output.push(PADDING);
            output.push(PADDING);
        } else if last_len == 2 {
            output.push(CHAR_TABLE[(input[len - 2] >> 2) as usize]);
            output.push(CHAR_TABLE[((input[len - 2] & 0b11) << 4 | input[len - 1] >> 4) as usize]);
            output.push(CHAR_TABLE[((input[len - 1] & 0b1111) << 2) as usize]);
            output.push(PADDING);
        }

        output
    }
}

fn encode(input: u32, output: &mut String) {
    output.push(CHAR_TABLE[(input >> 18 & 0b111111) as usize]);
    output.push(CHAR_TABLE[(input >> 12 & 0b111111) as usize]);
    output.push(CHAR_TABLE[(input >> 6 & 0b111111) as usize]);
    output.push(CHAR_TABLE[(input & 0b111111) as usize]);
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
