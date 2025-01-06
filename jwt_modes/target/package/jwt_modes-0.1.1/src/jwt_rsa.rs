mod generate_keys;
mod save_keys;
mod read_keys;
mod rsa_private_key_to_pem;
mod rsa_public_key_to_pem;
mod generate_jwt;
mod verify_jwt;
use key_existance::key_existance;

use crate::calculate_current_time;
mod key_existance;

pub fn main(){
    let user_id = "user123";
    let email = "mohammad.sulaiman@exalt.ps";
    let(private_key, public_key) = key_existance();
    match generate_jwt::generate_jwt(&private_key, user_id, email) {
        Ok(token) => {
            println!("Generated JWT: {}", token);

            match verify_jwt::verify_jwt(&token, &public_key) {
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

            match verify_jwt::verify_jwt(&token, &public_key) {
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