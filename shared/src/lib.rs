use serde::Deserialize;
use ring::{
    aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM},
    rand::{SecureRandom, SystemRandom},
};

#[derive(Deserialize)]
pub struct FileUpload {
    pub filedata: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
pub struct EncryptedData {
    pub data: Vec<u8>,
    pub tag: [u8; 16],
    pub nonce: NonceGen,
}

#[derive(Debug)]
pub struct NonceGen([u8; 12]);

impl NonceGen {
    pub fn new() -> Self {
        let r = SystemRandom::new();
        let mut nonce = [0u8; 12];
        r.fill(&mut nonce).unwrap();
        NonceGen(nonce)
    }
}

impl NonceSequence for NonceGen {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        Nonce::try_assume_unique_for_key(&self.0)
    }
}
