// use std::rc::Rc; // not sure if i need to use RC here
use crate::{ScopeLevel, StringPool};

const ST_NONE: usize = 0x00;
const ST_DEFINED: usize = 0x01;
const ST_CLOSED: usize = 0x02;

pub fn init_tables(name: &String, scope: ScopeLevel, pool: &mut StringPool) -> SymbolContext {
  let name_id = pool.add(name);
  let mut root_table = SymTable::new(name_id);
  root_table.set_flags(ST_DEFINED);
  root_table.set_scope_type(scope);
  SymbolContext::new(root_table)
}

pub struct SymTable {
  next: Box<SymTable>,
  left: Box<SymTable>,
  right: Box<SymTable>,
  parent: Box<SymTable>,
  child: Box<SymTable>,
  label: SymEntry,
  spans: Vec<String>,
  id: usize,
  flags: usize,
  addr_size: u8,
  scope_type: u8,
  level: usize,
  table_slots: usize,
  table_entries: usize,
  name: usize,
  table: SymEntry,
}

impl SymTable {
  fn new(name_id: usize) -> SymTable {
    let slots = SymTable::scope_table_size(0);
    SymTable {
      next: Box::new(SymTable::new_empty()),
      left: Box::new(SymTable::new_empty()),
      right: Box::new(SymTable::new_empty()),
      parent: Box::new(SymTable::new_empty()),
      child: Box::new(SymTable::new_empty()),
      label: SymEntry::new(),
      spans: vec![],
      id: 0,
      flags: 0,
      addr_size: 0,
      scope_type: 0,
      level: 0,
      table_slots: slots,
      table_entries: 0,
      name: name_id,
      table: SymEntry::new(),
    }
  }

  fn new_empty() -> SymTable {
    let slots = SymTable::scope_table_size(0);
    SymTable {
      next: Box::new(SymTable::new_empty()),
      left: Box::new(SymTable::new_empty()),
      right: Box::new(SymTable::new_empty()),
      parent: Box::new(SymTable::new_empty()),
      child: Box::new(SymTable::new_empty()),
      label: SymEntry::new(),
      spans: vec![],
      id: 0,
      flags: 0,
      addr_size: 0,
      scope_type: 0,
      level: 0,
      table_slots: slots,
      table_entries: 0,
      name: 0,
      table: SymEntry::new(),
    }
  }

  fn set_flags(&mut self, flags: usize) {
    self.flags = flags;
  }

  fn set_scope_type(&mut self, scope_type: ScopeLevel) {
    self.scope_type = scope_type.u8_from();
  }

  // WHAT IS THIS FUCKING MADNESS
  fn scope_table_size(num: usize) -> usize {
    match num {
      0 => 213,
      1 => 53,
      _ => 29,
    }
  }
}

struct SymEntry {
  left: Box<SymEntry>,
  right: Box<SymEntry>,
  list: Box<SymEntry>,
  locals: Box<SymEntry>,
  sym_tab: Box<SymTable>, // originally these two were a union.
  sym_ent: Box<SymEntry>,
  def_lines: Vec<String>,
  ref_lines: Vec<String>,
  file_pos: usize, // originally a pointer array
  hll_sym: HLLDbgSym,
  flags: usize,
  debug_sym_id: usize,
  import_id: usize,
  export_id: usize,
  expr: ExprNode,
  expr_refs: Vec<String>,
  export_size: u8,
  addr_size: u8,
  con_des_priorities: Vec<u8>,
  name: usize, // name index in global string pool
}

impl SymEntry {
  fn new() -> SymEntry {
    SymEntry {
      left: Box::new(SymEntry::new()),
      right: Box::new(SymEntry::new()),
      list: Box::new(SymEntry::new()),
      locals: Box::new(SymEntry::new()),
      sym_tab: Box::new(SymTable::new_empty()),
      sym_ent: Box::new(SymEntry::new()),
      def_lines: vec![],
      ref_lines: vec![],
      file_pos: 0,
      hll_sym: HLLDbgSym {},
      flags: 0,
      debug_sym_id: 0,
      import_id: 0,
      export_id: 0,
      expr: ExprNode {},
      expr_refs: vec![],
      export_size: 0,
      addr_size: 0,
      con_des_priorities: vec![],
      name: 0,
    }
  }
}

struct HLLDbgSym;
struct ExprNode;

pub struct SymbolContext {
  tables: Vec<SymTable>,
  current_scope_index: usize,
  root_scope_index: usize,
}

impl SymbolContext {
  fn new(root_table: SymTable) -> SymbolContext {
    SymbolContext {
      tables: vec![root_table],
      current_scope_index: 0,
      root_scope_index: 0,
    }
  }
}
