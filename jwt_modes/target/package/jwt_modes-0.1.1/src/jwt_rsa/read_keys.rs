use rsa::{pkcs8::{FromPrivateKey, FromPublicKey}, RsaPrivateKey, RsaPublicKey};
pub fn read_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let private_key_pem = std::fs::read_to_string("private_key.pem").expect("Failed to read private key file");
    let public_key_pem = std::fs::read_to_string("public_key.pem").expect("Failed to read public key file");
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem).expect("Failed to parse private key");
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem).expect("Failed to parse public key");
    (private_key, public_key)
}