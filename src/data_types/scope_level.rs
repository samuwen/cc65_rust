pub enum ScopeLevel {
  Global,
  File,
  Scope,   // .SCOPE / .PROC
  HasData, // last scope that has data (originally equal to scope scope)
  Struct,  // .STRUCT / .UNION
  Enum,
  Undef,
}

impl ScopeLevel {
  pub fn u8_from(&self) -> u8 {
    match self {
      ScopeLevel::Global => 0,
      ScopeLevel::File => 1,
      ScopeLevel::Scope => 2,
      ScopeLevel::HasData => 2,
      ScopeLevel::Struct => 3,
      ScopeLevel::Enum => 4,
      ScopeLevel::Undef => 0xFF,
    }
  }
}
