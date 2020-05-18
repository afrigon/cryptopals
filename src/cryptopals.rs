mod set1 {
    use crate::*;

    #[test]
    fn challenge1() {
        let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        let hex = unhex(input);

        assert_eq!(Base64::encode(hex.as_slice()), expected);
    }

    #[test]
    fn challenge2() {
        let left = unhex(b"1c0111001f010100061a024b53535009181c");
        let right = unhex(b"686974207468652062756c6c277320657965");

        let expected = unhex(b"746865206b696420646f6e277420706c6179");

        assert_eq!(xor(left.as_slice(), right.as_slice()), expected);
    }

    #[test]
    fn challenge3() {
        let cipher = unhex(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let expected = b"Cooking MC's like a pound of bacon".to_vec();

        let mut max = 0;
        let mut m: Vec<u8> = Vec::new();

        for x in 0..=0xff {
            let right = [x];
            let result = xor(cipher.as_slice(), &right);
            let score = ascii_frequency(result.as_slice());

            if score > max {
                max = score;
                m = result
            }
        }
        let mut y = Vec::new();
        y[10] = 1;

        assert_eq!(m, expected);
    }
}
