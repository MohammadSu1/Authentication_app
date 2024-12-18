use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng: OsRng = OsRng;
    let bits = 2048;
    let private_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}