use aes::Aes256;
use aes::cipher::KeyIvInit;
use cipher::StreamCipher;
use base64::{encode, decode};
use rand::Rng;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;

type Aes256Ctr = ctr::Ctr128BE<Aes256>;


const PBKDF2_ITERATIONS: u32 = 100_000;

fn derive_key(passkey: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(passkey.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

pub fn encrypt(text: &str, passkey: &str) -> String {
    let salt: [u8; 16] = rand::thread_rng().gen();
    let key = derive_key(passkey, &salt);
    let mut buffer = text.as_bytes().to_vec();
    let iv: [u8; 16] = rand::thread_rng().gen();
    let mut cipher = Aes256Ctr::new(&key.into(), &iv.into());
    cipher.apply_keystream(&mut buffer);
    let mut result = salt.to_vec();
    result.extend(&iv);
    result.extend(buffer);
    encode(&result)
}

pub fn decrypt(encoded_text: &str, passkey: &str) -> Result<String, Box<dyn std::error::Error>> {
    let buffer = decode(encoded_text).expect("Failed to decode base64");
    let (salt, rest) = buffer.split_at(16);
    let (iv, ciphertext) = rest.split_at(16);
    let key = derive_key(passkey, salt);
    let mut cipher = Aes256Ctr::new(&key.into(), iv.into());
    let mut decrypted_buffer = ciphertext.to_vec();
    cipher.apply_keystream(&mut decrypted_buffer);
    match String::from_utf8(decrypted_buffer) {
        Ok(text) => Ok(text),
        Err(e) => {
           // eprintln!("Failed to convert to String: {:?}", e);
            Err(Box::new(e))
        }
    }
}