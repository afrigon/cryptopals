mod xor;

pub use self::xor::{find_repeating_xor_key, find_single_byte_xor, xor};

pub trait Encrypt {
    fn encrypt(&self, message: &[u8]) -> Vec<u8>;
}

pub trait Decrypt {
    fn decrypt(&self, cipher: &[u8]) -> Vec<u8>;
}
