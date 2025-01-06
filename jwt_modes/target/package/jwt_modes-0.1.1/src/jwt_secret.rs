mod generate_jwt;
mod verify_jwt;
use crate::calculate_current_time;

pub fn main() {
    let secret = "Mo145";
    let user_id = "user123";
    let email = "mohammad.sulaiman@exalt.ps";
    match generate_jwt::generate_jwt(secret, user_id, email) {
        Ok(token) => {
            println!("Generated JWT: {}", token);

            match verify_jwt::verify_jwt(&token, secret) {
                Ok(claims) => {
                    println!("Token is valid. Claims: {:?}", claims);
                },
                Err(err) => {
                    println!("Token verification failed: {}", err);
                }
            }

            println!("Waiting for token expiration...");
            std::thread::sleep(std::time::Duration::from_secs(90));

            let current_time_after_wait = calculate_current_time();
            println!("Current time after wait: {}", current_time_after_wait);

            match verify_jwt::verify_jwt(&token, secret) {
                Ok(claims) => {
                    println!("Token is still valid after expiration. Claims: {:?}", claims);
                    println!("Expiration time from Claims: {}", claims.exp);
                },
                Err(err) => {
                    println!("Token verification failed, because it was expired: {}", err);
                }
            }
        },
        Err(err) => {
            println!("Failed to generate token: {}", err);
        }
    }
}