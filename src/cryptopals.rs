mod set1 {
    use crate::encoding::{unhex, Base64, Encode};

    #[test]
    fn challenge1() {
        let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        let hex = unhex(input);

        assert_eq!(Base64::encode(hex.as_slice()), expected);
    }
}
