use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::Rng;

fn main() {
    let plaintext = b"Hello, world!";
    let binding = rand::thread_rng().gen::<[u8; 32]>();
    let key = Key::from_slice(&binding);
    let cipher = ChaCha20Poly1305::new(key);

    let binding = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&binding);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .expect("encryption failure!");
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");
    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}
