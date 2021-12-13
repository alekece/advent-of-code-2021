use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::io::{BufReader, Read};

use colored::Colorize;
use eyre::eyre;
use parse_display::FromStr;

use aoc_core::{Error, Result};

#[derive(Debug, PartialEq, Eq, Hash, FromStr, Copy, Clone)]
#[display("{x},{y}")]
struct Point {
  x: usize,
  y: usize,
}

#[derive(FromStr)]
enum Instruction {
  #[display("fold along y={0}")]
  FoldY(usize),
  #[display("fold along x={0}")]
  FoldX(usize),
}

pub struct TransparentPaper {
  dots: HashSet<Point>,
  instructions: VecDeque<Instruction>,
}

impl fmt::Display for TransparentPaper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (width, height) = self.dots.iter().fold((0, 0), |(width, height), dot| {
      (width.max(dot.x), height.max(dot.y))
    });

    let mut paper = vec![vec!['.'; width + 1]; height + 1];

    for dot in self.dots.iter() {
      paper[dot.y][dot.x] = '#';
    }

    for row in paper.iter() {
      for col in row.iter() {
        match col {
          '#' => write!(f, "{}", col.to_string().bold())?,
          _ => write!(f, "{}", col)?,
        }
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl TransparentPaper {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    let (dots, instructions) = buffer
      .split_once("\n\n")
      .map(|(dots, instructions)| {
        (
          parse_as::<Point>(dots).collect::<Result<HashSet<_>>>(),
          parse_as::<Instruction>(instructions).collect::<Result<VecDeque<_>>>(),
        )
      })
      .ok_or_else(|| eyre!("invalid transparent paper"))?;

    let (dots, instructions) = (dots?, instructions?);

    Ok(Self { dots, instructions })
  }

  pub fn count_visible_dots(&self) -> usize {
    self.dots.len()
  }

  pub fn fold_next(&mut self) {
    if let Some(instruction) = self.instructions.pop_front() {
      let (map_y, filter_y, partition_y, map_x, filter_x, partition_x);

      #[allow(clippy::type_complexity)]
      let (map, filter, partition): (
        &dyn Fn(Point) -> Point,
        &dyn Fn(&Point) -> bool,
        &dyn Fn(&Point) -> bool,
      ) = match &instruction {
        Instruction::FoldX(x) => {
          map_x = |dot: Point| Point {
            x: *x - (dot.x - *x),
            ..dot
          };
          filter_x = |dot: &Point| dot.x != *x;
          partition_x = |dot: &Point| dot.x < *x;

          (&map_x, &filter_x, &partition_x)
        }
        Instruction::FoldY(y) => {
          map_y = |dot: Point| Point {
            y: *y - (dot.y - *y),
            ..dot
          };
          filter_y = |dot: &Point| dot.y != *y;
          partition_y = |dot: &Point| dot.y < *y;

          (&map_y, &filter_y, &partition_y)
        }
      };

      self.dots = self
        .dots
        .iter()
        .copied()
        .filter(filter)
        .map(|dot| if partition(&dot) { dot } else { map(dot) })
        .collect();
    }
  }

  pub fn fold(&mut self) {
    while !self.instructions.is_empty() {
      self.fold_next();
    }
  }
}

fn parse_as<T>(buffer: &str) -> impl Iterator<Item = Result<T>> + '_
where
  T: std::str::FromStr,
  T::Err: Into<Error>,
{
  buffer.trim().split('\n').map(|s| {
    s.trim()
      .parse::<T>()
      .map_err(|e| eyre!("{}: {}", e.into(), s))
  })
}
