#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {

      print!("$ ");
      io::stdout().flush().unwrap();

      // Wait for user input
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      // if input.trim() == "exit 0"{
      //   break;
      // }
      let result:&str = input.trim();
      match result {
        "exit 0" => {
          break;
        }
        _ => {
          println!("{}: command not found",result);
        }
      }

    }
}
