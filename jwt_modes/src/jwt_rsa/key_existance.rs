use std::path::Path;

use rsa::{RsaPrivateKey, RsaPublicKey};
pub fn key_existance() -> (RsaPrivateKey, RsaPublicKey) {
    let key_path = Path::new("private_key.pem");
    let (private_key, public_key) = if key_path.exists() {
        super::read_keys::read_keys()
    } else {
        let (private_key, public_key) = super::generate_keys::generate_keys();
        super::save_keys::save_keys(&private_key, &public_key)
            .expect("Failed to save keys");
        (private_key, public_key)
    };

    (private_key, public_key)
}