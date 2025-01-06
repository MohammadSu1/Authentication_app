# Authentication_app
## Overview
This project implements a robust authentication system using both secret key authentication and RSA public-key cryptography. It is designed to provide secure communication between clients and servers, ensuring data integrity and confidentiality.

## Features
1. Secret Key Authentication: Utilizes symmetric encryption to authenticate users with a shared secret key, ensuring that only authorized users can access the system.
2. RSA Authentication: Implements asymmetric encryption using RSA keys to securely exchange messages and verify identities without requiring a shared secret.
3. Secure Data Transmission: Protects sensitive information during transmission using industry-standard cryptographic algorithms.
4. Modular Design: The code is organized into modules for easy maintenance and extensibility.

## Getting Started

### RSA-Based JWTs
1. **Check Key Existence**: Use the *key_existance* function to determine whether RSA public and private keys already exist.
2. **Read Existing Keys**: If keys are found, the *read_keys* function will read and load them for use.
3. **Generate New Keys**: If no keys exist, the generate_jwt function will create new RSA keys.
4. **Save Keys**: After generatig or reading keys, you can save them using the *save_keys* function for future use.
5. **Verify Keys**: To validate any public key, use the *verify_jwt* function.

### Secret Key-Based JWTs
If you prefer to use a secret key instead of RSA, you can utilize the following:
1. **Generate JWT**: Use the *generate_jwt* function under the *jwt_secret* module to create a JWT.
2. **Verify JWT**: Use the *verify_jwt* function under the *jwt_secret* module to validate a secret-based JWT.

## Installation
Add this crate to your **Cargo.toml** file:
```Rust
[dependencies]
jwt_modes = "0.1.1"
```
## Usage
Here's an example of how to use this crate using RSA JWT:
### RSA-Based JWTs
```Rust
use jwt_modes::{key_existance, read_keys, generate_jwt, save_keys, verify_jwt};

fn main() {
   let user_id = "user123";
   let email = "user.user@gmail.com";
   let(private_key, public_key) = key_existance();
   let token = generate_jwt(&private_key, user_id, email);
   let claims = verify_jwt(&token, &public_key);
}
```
### Secret Key-Based JWTs
```Rust
use jwt_modes::{generate_jwt, verify_jwt};
fn main(){
   let secret = "User1984@";
   let user_id = "user123";
   let email = "user.user@gmail.com";
   let token = generate_jwt(secret, user_id, email);
   let claims = verify_jwt(&token, secret);
}
```
