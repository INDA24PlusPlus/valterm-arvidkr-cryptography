use argon2::Argon2;
use ring::{
    aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM},
    rand::{SecureRandom, SystemRandom},
};
use shared::EncryptedData;
use shared::NonceGen;
use shared::NonceSequence;

#[derive(Debug, Copy, Clone)]

#[derive(Debug)]

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

fn main() {
    let data = "Jag älskar att spela fortnite".as_bytes().to_vec();
    let enc = encrypt("password123".into(), data);
    println!("Encrypted data: {:?}", enc);
    let dec = decrypt("password123".into(), enc);
    println!("Decrypted data: {}", String::from_utf8(dec).unwrap());
}
