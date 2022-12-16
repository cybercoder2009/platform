use md5 as _md5;
use sha3::{Digest, Sha3_256};

pub fn md5(input: &str) -> String {
    format!("{:x}", _md5::compute(input))
}

pub fn sha3_256(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input);
    format!("{:x}", hasher.finalize())
}