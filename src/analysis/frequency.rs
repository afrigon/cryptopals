pub fn ascii_frequency(input: &[u8]) -> [f32; 26] {
    let len = input.len();
    let mut output = [0.0_f32; 26];

    if len == 0 {
        return output;
    }

    for i in 0..len {
        if !input[i].is_ascii_alphanumeric()
            && !input[i].is_ascii_whitespace()
            && !input[i].is_ascii_punctuation()
        {
            return [0.0_f32; 26];
        }

        if !input[i].is_ascii_alphabetic() {
            continue;
        }

        output[input[i].to_ascii_lowercase() as usize - 0x61] += 1.0;
    }

    for i in 0..26 {
        output[i] /= len as f32;
    }

    output
}

const ENGLISH_FREQUENCIES: [f32; 26] = [
    0.084966, 0.02072, 0.045388, 0.033844, 0.1116, 0.01812, 0.0247, 0.03003, 0.075448, 0.00196,
    0.01101, 0.054893, 0.0301, 0.066544, 0.071635, 0.031671, 0.0019, 0.075809, 0.057351, 0.069509,
    0.036308, 0.01007, 0.01289, 0.0029, 0.01777, 0.00272,
];

/// Return how close a frequency table is to the english frequency table
/// 0 is closest to english.
pub fn is_english(input: &[f32; 26]) -> f32 {
    input
        .iter()
        .zip(ENGLISH_FREQUENCIES.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}
