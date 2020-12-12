#![feature(const_fn, const_fn_fn_ptr_basics)]
mod data_types;

use data_types::*;
use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};

const PROG_NAME: &str = "CA65";

static OPT_TAB: [LongOpt; 21] = [
  LongOpt::new("--auto-import", 0, opt_auto_import),
  LongOpt::new("--bin-include-dir", 1, opt_bin_include_dir),
  LongOpt::new("--cpu", 1, opt_cpu),
  LongOpt::new("--create-dep", 1, opt_create_dep),
  LongOpt::new("--create-full-dep", 1, opt_create_full_dep),
  LongOpt::new("--debug", 0, opt_debug),
  LongOpt::new("--debug-info", 0, opt_debug_info),
  LongOpt::new("--feature", 1, opt_feature),
  LongOpt::new("--help", 0, opt_help),
  LongOpt::new("--ignore-case", 0, opt_ignore_case),
  LongOpt::new("--include-dir", 1, opt_include_dir),
  LongOpt::new("--large-alignemnt", 0, opt_large_alignment),
  LongOpt::new("--list-bytes", 1, opt_list_bytes),
  LongOpt::new("--listing", 1, opt_listing),
  LongOpt::new("--memory-model", 1, opt_memory_model),
  LongOpt::new("--pagelength", 1, opt_page_length),
  LongOpt::new("--relax-checks", 0, opt_relax_checks),
  LongOpt::new("--smart", 0, opt_smart),
  LongOpt::new("--target", 1, opt_target),
  LongOpt::new("--verbose", 0, opt_verbose),
  LongOpt::new("--version", 0, opt_version),
];

fn opt_auto_import(_: String, _: String) {
  // sets a global flag
  todo!();
}

fn opt_bin_include_dir(_: String, _: String) {
  // adds a listing in a search path
  todo!();
}

fn opt_cpu(_: String, _: String) {
  // sets the target CPU
  todo!();
}

fn opt_create_dep(_: String, _: String) {
  // handles creating a make file dependency
  todo!();
}

fn opt_create_full_dep(_: String, _: String) {
  // handles creating a full make file dependency ???
  todo!();
}

fn opt_debug(_: String, _: String) {
  // toggles on the debug flag
  todo!();
}

fn opt_debug_info(_: String, _: String) {
  // toggles on the debug flag
  todo!();
}

fn opt_feature(_: String, _: String) {
  // sets an emulation feature
  todo!();
}

fn opt_help(_: String, _: String) {
  // sets help flag
  todo!();
}

fn opt_ignore_case(_: String, _: String) {
  // sets to ignore casing
  todo!();
}

fn opt_include_dir(_: String, _: String) {
  // adds an include search path
  todo!();
}

fn opt_large_alignment(_: String, _: String) {
  // don't warn about large alignments (???)
  todo!();
}

fn opt_list_bytes(_: String, _: String) {
  // set max num bytes per list line
  todo!();
}

fn opt_listing(_: String, _: String) {
  // create a listing file
  todo!();
}

fn opt_memory_model(_: String, _: String) {
  // create the memory model
  todo!();
}

fn opt_page_length(_: String, _: String) {
  todo!();
}

fn opt_relax_checks(_: String, _: String) {
  todo!();
}

fn opt_smart(_: String, _: String) {
  todo!();
}

fn opt_target(_: String, _: String) {
  todo!();
}

fn opt_verbose(_: String, _: String) {
  todo!();
}

fn opt_version(_: String, _: String) {
  todo!();
}

/// Print usage information and exit
fn usage() {
  let rest = concat!(
    "Short options:\n",
    "  -D name[=value]\t\tDefine a symbol\n",
    "  -I dir\t\t\tSet an include directory search path\n",
    "  -U\t\t\t\tMark unresolved symbols as import\n",
    "  -V\t\t\t\tPrint the assembler version\n",
    "  -W n\t\t\t\tSet warning level n\n",
    "  -d\t\t\t\tDebug mode\n",
    "  -g\t\t\t\tAdd debug info to object file\n",
    "  -h\t\t\t\tHelp (this text)\n",
    "  -i\t\t\t\tIgnore case of symbols\n",
    "  -l name\t\t\tCreate a listing file if assembly was ok\n",
    "  -mm model\t\t\tSet the memory model\n",
    "  -o name\t\t\tName the output file\n",
    "  -s\t\t\t\tEnable smart mode\n",
    "  -t sys\t\t\tSet the target system\n",
    "  -v\t\t\t\tIncrease verbosity\n",
    "\n",
    "Long options:\n",
    "  --auto-import\t\t\tMark unresolved symbols as import\n",
    "  --bin-include-dir dir\t\tSet a search path for binary includes\n",
    "  --cpu type\t\t\tSet cpu type\n",
    "  --create-dep name\t\tCreate a make dependency file\n",
    "  --create-full-dep name\tCreate a full make dependency file\n",
    "  --debug\t\t\tDebug mode\n",
    "  --debug-info\t\t\tAdd debug info to object file\n",
    "  --feature name\t\tSet an emulation feature\n",
    "  --help\t\t\tHelp (this text)\n",
    "  --ignore-case\t\t\tIgnore case of symbols\n",
    "  --include-dir dir\t\tSet an include directory search path\n",
    "  --large-alignment\t\tDon't warn about large alignments\n",
    "  --listing name\t\tCreate a listing file if assembly was ok\n",
    "  --list-bytes n\t\tMaximum number of bytes per listing line\n",
    "  --memory-model model\t\tSet the memory model\n",
    "  --pagelength n\t\tSet the page length for the listing\n",
    "  --relax-checks\t\tRelax some checks (see docs)\n",
    "  --smart\t\t\tEnable smart mode\n",
    "  --target sys\t\t\tSet the target system\n",
    "  --verbose\t\t\tIncrease verbosity\n",
    "  --version\t\t\tPrint the assembler version\n",
  );
  let start = format!("Usage: {} [options] file\n{}", PROG_NAME, rest);
  println!("{}", start);
}

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
  let command_line = init_command_line();
  let mut string_pool = StringPool::new();
  init_include_paths();
  let seg_context = init_segments();
  let sym_context = init_tables(&GLOBAL_NAMESPACE, ScopeLevel::File, &mut string_pool);
  let mut global_context = GlobalContext::new(command_line, string_pool, seg_context, sym_context);
  let line_info_context = init_line_info(&mut global_context);
  usage();
  set_options();
}

fn init_command_line() -> CommandLine {
  let mut command_line = CommandLine::new();
  let mut args = args();
  let mut next = args.next();
  while next.is_some() {
    let arg = next.unwrap();
    match arg.chars().next().unwrap() {
      '@' => command_line.expand_file(arg),
      _ => command_line.add_arg(arg),
    }
    next = args.next();
  }
  command_line
}

fn init_include_paths() {
  // create 2 search paths. search paths are just collections of something.
  println!("Nothin here boss");
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
