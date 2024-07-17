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
    let parsed_data: Vec<ParsedData> = Self::parse_line(&line);
    for data in parsed_data {
      match data.data_type {
        ParsedDataType::Command => {
          Self::call_command(data);
        }
        ParsedDataType::Operator => {

        }
        ParsedDataType::Value => {

        }
      }
    }
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
      let data_type: ParsedDataType;

      if COMMANDS.contains(&part) {
        data_type = ParsedDataType::Command;
      }
      else if part.len() == 1 && Interface::OPERATORS.contains(&part.chars().nth(0).unwrap()) {
        data_type = ParsedDataType::Operator;
      }
      else {
        data_type = ParsedDataType::Value;
      }
      
      line_parts_data.push(ParsedData { data_type, value: String::from(part) });
    }

    return line_parts_data;
  }
  // TODO: Můžu přidat nějak aby data co zde byly poslané měli vždy data_type Command?
  fn call_command(data: ParsedData) {
    if data.data_type != ParsedDataType::Command {
      return;
    }
    let command: Commands = Commands::from_str(&data.value);
    Commands::run_command(command);
  }
}