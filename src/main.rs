#![feature(const_fn, const_fn_fn_ptr_basics)]
mod data_types;
mod errors;
mod init;
mod utils;

use data_types::*;
use init::*;
use std::collections::HashMap;
use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::*;

fn main() {
  let mut sym_map: HashMap<String, String> = HashMap::new();
  let command_line = init_application(&mut sym_map);
}

pub fn exit_success() -> ! {
  std::process::exit(1);
}

pub fn exit_failure() -> ! {
  std::process::exit(1);
}

pub fn success_with_message(msg: &String) -> ! {
  println!("{}", msg);
  exit_success()
}

pub fn fail_with_message(msg: &String) -> ! {
  println!("{}", msg);
  exit_failure()
}
