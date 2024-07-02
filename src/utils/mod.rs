mod echo;

pub enum Commands {
  Echo,
  None
}

impl Commands {
  pub fn from_str(command: &str) -> Self {
    match command {
      "echo" => {
        Commands::Echo
      }
      _ => {
        Commands::None
      }
    }
  }
}