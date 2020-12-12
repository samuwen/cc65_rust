use crate::Segment;

#[derive(Clone)]
pub struct Span {
  id: usize,
  // node: something,
  start: usize,
  end: usize,
  span_type: usize,
  segment: Segment,
}

impl Span {
  pub fn new(segment: &Segment) -> Span {
    let pc = segment.get_pc();
    Span {
      id: usize::MAX,
      start: pc,
      end: pc,
      span_type: 0,
      segment: segment.clone(),
    }
  }
}
