use parse_display::FromStr;
use strum::EnumString;

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
  Forward,
  Up,
  Down,
}

#[derive(FromStr)]
#[display("{direction} {value}")]
pub struct Command {
  direction: Direction,
  value: isize,
}

pub trait Console {
  fn interpret(&mut self, command: &Command);
  fn get_depth(&self) -> isize;
  fn get_horizontal_position(&self) -> isize;
}

#[derive(Default)]
pub struct Submarine {
  depth: isize,
  horizontal_position: isize,
}

impl Console for Submarine {
  fn interpret(&mut self, command: &Command) {
    match command.direction {
      Direction::Forward => self.horizontal_position += command.value,
      Direction::Up => self.depth -= command.value,
      Direction::Down => self.depth += command.value,
    }
  }

  fn get_depth(&self) -> isize {
    self.depth
  }

  fn get_horizontal_position(&self) -> isize {
    self.horizontal_position
  }
}

#[derive(Default)]
pub struct FixedSubmarine {
  depth: isize,
  horizontal_position: isize,
  aim: isize,
}

impl Console for FixedSubmarine {
  fn interpret(&mut self, command: &Command) {
    match command.direction {
      Direction::Forward => {
        self.horizontal_position += command.value;
        self.depth += self.aim * command.value;
      }
      Direction::Up => self.aim -= command.value,
      Direction::Down => self.aim += command.value,
    }
  }

  fn get_depth(&self) -> isize {
    self.depth
  }

  fn get_horizontal_position(&self) -> isize {
    self.horizontal_position
  }
}
