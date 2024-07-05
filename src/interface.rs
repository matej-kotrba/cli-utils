use std::io;
use crate::utils::{Commands, COMMANDS};

#[derive(PartialEq)]
enum ParsedDataType {
  Command,
  Operator,
  Value
}

struct ParsedData {
  data_type: ParsedDataType,
  value: String
}

pub struct Interface {

}

impl Interface {
  const SEPARATORS: [char; 1] = [' '];
  const OPERATORS: [char; 2] = ['|', '&'];
  pub fn new() {
    let line =  Self::get_input();
    let parsedData: Vec<ParsedData> = Self::parse_line(&line);
  }
  fn get_input() -> String {
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
  fn parse_line(line: &str) -> Vec<ParsedData> {
    // for (idx, char) in line.char_indices() {
    //   if Self::OPERATORS.contains(&char) {
        
    //   }
    // }

    let split = line.split(Self::SEPARATORS);
    let mut line_parts_data: Vec<ParsedData> = Vec::new();

    for part in split {
      if (COMMANDS.contains(&part)) {
        line_parts_data.push(ParsedData { data_type: ParsedDataType::Command, value: String::from(part) });
        continue;
      }
      else if (part.len() == 1 && Interface::OPERATORS.contains(&part.chars().nth(0).unwrap())) {
        line_parts_data.push(ParsedData { data_type: ParsedDataType::Operator, value: String::from(part) });
        continue;
      }
      else {
        line_parts_data.push(ParsedData { data_type: ParsedDataType::Value, value: String::from(part) });
        continue;
      }
    }

    return line_parts_data;
  }
  fn call_command(data: ParsedData) {
    if data.data_type != ParsedDataType::Command {
      return;
    }
    let command = Commands::from_str(&data.value);
    match command {
      Commands::Echo => {
        println!("ECHOOOO")
      }
      Commands::None => {

      }
    }
  }
}