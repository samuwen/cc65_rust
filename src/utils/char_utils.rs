pub fn is_id_start(c: char) -> bool {
  c.is_ascii_alphabetic() || c == '_'
}

pub fn is_id_char(c: char) -> bool {
  c == '_' || c == '@' || c == '$' || c.is_ascii_alphabetic() || c.is_digit(10)
}

pub fn is_dec_digit(c: char) -> bool {
  c.is_digit(10)
}

pub fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}
