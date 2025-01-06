use crate::Claims;
use std::time::{SystemTime, UNIX_EPOCH};
use rsa::RsaPrivateKey;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, errors::Result};
use super::rsa_private_key_to_pem;
use crate::calculate_current_time;
pub fn generate_jwt(private_key: &RsaPrivateKey, user_id: &str, email: &str) -> Result<String> {
    let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map_err(|_| jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?
    .as_secs() + 60;

    println!("Current time: {}", calculate_current_time());
    println!("Expiration time: {}", expiration);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        email: email.to_string(),
        iat: super::calculate_current_time() as usize,
    };

    let header = Header::new(Algorithm::RS256);
    let encoding_key = EncodingKey::from_rsa_pem(rsa_private_key_to_pem::rsa_private_key_to_pem(private_key).as_bytes())?;
    let token = encode(&header, &claims, &encoding_key)?;
    Ok(token)
}