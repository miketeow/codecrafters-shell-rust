#[allow(unused_imports)]
use std::io::{self, Write};

struct Input {
  command: String,
  args: Vec<String>
}

fn main() {
    // Uncomment this block to pass the first stage
    loop {

      print!("$ ");
      io::stdout().flush().unwrap();

      // Wait for user input
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let formatted_input: Input = parse_input(input);
      match formatted_input.command.as_str() {
        "exit" => {
          break;
        }
        "echo" => {
          if formatted_input.args.is_empty() {
            println!("{}: command not found",formatted_input.command);
          } else {
            println!("{}", formatted_input.args.join(" "));
          }
        }
        _ => {
          println!("{}: command not found",formatted_input.command);
        }
      }
    }
}

fn parse_input(input: String) -> Input {
  let input:&str = input.trim();

  let command= if let Some(first_space) = input.find(" "){
    input[..first_space].to_string()
  } else {
    input.to_string()
  };

  let mut args: Vec<String> = Vec::new();

  if let Some(first_space) = input.find(" "){
    let mut rest = &input[first_space + 1 ..];
    while !rest.is_empty() {
      if let Some(next_space) = rest.find(" "){
        args.push(rest[..next_space].to_string());
        rest = &rest[next_space + 1..]
      } else {
        args.push(rest.to_string());
        rest = ""
      }
    }
  }

  Input {command, args}
}
