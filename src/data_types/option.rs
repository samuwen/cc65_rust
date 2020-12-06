const OPT_ARGMASK: u8 = 0xC0; // Mask for argument
const OPT_ARGSTR: u8 = 0x00; // String argument
const OPT_ARGNUM: u8 = 0x40; // Numerical argument

const OPT_COMMENT: u8 = OPT_ARGSTR + 0; // Generic comment
const OPT_AUTHOR: u8 = OPT_ARGSTR + 1; // Author specification
const OPT_TRANSLATOR: u8 = OPT_ARGSTR + 2; // Translator specification
const OPT_COMPILER: u8 = OPT_ARGSTR + 3; // Compiler specification
const OPT_OS: u8 = OPT_ARGSTR + 4; // Operating system specification

const OPT_DATETIME: u8 = OPT_ARGNUM + 0; // Date/time of translation

pub struct CA65Option {
  opt_type: u8,
  val: u128,
  next: Box<CA65Option>,
}

impl CA65Option {
  pub fn new(opt_type: u8, val: u128) -> CA65Option {
    CA65Option {
      opt_type,
      val,
      next: Box::new(CA65Option::new_empty()),
    }
  }

  pub fn opt_translator(translator: String) -> CA65Option {
    CA65Option::new(OPT_TRANSLATOR, 0)
  }

  pub fn opt_datetime(datetime: u128) -> CA65Option {
    CA65Option::new(OPT_DATETIME, datetime)
  }

  pub fn add_option(&mut self, opt: Self) {
    self.next = Box::new(opt);
  }

  fn new_empty() -> CA65Option {
    CA65Option {
      opt_type: 0,
      val: 0,
      next: Box::new(CA65Option::new_empty()),
    }
  }
}
