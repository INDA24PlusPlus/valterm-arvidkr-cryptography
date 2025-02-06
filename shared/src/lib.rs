use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileUpload {
    pub filedata: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
struct EncryptedData {
    pub data: Vec<u8>,
    pub tag: [u8; 16],
    pub nonce: NonceGen,
}