use crate::init::MemoryModel;

const _ADDR_SIZE_DEFAULT: u8 = 0x00;
const ZP_SIZE: u8 = 0x01;
const ABS_SIZE: u8 = 0x02;
const ADDR_SIZE_FAR: u8 = 0x03;
const _ADDR_SIZE_LONG: u8 = 0x04;
const _ADDR_SIZE_INVALID: u8 = 0xFF;

pub fn init_segments(model: &MemoryModel) -> SegmentContext {
  let size = match model {
    MemoryModel::Near => ABS_SIZE,
    _ => ADDR_SIZE_FAR,
  };
  let start_segment = Segment::new_from_def(SegDef::new("CODE", size), 0);
  let mut context = SegmentContext::new(start_segment);
  let size = match model {
    MemoryModel::Huge => ADDR_SIZE_FAR,
    _ => ABS_SIZE,
  };
  context.add_default_segment("RODATA", size);
  context.add_default_segment("BSS", size);
  context.add_default_segment("DATA", size);
  context.add_default_segment("ZEROPAGE", ZP_SIZE);
  context.add_default_segment("NULL", ABS_SIZE);
  context
}

#[derive(Clone, Eq, PartialEq)]
pub struct Segment {
  root: Fragment,
  last: Fragment,
  frag_count: usize,
  num: usize,
  flags: usize,
  align: usize,
  reloc_mode: isize,
  pc: usize,
  abs_pc: usize,
  def: SegDef,
}

impl Segment {
  pub fn new_from_def(def: SegDef, seg_count: usize) -> Segment {
    Segment {
      root: Fragment {},
      last: Fragment {},
      frag_count: 0,
      num: seg_count,
      flags: 0,
      align: 1,
      reloc_mode: 1,
      pc: 0,
      abs_pc: 0,
      def,
    }
  }

  fn get_addr_size(&self) -> u8 {
    self.def.addr_size
  }

  pub fn get_pc(&self) -> usize {
    self.pc
  }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SegDef {
  name: &'static str,
  addr_size: u8,
}

impl SegDef {
  fn new(name: &'static str, addr_size: u8) -> SegDef {
    SegDef { name, addr_size }
  }

  fn get_addr_size(&self) -> u8 {
    self.addr_size
  }
}

#[derive(Clone, Eq, PartialEq)]
struct Fragment; // this is some kind of tree

pub struct SegmentContext {
  active_segment_index: usize,
  all_segments: Vec<Segment>,
}

impl SegmentContext {
  fn new(start_segment: Segment) -> SegmentContext {
    SegmentContext {
      active_segment_index: 0,
      all_segments: vec![start_segment],
    }
  }

  fn add_segment(&mut self, segment: Segment) {
    self.all_segments.push(segment);
  }

  fn add_default_segment(&mut self, name: &'static str, addr_size: u8) {
    self.add_segment(Segment::new_from_def(
      SegDef::new(name, addr_size),
      self.get_segment_count(),
    ));
  }

  fn get_segment_count(&self) -> usize {
    self.all_segments.len()
  }

  pub fn get_active_segment(&self) -> &Segment {
    &self.all_segments[self.active_segment_index]
  }

  pub fn get_current_seg_addr_size(&self) -> u8 {
    let active = self.get_active_segment();
    active.get_addr_size()
  }

  pub fn get_segment_list(&self) -> &Vec<Segment> {
    &self.all_segments
  }
}
