# cryptopals

Solutions to the [Cryptopals crypto challenges](https://cryptopals.com), written as a Rust library. Every primitive — encoding, XOR ciphers, frequency analysis — is implemented from scratch, and each challenge is solved as a test in `src/cryptopals.rs` that exercises the public API.

Do not use this code in production. The point of the challenges is to build the primitives yourself, not to build them securely.

## Usage

The crate is named `crypto` and exposes the building blocks the challenges are solved with:

```rust
use crypto::*;

// hex decoding and base64 encoding
let bytes = unhex(b"49276d206b696c6c696e6720796f7572");
let encoded = Base64::encode(bytes.as_slice());

// repeating-key XOR
let cipher = xor(b"Burning 'em, if you ain't quick and nimble", b"ICE");

// break single-byte XOR with english frequency analysis
let (key, score) = find_single_byte_xor(cipher.as_slice());

// recover the key of a repeating-key XOR cipher
let key = find_repeating_xor_key(cipher.as_slice());
```

The analysis primitives behind the attacks — `hamming_distance`, `ascii_frequency`, and `is_english` — are public as well.

## Development

```sh
mise install
mise run build
mise run test
```
