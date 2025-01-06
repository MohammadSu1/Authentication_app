use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, errors::Result};
use rsa::RsaPublicKey;
use crate::Claims;
use super::rsa_public_key_to_pem;
use crate::calculate_current_time;
pub fn verify_jwt(token: &str, public_key: &RsaPublicKey) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;
    let decoding_key = DecodingKey::from_rsa_pem(rsa_public_key_to_pem::rsa_public_key_to_pem(public_key).as_bytes())?;
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    let current_time = calculate_current_time();
    if current_time > token_data.claims.exp as u64{
        Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?
    }
    Ok(token_data.claims)
}