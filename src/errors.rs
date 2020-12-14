use crate::fail_with_message;

pub fn invalid_repeated_option(arg: &String) {
  fail_with_message(&format!("Can't use {} option twice", arg))
}

pub fn invalid_number_option(arg: &String, opt: &String) -> ! {
  fail_with_message(&format!("{} is not a valid number for {}", arg, opt))
}

pub fn invalid_option(arg: &String) -> ! {
  fail_with_message(&format!("Invalid option supplied: {}", arg))
}

pub fn unknown_option(arg: &String) -> ! {
  fail_with_message(&format!("Unknown option supplied: {}", arg))
}

pub fn unknown_definition(arg: &String) -> ! {
  fail_with_message(&format!("Invalid definition: {}", arg))
}
