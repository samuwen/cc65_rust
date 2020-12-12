use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static ENTRY_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_entry_id() -> usize {
  ENTRY_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub struct StringPool {
  entries: Vec<StringPoolEntry>,
  total_size: usize,
  tab: HashMap<String, StringPoolEntry>,
}

impl StringPool {
  pub fn new() -> StringPool {
    let mut pool = StringPool {
      entries: vec![],
      total_size: 0,
      tab: HashMap::new(),
    };
    pool.add(&String::from(""));
    pool
  }

  fn add_string(&mut self, entry: StringPoolEntry) {
    self.entries.push(entry);
  }

  pub fn add(&mut self, string: &String) -> usize {
    match self.tab.get(string) {
      Some(s) => s.get_id(),
      None => {
        let entry = StringPoolEntry::new(string);
        let id = entry.get_id();
        self.entries.push(entry.clone());
        self.tab.insert(string.to_owned(), entry);
        self.total_size += string.len();
        id
      }
    }
  }
}

#[derive(Clone)]
struct StringPoolEntry {
  node: String,
  id: usize,
  buf: String,
}

impl StringPoolEntry {
  fn new(string: &String) -> StringPoolEntry {
    StringPoolEntry {
      node: string.to_owned(),
      id: next_entry_id(),
      buf: string.to_owned(),
    }
  }

  fn get_buf(&self) -> &String {
    &self.buf
  }

  fn get_id(&self) -> usize {
    self.id
  }
}
