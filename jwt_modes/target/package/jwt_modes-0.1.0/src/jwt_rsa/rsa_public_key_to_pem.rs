use rsa::{pkcs8::ToPublicKey, RsaPublicKey};
pub fn rsa_public_key_to_pem(public_key: &RsaPublicKey) -> String {
    let public_key_pem = public_key.to_public_key_pem().expect("Failed to convert public key to PEM");
    public_key_pem.to_string()
}