use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct CommandLine {
  vec: Vec<String>,
}

impl CommandLine {
  pub fn new() -> CommandLine {
    let vec = vec![];
    CommandLine { vec }
  }

  pub fn add_arg(&mut self, arg: String) {
    self.vec.push(arg);
  }

  pub fn expand_file(&mut self, arg: String) {
    let error = format!("File not found: {}", arg);
    let file = File::open(arg).expect(&error);
    let f = BufReader::new(file);
    let is_ws = |c: &char| c.is_ascii_whitespace();
    for line in f.lines() {
      let line_string: String = line.unwrap().chars().rev().skip_while(is_ws).collect();
      let line_string: String = line_string.chars().skip_while(is_ws).collect();
      if line_string.len() == 0 {
        continue;
      }
      self.add_arg(line_string);
    }
  }
}

pub struct LongOpt {
  option: &'static str,
  arg_count: usize,
  func: fn(String, String),
}

impl LongOpt {
  pub const fn new(option: &'static str, count: usize, f: fn(String, String)) -> LongOpt {
    LongOpt {
      option: option,
      arg_count: count,
      func: f,
    }
  }
}
