use std::io::{self, Write};
use std::fs::File;
use rsa::{pkcs8::{ToPrivateKey, ToPublicKey}, RsaPrivateKey, RsaPublicKey};
pub fn save_keys(private_key: &RsaPrivateKey, public_key: &RsaPublicKey) -> io::Result<()> {
    let private_key_pem = private_key.to_pkcs8_pem().expect("Failed to convert private key to PEM");
    let public_key_pem = public_key.to_public_key_pem().expect("Failed to convert public key to PEM");
    let mut private_file = File::create("private_key.pem")?;
    private_file.write_all(private_key_pem.as_bytes())?;
    let mut public_file = File::create("public_key.pem")?;
    public_file.write_all(public_key_pem.as_bytes())?;
    Ok(())
}