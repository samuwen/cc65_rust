use crate::{fail_with_message, success_with_message};
use crate::{is_dec_digit, is_hex_digit, is_id_char, is_id_start};
use std::collections::HashMap;
use std::env::{args, vars};

const MIN_LIST_BYTES: u8 = 4;
const MIN_PAGE_LEN: u8 = 32;
const MAX_PAGE_LEN: u8 = 127;

const VERSION: &str = "1.0.0";

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

/// Print usage information and exit
fn usage(program_name: String) -> String {
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
  let start = format!("Usage: {} [options] file\n{}", program_name, rest);
  start
}

fn opt_auto_import(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.auto_import = true;
  cmd
}

fn opt_bin_include_dir(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.bin_search_paths.push(arg);
  cmd
}

fn opt_cpu(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.cpu = CpuType::from_string(arg);
  cmd
}

fn opt_create_dep(opt: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  if !cmd.dep_name.is_empty() {
    invalid_repeated_option(&opt);
  }
  cmd.dep_name = arg;
  cmd
}

fn opt_create_full_dep(opt: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  if !cmd.full_dep_name.is_empty() {
    invalid_repeated_option(&opt);
  }
  cmd.full_dep_name = arg;
  cmd
}

fn opt_debug(_: String, _: String, mut command_line: CommandLine) -> CommandLine {
  command_line.debug_mode += 1;
  command_line
}

fn opt_debug_info(_: String, _: String, mut command_line: CommandLine) -> CommandLine {
  command_line.debug_info = true;
  command_line
}

fn opt_feature(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.feature = Feature::from_string(arg);
  cmd
}

fn opt_help(_: String, _: String, cmd: CommandLine) -> CommandLine {
  success_with_message(&usage(cmd.program_name))
}

fn opt_ignore_case(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.ignore_case = true;
  cmd
}

fn opt_include_dir(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.inc_search_paths.push(arg);
  cmd
}

fn opt_large_alignment(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.large_alignment = true;
  cmd
}

fn opt_list_bytes(opt: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  let num = u8::from_str_radix(&arg, 10);
  let num = match num {
    Ok(n) => n,
    Err(_) => invalid_number_option(&arg, &opt),
  };
  if num < MIN_LIST_BYTES {
    invalid_number_option(&arg, &opt);
  }
  cmd.list_bytes = num;
  cmd
}

fn opt_listing(opt: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  if !cmd.listing.is_empty() {
    invalid_repeated_option(&opt);
  }
  cmd.listing = arg;
  cmd
}

fn opt_memory_model(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.memory_model = MemoryModel::from_string(arg);
  cmd
}

fn opt_page_length(opt: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  let num = u8::from_str_radix(&arg, 10);
  let num = match num {
    Ok(n) => n,
    Err(_) => invalid_number_option(&arg, &opt),
  };
  if num < MIN_PAGE_LEN || num > MAX_PAGE_LEN {
    invalid_number_option(&arg, &opt);
  }
  cmd.page_length = num;
  cmd
}

fn opt_relax_checks(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.relax_checks = true;
  cmd
}

fn opt_smart(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.smart = true;
  cmd
}

fn opt_target(_: String, arg: String, mut cmd: CommandLine) -> CommandLine {
  cmd.target = TargetSystem::from_string(arg);
  cmd
}

fn opt_verbose(_: String, _: String, mut cmd: CommandLine) -> CommandLine {
  cmd.verbose += 1;
  cmd
}

fn opt_version(_: String, _: String, cmd: CommandLine) -> CommandLine {
  success_with_message(&format!("{} V{}", cmd.program_name, VERSION))
}

pub fn init_command_line(sym_map: &mut HashMap<String, String>) -> CommandLine {
  let mut args = args();
  let program_name = args.next().unwrap();
  let program_name = program_name.split("/").last().unwrap();
  let mut command_line = CommandLine::new(program_name.to_owned());
  command_line = parse_args(args.collect(), command_line, sym_map);
  if command_line.in_file.is_empty() {
    fail_with_message(&"No input files".to_string())
  }
  finish_include_paths(&mut command_line);
  set_default_cpu(&mut command_line);
  command_line
}

fn parse_args(
  mut args: Vec<String>,
  command_line: CommandLine,
  sym_map: &mut HashMap<String, String>,
) -> CommandLine {
  let mut next = args.get(0);
  let mut cmd = command_line;
  while next.is_some() {
    let arg = args.remove(0);
    let first_char = arg.chars().next().expect("empty arg");
    match first_char == '-' {
      true => {
        let second_char = arg.chars().skip(1).next().expect("empty arg");
        let mut second_arg = String::from("");
        match second_char == '-' {
          true => {
            let found = OPT_TAB.iter().find(|opt| opt.option == arg);
            match found {
              Some(opt) => {
                if opt.arg_count > 0 {
                  test_for_arg(&arg, args.get(0));
                  second_arg = args.remove(0);
                }
                let f = opt.func;
                cmd = f(arg, second_arg, cmd);
              }
              None => {
                invalid_option(&arg);
              }
            }
          }
          false => {
            cmd = match second_char {
              'd' => opt_debug(arg, second_arg, cmd),
              'g' => opt_debug_info(arg, second_arg, cmd),
              'h' => opt_help(arg, second_arg, cmd),
              'i' => opt_ignore_case(arg, second_arg, cmd),
              'l' => {
                test_for_arg(&arg, args.get(0));
                second_arg = args.remove(0);
                opt_listing(arg, second_arg, cmd)
              }
              'm' => match arg.chars().skip(2).next().expect("empty arg") == 'm' {
                true => {
                  test_for_arg(&arg, args.get(0));
                  second_arg = args.remove(0);
                  opt_memory_model(arg, second_arg, cmd)
                }
                false => invalid_option(&arg),
              },
              'o' => {
                test_for_arg(&arg, args.get(0));
                second_arg = args.remove(0);
                cmd.out_file = second_arg;
                cmd
              }
              's' => opt_smart(arg, second_arg, cmd),
              't' => {
                test_for_arg(&arg, args.get(0));
                second_arg = args.remove(0);
                opt_target(arg, second_arg, cmd)
              }
              'v' => opt_verbose(arg, second_arg, cmd),
              'D' => {
                test_for_arg(&arg, args.get(0));
                second_arg = args.remove(0);
                let mut name = String::new();
                let mut value = String::new();
                let mut chars = second_arg.chars();
                let fca = chars.next();
                let fca = test_for_char(fca);
                match !is_id_start(fca) {
                  true => {
                    name.push(fca);
                  }
                  false => fail_with_message(&format!("Invalid definition: {}", second_arg)),
                }
                let mut next = chars.next();
                while next.is_some() {
                  let c = next.unwrap();
                  if !is_id_char(c) {
                    break;
                  }
                  name.push(c);
                  next = chars.next();
                }
                let c = next.unwrap();
                if c == '=' {
                  next = chars.next();
                  test_for_char(next);
                  let c = next.unwrap();
                  let f = match c == '$' {
                    true => is_hex_digit,
                    false => is_dec_digit,
                  };
                  next = chars.next();
                  while next.is_some() {
                    let c = next.unwrap();
                    if !f(c) {
                      fail_with_message(&format!("Invalid definition: {}", second_arg))
                    }
                    value.push(c);
                    next = chars.next();
                  }
                }
                if sym_map.get(&name).is_some() {
                  fail_with_message(&format!("Cannot define symbol twice: {}", name))
                }
                sym_map.insert(name, value);
                cmd
              }
              'I' => {
                test_for_arg(&arg, args.get(0));
                second_arg = args.remove(0);
                opt_include_dir(arg, second_arg, cmd)
              }
              'U' => opt_auto_import(arg, second_arg, cmd),
              'V' => opt_version(arg, second_arg, cmd),
              'W' => {
                // sets the logging level
                // we should do this elsewhere
                todo!();
              }
              _ => unknown_option(&arg),
            };
          }
        }
      }
      false => match cmd.in_file.is_empty() {
        true => {
          cmd.in_file = arg;
        }
        false => fail_with_message(&format!("Don't know what to do with {}", arg)),
      },
    }
    next = args.get(0);
  }
  cmd
}

fn finish_include_paths(cmd: &mut CommandLine) {
  let vars = vars();
  for (k, v) in vars {
    match k.as_ref() {
      "CA65_INC" => cmd.inc_search_paths.push(v),
      "CA65_HOME" => {
        let v = format!("{}/asminc", v);
        cmd.inc_search_paths.push(v);
      }
      _ => panic!(),
    }
  }
}

fn set_default_cpu(cmd: &mut CommandLine) {
  match cmd.cpu {
    CpuType::Unknown => match cmd.target {
      TargetSystem::Unknown => cmd.cpu = CpuType::C6502,
      _ => (),
    },
    _ => (),
  }
}

fn test_for_arg(arg: &String, test: Option<&String>) {
  if test.is_none() {
    fail_with_message(&format!(
      "Unexpected end of input. Need a second argument for {}",
      arg
    ));
  }
}

fn test_for_char(c: Option<char>) -> char {
  match c {
    Some(c) => c,
    None => fail_with_message(&"Unexpected end of input".to_string()),
  }
}

fn invalid_repeated_option(arg: &String) {
  fail_with_message(&format!("Can't use {} option twice", arg))
}

fn invalid_number_option(arg: &String, opt: &String) -> ! {
  fail_with_message(&format!("{} is not a valid number for {}", arg, opt))
}

fn invalid_option(arg: &String) -> ! {
  fail_with_message(&format!("Invalid option supplied: {}", arg))
}

fn unknown_option(arg: &String) -> ! {
  fail_with_message(&format!("Unknown option supplied: {}", arg))
}

fn unknown_definition(arg: &String) -> ! {
  fail_with_message(&format!("Invalid definition: {}", arg))
}

pub struct CommandLine {
  program_name: String,
  auto_import: bool,
  cpu: CpuType,
  dep_name: String,
  full_dep_name: String,
  debug_mode: u8,
  debug_info: bool,
  feature: Feature,
  ignore_case: bool,
  large_alignment: bool,
  list_bytes: u8,
  listing: String,
  memory_model: MemoryModel,
  page_length: u8,
  relax_checks: bool,
  smart: bool,
  target: TargetSystem,
  verbose: u8,
  inc_search_paths: Vec<String>,
  bin_search_paths: Vec<String>,
  in_file: String,
  out_file: String,
}

impl CommandLine {
  pub fn new(program_name: String) -> CommandLine {
    CommandLine {
      program_name,
      auto_import: false,
      cpu: CpuType::Unknown,
      dep_name: String::new(),
      full_dep_name: String::new(),
      debug_mode: 0,
      debug_info: false,
      feature: Feature::Unknown,
      ignore_case: false,
      large_alignment: false,
      list_bytes: MIN_LIST_BYTES,
      listing: String::new(),
      memory_model: MemoryModel::Unknown,
      page_length: 0,
      relax_checks: false,
      smart: false,
      target: TargetSystem::Unknown,
      verbose: 0,
      inc_search_paths: vec![],
      bin_search_paths: vec![],
      in_file: String::new(),
      out_file: String::new(),
    }
  }
}

pub struct LongOpt {
  option: &'static str,
  arg_count: usize,
  func: fn(String, String, CommandLine) -> CommandLine,
}

impl LongOpt {
  pub const fn new(
    option: &'static str,
    count: usize,
    f: fn(String, String, CommandLine) -> CommandLine,
  ) -> LongOpt {
    LongOpt {
      option: option,
      arg_count: count,
      func: f,
    }
  }
}

enum CpuType {
  Unknown, /* Not specified or invalid target */
  Nope,    /* No CPU - for assembler */
  C6502,
  C6502X, /* "Extended", that is: with illegal opcodes */
  C65SC02,
  C65C02,
  C65816,
  CSWEET16,
  CHUC6280, /* Used in PC engine */
  CM740,    /* Mitsubishi 740 series MCUs */
  C4510,    /* CPU of C65 */
}

impl CpuType {
  fn from_string(s: String) -> CpuType {
    match s.to_ascii_lowercase().as_ref() {
      "None" => CpuType::Nope,
      "6502" => CpuType::C6502,
      "6502x" => CpuType::C6502X,
      "65sc02" => CpuType::C65SC02,
      "65c02" => CpuType::C65C02,
      "65816" => CpuType::C65816,
      "sweet16" => CpuType::CSWEET16,
      "huc6280" => CpuType::CHUC6280,
      "m640" => CpuType::CM740,
      "4510" => CpuType::C4510,
      _ => unknown_option(&s),
    }
  }
}

enum MemoryModel {
  Unknown,
  Near, // Code: near, data: near
  Far,  // Code: far, data: near
  Huge, // Code: far, data: far
}

impl MemoryModel {
  fn from_string(s: String) -> MemoryModel {
    match s.to_ascii_lowercase().as_ref() {
      "near" => MemoryModel::Near,
      "far" => MemoryModel::Far,
      "huge" => MemoryModel::Huge,
      _ => unknown_option(&s),
    }
  }
}

enum TargetSystem {
  Unknown,
  Nope,
  Module,
  Atari,
  Atari2600,
  Atari5200,
  Atarixl,
  Vic20,
  C16,
  C64,
  C128,
  Plus4,
  Cbm510,
  Cbm610,
  Osic1p,
  Pet,
  Bbc,
  Apple2,
  Apple2enh,
  GeosCbm,
  Creativision,
  GeosApple,
  Lunix,
  Atmos,
  Telestrat,
  Nes,
  Supervision,
  Lynx,
  Sim6502,
  Sim65c02,
  Pcengine,
  Gamate,
  C65,
  Cx16,
}

impl TargetSystem {
  fn from_string(s: String) -> TargetSystem {
    match s.to_ascii_lowercase().as_ref() {
      "unknown" => TargetSystem::Unknown,
      "nope" => TargetSystem::Nope,
      "module" => TargetSystem::Module,
      "atari" => TargetSystem::Atari,
      "atari2600" => TargetSystem::Atari2600,
      "atari5200" => TargetSystem::Atari5200,
      "atarixl" => TargetSystem::Atarixl,
      "vic20" => TargetSystem::Vic20,
      "c16" => TargetSystem::C16,
      "c64" => TargetSystem::C64,
      "c128" => TargetSystem::C128,
      "plus4" => TargetSystem::Plus4,
      "cbm510" => TargetSystem::Cbm510,
      "cbm610" => TargetSystem::Cbm610,
      "osic1p" => TargetSystem::Osic1p,
      "pet" => TargetSystem::Pet,
      "bbc" => TargetSystem::Bbc,
      "apple2" => TargetSystem::Apple2,
      "apple2enh" => TargetSystem::Apple2enh,
      "geos_cbm" => TargetSystem::GeosCbm,
      "creativision" => TargetSystem::Creativision,
      "geos_apple" => TargetSystem::GeosApple,
      "lunix" => TargetSystem::Lunix,
      "atmos" => TargetSystem::Atmos,
      "telestrat" => TargetSystem::Telestrat,
      "nes" => TargetSystem::Nes,
      "supervision" => TargetSystem::Supervision,
      "lynx" => TargetSystem::Lynx,
      "sim6502" => TargetSystem::Sim6502,
      "sim65c02" => TargetSystem::Sim65c02,
      "pcengine" => TargetSystem::Pcengine,
      "gamate" => TargetSystem::Gamate,
      "c65" => TargetSystem::C65,
      "cx16" => TargetSystem::Cx16,
      _ => unknown_option(&s),
    }
  }
}

enum Feature {
  Unknown,
  DollarIsPc,
  LabelsWithoutColons,
  LooseStringTerm,
  LooseCharTerm,
  AtInIdentifiers,
  DollarInIdentifiers,
  LeadingDotInIdentifiers,
  OrgPerSeg,
  PcAssignment,
  MissingCharTerm,
  UbiquitousIdents,
  CComments,
  ForceRange,
  UnderlineInNumbers,
  Addrsize,
  BracketAsIndirect,
  StringEscapes,
}

impl Feature {
  fn from_string(s: String) -> Feature {
    match s.to_ascii_lowercase().as_ref() {
      "Unknown" => Feature::Unknown,
      "DollarIsPc" => Feature::DollarIsPc,
      "LabelsWithoutColons" => Feature::LabelsWithoutColons,
      "LooseStringTerm" => Feature::LooseStringTerm,
      "LooseCharTerm" => Feature::LooseCharTerm,
      "AtInIdentifiers" => Feature::AtInIdentifiers,
      "DollarInIdentifiers" => Feature::DollarInIdentifiers,
      "LeadingDotInIdentifiers" => Feature::LeadingDotInIdentifiers,
      "OrgPerSeg" => Feature::OrgPerSeg,
      "PcAssignment" => Feature::PcAssignment,
      "MissingCharTerm" => Feature::MissingCharTerm,
      "UbiquitousIdents" => Feature::UbiquitousIdents,
      "CComments" => Feature::CComments,
      "ForceRange" => Feature::ForceRange,
      "UnderlineInNumbers" => Feature::UnderlineInNumbers,
      "Addrsize" => Feature::Addrsize,
      "BracketAsIndirect" => Feature::BracketAsIndirect,
      "StringEscapes" => Feature::StringEscapes,
      _ => unknown_option(&s),
    }
  }
}
