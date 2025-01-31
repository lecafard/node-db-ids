mod hashing;
use std::{convert::TryInto, num::NonZeroUsize};

use aes::{
    cipher::{generic_array::GenericArray, BlockDecryptMut, BlockEncryptMut, KeyInit},
    Aes128,
};
use base62::{decode, encode};
use crc::{Crc, CRC_32_ISO_HDLC};
use hashing::hmac_sha256_128;
use lru::LruCache;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmIdGenerator {
    secret: String,
    iv_cache: LruCache<String, Aes128>,
    crc: Crc<u32>,
}

#[wasm_bindgen]
impl WasmIdGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str) -> Self {
        Self {
            secret: key.to_string(),
            iv_cache: LruCache::new(NonZeroUsize::new(1024).unwrap()),
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC),
        }
    }

    pub fn encode(&mut self, t: &str, i: u64, g: u32) -> Result<String, JsValue> {
        let mut input_bytes = [0u8; 16];
        input_bytes[..8].copy_from_slice(&i.to_le_bytes());
        input_bytes[8..12].copy_from_slice(&g.to_le_bytes());
        let sum = self.crc.checksum(&input_bytes[..12]);
        input_bytes[12..].copy_from_slice(&sum.to_le_bytes());
        let mut cipher = self
            .get_key(t)
            .ok_or_else(|| JsValue::from_str("Failed to initialize cipher."))?;
        let mut array = GenericArray::from_mut_slice(&mut input_bytes);
        cipher.encrypt_block_mut(&mut array);
        Ok(format!(
            "{}_{}",
            t,
            encode(u128::from_be_bytes(input_bytes))
        ))
    }

    pub fn decode(&mut self, input: &str) -> Result<Vec<JsValue>, JsValue> {
        let (t, encoded_value) = input
            .rsplit_once('_')
            .ok_or_else(|| JsValue::from_str("Invalid input format"))?;
        // magic number for base62
        if encoded_value.len() > 22 {
            return Err(JsValue::from_str("Invalid input format"));
        }
        let mut input_bytes = decode(encoded_value)
            .map_err(|_| JsValue::from_str("Failed to decode input."))?
            .to_be_bytes();
        let mut cipher = self
            .get_key(t)
            .ok_or_else(|| JsValue::from_str("Failed to initialize cipher."))?;
        let mut array = GenericArray::from_mut_slice(&mut input_bytes);
        cipher.decrypt_block_mut(&mut array);
        let sum = self.crc.checksum(&array[..12]);
        if sum != u32::from_le_bytes(array[12..16].try_into().unwrap()) {
            return Err(JsValue::from_str("Integrity check failed."));
        }

        // Convert the decrypted bytes back into a u64 (little-endian)
        let i = u64::from_le_bytes(
            input_bytes[..8]
                .try_into()
                .map_err(|_| JsValue::from_str("Failed to convert decrypted bytes to u64."))?,
        );
        let g = u32::from_le_bytes(
            input_bytes[8..12]
                .try_into()
                .map_err(|_| JsValue::from_str("Failed to convert decrypted bytes to u32."))?,
        );

        Ok(vec![JsValue::from(t), JsValue::from(i), JsValue::from(g)])
    }

    fn get_key(&mut self, t: &str) -> Option<&Aes128> {
        if !self.iv_cache.contains(t) {
            let key = hmac_sha256_128(self.secret.as_bytes(), t.as_bytes());
            let k = Aes128::new_from_slice(&key).ok()?;
            self.iv_cache.put(t.to_string(), k);
        }
        self.iv_cache.get(t)
    }
}
