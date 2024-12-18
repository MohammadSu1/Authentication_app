use std::io;
use std::io::Write;

use jwt_modes::jwt_secret;
use jwt_modes::jwt_rsa;

fn main() {
   let mut input = String::new();
   println!("****************Authentication Program****************");
   loop {
      print!("(Auth app) > ");
      println!("Choose the authentication mode: ");
      println!("1.Secret key Auth");
      println!("2.RSA Auth");
      println!("3.exit");
      print!("(Auth app) > ");
      io::stdout().flush().unwrap();
      input.clear();
      io::stdin()
          .read_line(&mut input)
          .expect("Failed to read from the user");
      let input = input.trim();
      if input.eq_ignore_ascii_case("1"){
         jwt_secret::main();  
      }
      else if input.eq_ignore_ascii_case("2"){
         jwt_rsa::main();
      }
      else if input.eq_ignore_ascii_case("3") {
         println!("Exiting the Auth app...");
         break;
      }
   }
}
