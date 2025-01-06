use jsonwebtoken::{decode, Algorithm, Validation, DecodingKey, errors::Result};
use crate::calculate_current_time;
use crate::Claims;

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)?;
    let current_time = calculate_current_time();
    if current_time > token_data.claims.exp as u64{
        Err(jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken))?
    }
    Ok(token_data.claims)
}