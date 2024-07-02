use std::io;
use crate::utils::Commands;

pub struct Interface {

}

impl Interface {
  const SEPARATORS: [char; 1] = [' '];
  const OPERATORS: [char; 2] = ['|', '&'];
  pub fn get_input() -> String {
    println!("Insert your command here:\n");
    loop {
      let mut input: String = String::from("");

      match io::stdin().read_line(&mut input) {
        Ok(_) => {
          return input;
        }
        Err(_) => {
          println!("Unknown syntax!");
          continue;
        }
      }
    }
  }
  fn parse_line(line: &str) {
    for (idx, char) in line.char_indices() {
      if Self::OPERATORS.contains(&char) {
        
      }
    }

    let split = line.split(Self::SEPARATORS);

    for part in split {
      println!("Hehe: {part}");
    }
  }
  pub fn call_command(text: &str) {
    Self::parse_line(text);

    let command = Commands::from_str(text);
    match command {
      Commands::Echo => {

      }
      Commands::None => {

      }
    }
  }
}