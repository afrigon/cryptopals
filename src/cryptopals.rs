mod set1 {
    use crate::*;
    use std::fs;

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
        let expected =
            unhex(b"436f6f6b696e67204d432773206c696b65206120706f756e64206f66206261636f6e");

        let mut min = 1.0_f32;
        let mut message: Vec<u8> = Vec::new();

        for x in 0..=0xff {
            let right = [x];
            let result = xor(cipher.as_slice(), &right);
            let frequencies = ascii_frequency(result.as_slice());
            let score = is_english(&frequencies);

            if score < min {
                min = score;
                message = result
            }
        }

        assert_eq!(message, expected);
    }

    #[test]
    fn challenge4() {
        let data =
            fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "res/s1c4")).unwrap();
        let ciphers = data.split('\n');
        let expected = unhex(b"4e6f77207468617420746865207061727479206973206a756d70696e670a");

        let mut min = 1.0_f32;
        let mut message: Vec<u8> = Vec::new();

        for cipher in ciphers {
            for x in 0..=0xff {
                let right = [x];
                let result = xor(unhex(cipher.as_bytes()).as_slice(), &right);
                let frequencies = ascii_frequency(result.as_slice());
                let score = is_english(&frequencies);

                if score < min {
                    min = score;
                    message = result
                }
            }
        }

        assert_eq!(message, expected);
    }

    #[test]
    fn challenge5() {
        let key = b"ICE";
        let message =
            b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected =
            unhex(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

        let cipher = xor(message, key);

        assert_eq!(cipher, expected);
    }
}
