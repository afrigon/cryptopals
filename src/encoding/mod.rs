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
        0x30 => 0,
        0x31 => 1,
        0x32 => 2,
        0x33 => 3,
        0x34 => 4,
        0x35 => 5,
        0x36 => 6,
        0x37 => 7,
        0x38 => 8,
        0x39 => 9,
        0x41 | 0x61 => 10,
        0x42 | 0x62 => 11,
        0x43 | 0x63 => 12,
        0x44 | 0x64 => 13,
        0x45 | 0x65 => 14,
        0x46 | 0x66 => 15,
        _ => panic!("your hex is broke mate"),
    }
}
