mod base64;

pub use self::base64::Base64;

pub trait Decode {
    fn decode(input: &[u8]) -> String;
}

pub trait Encode {
    fn encode(input: &[u8]) -> String;
}

pub fn unhex(input: &[u8]) -> Vec<u8> {
    assert!(input.len() % 2 == 0);

    input
        .chunks(2)
        .map(|n| match_nibble(n[0]) << 4 | match_nibble(n[1]))
        .collect()
}

fn match_nibble(input: u8) -> u8 {
    match input {
        0x30..=0x39 => input - 0x30,
        0x41..=0x46 => input - 0x37,
        0x61..=0x66 => input - 0x57,
        _ => panic!("your hex is broke mate"),
    }
}
