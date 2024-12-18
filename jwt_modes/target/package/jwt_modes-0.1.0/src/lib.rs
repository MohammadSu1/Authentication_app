use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
pub mod jwt_secret;
pub mod jwt_rsa;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    email: String,
    iat : usize,
}

pub fn calculate_current_time() -> u64 {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("current time")
        .as_secs();
    current_time
}