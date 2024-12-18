use jsonwebtoken::{encode, Header, Algorithm, EncodingKey, errors::Result};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::calculate_current_time;
use crate::Claims;

pub fn generate_jwt(secret: &str, user_id: &str, email: &str) -> Result<String> {
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
        iat: calculate_current_time() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}