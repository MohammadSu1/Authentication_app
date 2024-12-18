use rsa::{pkcs8::ToPrivateKey, RsaPrivateKey};
pub fn rsa_private_key_to_pem(private_key: &RsaPrivateKey) -> String {
    let private_key_pem = private_key.to_pkcs8_pem().expect("Failed to convert private key to PEM");
    private_key_pem.to_string()
}