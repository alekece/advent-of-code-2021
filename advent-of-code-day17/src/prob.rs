use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};
use std::ops::RangeInclusive;
use std::str::FromStr;

use eyre::eyre;
use itertools::Itertools;
use regex::Regex;

use aoc_core::{Error, Result};

struct Area {
  x: RangeInclusive<isize>,
  y: RangeInclusive<isize>,
}

impl FromStr for Area {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let re = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();

    let values = &re
      .captures_iter(s)
      .map(|capture| {
        (1..=4)
          .into_iter()
          .map(|i| {
            capture[i]
              .parse::<isize>()
              .map_err(|e| eyre!("malformed area: {}", e))
          })
          .collect::<Result<Vec<_>>>()
      })
      .collect::<Result<Vec<Vec<_>>>>()?[0];

    Ok(Self {
      x: values[0]..=values[1],
      y: values[2]..=values[3],
    })
  }
}

pub struct ProbLauncher {
  target_area: Area,
}

impl ProbLauncher {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;

    let target_area = buffer.trim().parse()?;

    Ok(Self { target_area })
  }

  pub fn compute_coolest_launch(&self) -> usize {
    let mut y = 0;
    let mut dy = self.get_highest_dy();

    while dy > 0 {
      y += dy;
      dy -= 1
    }

    y as usize
  }

  fn get_highest_dy(&self) -> isize {
    (self.target_area.y.start() * -1) - 1
  }

  pub fn calculate_successful_launches(&self) -> usize {
    (0..=*self.target_area.x.end())
      .into_iter()
      .cartesian_product(*self.target_area.y.start()..=self.get_highest_dy())
      .filter(|(dx, dy)| emulate_launch(*dx, *dy, &self.target_area))
      .count()
  }
}

fn emulate_launch(mut dx: isize, mut dy: isize, area: &Area) -> bool {
  let mut x = 0;
  let mut y = 0;

  loop {
    if area.x.contains(&x) && area.y.contains(&y) {
      return true;
    }

    if y < *area.y.start() {
      return false;
    }

    x += dx;
    y += dy;

    dx = match dx.cmp(&0) {
      Ordering::Equal => 0,
      Ordering::Greater => dx - 1,
      Ordering::Less => dx + 1,
    };
    dy -= 1;
  }
}
