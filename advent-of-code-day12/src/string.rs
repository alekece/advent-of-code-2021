pub trait StringExt {
  fn is_uppercase(&self) -> bool;

  fn is_lowercase(&self) -> bool {
    !self.is_uppercase()
  }
}

impl StringExt for str {
  fn is_uppercase(&self) -> bool {
    self.chars().all(|c| c.is_uppercase())
  }
}
