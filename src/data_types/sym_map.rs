use std::collections::HashMap;

pub struct SymMap {
  scope_name: String,
  map: HashMap<String, String>,
  children: Vec<SymMap>,
}

impl SymMap {
  pub fn new_empty() -> SymMap {
    SymMap {
      scope_name: String::new(),
      map: HashMap::new(),
      children: Vec::with_capacity(2),
    }
  }

  pub fn new(name: &String) -> SymMap {
    SymMap {
      scope_name: name.to_owned(),
      map: HashMap::new(),
      children: Vec::with_capacity(2),
    }
  }

  pub fn get_symbol(&self, name: &String) -> Option<&String> {
    self.map.get(name)
  }

  pub fn add_symbol(&mut self, name: &String, val: &String) {
    self.map.insert(name.to_owned(), val.to_owned());
  }
}

pub struct SymbolContext {
  current_scope_name: String,
  root_scope_name: String,
  map: SymMap,
  scope_level: ScopeLevel,
}

impl SymbolContext {
  pub fn new(name: &String) -> SymbolContext {
    SymbolContext {
      current_scope_name: name.to_string(),
      root_scope_name: name.to_string(),
      map: SymMap::new_empty(),
      scope_level: ScopeLevel::File,
    }
  }

  pub fn get_map(&self) -> &SymMap {
    &self.map
  }

  pub fn get_map_mut(&mut self) -> &mut SymMap {
    &mut self.map
  }
}

enum ScopeLevel {
  File,
}
