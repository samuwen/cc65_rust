mod data_types;

use data_types::*;
use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};

const PROG_NAME: &str = "CA65";

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
  init_command_line();
  usage();
  set_options();
}
