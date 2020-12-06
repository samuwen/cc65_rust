use std::env::args;

pub struct CommandLine {
  args: Vec<String>,
  count: u8,
  size: u8,
}

impl CommandLine {
  pub fn new() -> CommandLine {
    let args: Vec<String> = args().collect();
    let count = args.len() as u8;
    let size = args.len() as u8;
    CommandLine { args, count, size }
  }
}
