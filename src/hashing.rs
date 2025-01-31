use sha2::{Sha256, Digest};
use std::iter::repeat;

const BLOCK_SIZE: usize = 64;

// They say to not impl your own crypto but i kinda did
pub fn hmac_sha256_128(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut key = key.to_vec();

    if key.len() > BLOCK_SIZE {
        key = Sha256::digest(&key).to_vec();
    }
    if key.len() < BLOCK_SIZE {
        key.extend(repeat(0).take(BLOCK_SIZE - key.len()));
    }

    let ipad: Vec<u8> = key.iter().map(|&b| b ^ 0x36).collect();
    let opad: Vec<u8> = key.iter().map(|&b| b ^ 0x5c).collect();

    let mut inner = Sha256::new();
    inner.update(&ipad);
    inner.update(message);
    let inner_hash = inner.finalize();

    let mut outer = Sha256::new();
    outer.update(&opad);
    outer.update(inner_hash);
    
    outer.finalize()[..16].to_vec()
}