use argon2::Argon2;
use ring::aead::{Aad, BoundKey, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use shared::EncryptedData;
use shared::NonceGen;

fn get_key(key: String) -> UnboundKey {
    let salt = "skibidi toilet".as_bytes(); // static salt
    let a2 = Argon2::default();
    let mut digest = [0u8; 32];
    a2.hash_password_into(key.as_bytes(), salt, &mut digest)
        .unwrap();

    UnboundKey::new(&AES_256_GCM, &digest).unwrap()
}

fn encrypt(key: String, data: Vec<u8>) -> EncryptedData {
    let key = get_key(key);

    let mut encrypted_data = EncryptedData {
        data,
        tag: [0u8; 16],
        nonce: NonceGen::new(),
    };
    let mut sealing_key = SealingKey::new(key, encrypted_data.nonce);

    let tag = sealing_key
        .seal_in_place_separate_tag(Aad::empty(), &mut encrypted_data.data)
        .unwrap();
    encrypted_data.tag.copy_from_slice(tag.as_ref());

    encrypted_data
}

fn decrypt(key: String, encrypted_data: EncryptedData) -> Vec<u8> {
    let key = get_key(key);
    let mut opening_key = OpeningKey::new(key, encrypted_data.nonce);

    let mut data = [encrypted_data.data, encrypted_data.tag.to_vec()].concat();
    opening_key
        .open_in_place(Aad::empty(), &mut data)
        .unwrap()
        .to_vec()
}

fn upload(filename: String, key: String) -> u32 {
    let data = std::fs::read(filename).unwrap();
    let enc = encrypt(key, data);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:8080/upload")
        .json(&enc)
        .send()
        .unwrap();

    let id: u32 = res.json().unwrap();
    id
}

fn download(key: String, id: u32) -> Vec<u8> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("http://localhost:8080/download/{}", id))
        .send()
        .unwrap();

    let enc: EncryptedData = res.json().unwrap();
    decrypt(key, enc)
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let method = &args[1];
    let filename = &args[2];

    if method == "upload" {
        let key = args[3].clone();
        let id = upload(filename.clone(), key);
        println!("Uploaded file with id: {}", id);
    } else if method == "download" {
        let key = args[3].clone();
        let id = args[4].parse::<u32>().unwrap();

        let data = download(key, id);
        std::fs::write(filename.clone(), data).unwrap();
        println!("Downloaded file with id: {} to file {}", id, filename);
    }
}
