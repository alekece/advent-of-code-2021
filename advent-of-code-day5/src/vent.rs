use parse_display::FromStr;

#[derive(Debug, Copy, Clone, FromStr, PartialEq, Eq, Hash)]
#[display("{x},{y}")]
pub struct Point {
  pub x: isize,
  pub y: isize,
}

impl Point {
  pub fn get_directions(&self, other: &Point) -> (isize, isize) {
    let dir_x = if self.x < other.x { 1 } else { -1 };
    let dir_y = if self.y < other.y { 1 } else { -1 };

    (dir_x, dir_y)
  }
}

#[derive(FromStr)]
#[display("{start_pos} -> {end_pos}")]
pub struct Vent {
  start_pos: Point,
  end_pos: Point,
}

impl Vent {
  pub fn is_ortholinear(&self) -> bool {
    self.start_pos.x == self.end_pos.x || self.start_pos.y == self.end_pos.y
  }

  pub fn get_path(&self) -> Vec<Point> {
    let (dir_x, dir_y) = self.start_pos.get_directions(&self.end_pos);
    let (steps, ref_x, ref_y) = if self.start_pos.x == self.end_pos.x {
      (
        0..=(self.start_pos.y - self.end_pos.y).abs(),
        None,
        Some(&self.start_pos.y),
      )
    } else if self.start_pos.y == self.end_pos.y {
      (
        0..=(self.start_pos.x - self.end_pos.x).abs(),
        Some(&self.start_pos.x),
        None,
      )
    } else {
      (
        0..=(self.start_pos.x - self.end_pos.x).abs(),
        Some(&self.start_pos.x),
        Some(&self.start_pos.y),
      )
    };

    steps
      .into_iter()
      .map(|step| Point {
        x: ref_x
          .map(|x| x + (step * dir_x))
          .unwrap_or(self.start_pos.x),
        y: ref_y
          .map(|y| y + (step * dir_y))
          .unwrap_or(self.start_pos.y),
      })
      .collect()
  }
}
