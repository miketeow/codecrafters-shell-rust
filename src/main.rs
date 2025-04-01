#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::var, path::{Path, PathBuf}};

struct Input {
  command: String,
  args: Vec<String>
}

const BUILTINS: &[&str] = &["exit","echo","type","pwd","cd"];

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
        "type" => {
          if let Some(arg) = formatted_input.args.get(0){
            if BUILTINS.contains(&arg.as_str()){
              println!("{} is a shell builtin",arg);
            } else {
              let result_path: Option<PathBuf> = path_finder(arg);
              if let Some(path) = result_path {
                println!("{} is {}",arg, path.display())
              } else {
                println!("{}: not found",arg);
              }
            }
          }
        }
        _ => {
          println!("{}: command not found",formatted_input.command);
        }
      }
    }
}

fn path_finder(query: &String) -> Option<PathBuf>{
  let path: String = var("PATH").unwrap_or("".to_string());
  let dirs: Vec<&str> = path.split(":").collect();
  let mut found_path = None;
  for dir in &dirs{
    let new_path = Path::new(dir).join(query);
    if new_path.exists() {
      found_path = Some(new_path);
      break;
    }
  }
  found_path
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
