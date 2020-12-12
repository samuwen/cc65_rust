#[derive(Eq, PartialEq, Hash, Clone)]
pub struct FilePos {
  line: usize,
  col: usize,
  name: usize,
}

impl FilePos {
  pub fn new() -> FilePos {
    FilePos {
      line: 0,
      col: 0,
      name: 0,
    }
  }
}
