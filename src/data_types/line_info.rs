use crate::{FilePos, Span};
use std::collections::HashMap;

// pub fn _init_line_info(global_context: &mut OldContext) -> LineInfoContext {
//   let file_pos = FilePos::new();
//   let _line_info_list: Vec<usize> = Vec::with_capacity(200);
//   let mut context = LineInfoContext::new();
//   let li = LineInfo::start_line(file_pos, 0, 0, &mut context, global_context);
//   context
// }

pub enum LineInfoType {
  Asm,
  Ext,
  Macro,
  MacroParameter,
}

impl LineInfoType {
  fn value_of(&self) -> usize {
    match self {
      LineInfoType::Asm => 0,
      LineInfoType::Ext => 1,
      LineInfoType::Macro => 2,
      LineInfoType::MacroParameter => 3,
    }
  }

  fn make_type(info_type: usize, count: usize) -> usize {
    info_type | (count << 2)
  }

  fn get_type(info_type: usize) -> usize {
    info_type & 0x03
  }

  fn get_count(count: usize) -> usize {
    count >> 2
  }
}

#[derive(Clone)]
pub struct LineInfo {
  node: String, // hash table node - so an entry in the hash map
  id: usize,
  ref_count: usize,
  key: LineInfoKey,
  spans: Vec<Span>,
  open_spans: Vec<usize>,
}

impl LineInfo {
  // pub fn start_line(
  //   pos: FilePos,
  //   key_type: usize,
  //   count: usize,
  //   li_context: &mut LineInfoContext,
  //   global_context: &mut OldContext,
  // ) {
  //   let key = LineInfoKey::new(LineInfoType::make_type(key_type, count), pos);
  //   let mut li = match li_context.find_line_info(&key) {
  //     Some(li) => li.clone(),
  //     None => LineInfo::new(key),
  //   };
  //   li.open_span_list(global_context);
  //   li_context.add_line_info(li);
  // }

  fn new(key: LineInfoKey) -> LineInfo {
    LineInfo {
      node: String::new(),
      id: usize::MAX,
      ref_count: 0,
      key,
      spans: vec![],
      open_spans: vec![],
    }
  }

  // fn open_span_list(&mut self, global_context: &mut OldContext) {
  //   let active_segment = global_context.get_active_segment();
  //   let span = Span::new(active_segment);
  //   self.spans.push(span);
  //   let segment_list = global_context.get_segment_list();
  //   for segment in segment_list {
  //     if segment != active_segment {
  //       let span = Span::new(segment);
  //       self.spans.push(span);
  //     }
  //   }
  // }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct LineInfoKey {
  key_type: usize,
  pos: FilePos,
}

impl LineInfoKey {
  fn new(key_type: usize, pos: FilePos) -> LineInfoKey {
    LineInfoKey { key_type, pos }
  }
}

pub struct LineInfoContext {
  table: HashMap<LineInfoKey, LineInfo>,
  active_line: usize,
  line_info_list: Vec<LineInfo>,
}

impl LineInfoContext {
  pub fn new() -> LineInfoContext {
    LineInfoContext {
      table: HashMap::new(),
      active_line: 0,
      line_info_list: vec![],
    }
  }

  fn find_line_info(&self, key: &LineInfoKey) -> Option<&LineInfo> {
    self.table.get(key)
  }

  fn add_line_info(&mut self, line_info: LineInfo) {
    self.line_info_list.push(line_info);
  }
}
