#![feature(const_fn, const_fn_fn_ptr_basics)]
mod data_types;
mod errors;
mod utils;

use data_types::*;
use std::collections::HashMap;
use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};
use utils::*;

/// sets the command line options. doesn't work yet.
fn set_options() {
  let buf = args(); // cmd line arguments originally stored in string buffer
  let mut root = CA65Option::opt_translator(String::new());
  let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();
  let child = CA65Option::opt_datetime(now);
  root.add_option(child);
}

fn main() {
  const GLOBAL_NAMESPACE: String = String::new();
  let mut sym_map: HashMap<String, String> = HashMap::new();
  let mut string_pool = StringPool::new();
  // let seg_context = init_segments();
  // let mut sym_context = init_tables(&GLOBAL_NAMESPACE, ScopeLevel::File, &mut string_pool);
  let command_line = init_command_line(&mut sym_map);
  // let mut global_context = GlobalContext::new(command_line, string_pool, seg_context, sym_context);
  // let line_info_context = init_line_info(&mut global_context);
  // global_context.command_line.parse_args();
  // usage();
  // set_options();
}

pub struct GlobalContext {
  command_line: CommandLine,
  string_pool: StringPool,
  seg_context: SegmentContext,
  sym_context: SymbolContext,
  line_info_context: Option<LineInfoContext>,
}

impl GlobalContext {
  fn new(
    cmd: CommandLine,
    st: StringPool,
    seg: SegmentContext,
    sym: SymbolContext,
  ) -> GlobalContext {
    GlobalContext {
      command_line: cmd,
      string_pool: st,
      seg_context: seg,
      sym_context: sym,
      line_info_context: None,
    }
  }

  fn get_active_segment(&self) -> &Segment {
    self.seg_context.get_active_segment()
  }

  fn get_segment_list(&self) -> &Vec<Segment> {
    self.seg_context.get_segment_list()
  }

  fn add_line_info_context(&mut self, context: LineInfoContext) {
    self.line_info_context = Some(context);
  }
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
