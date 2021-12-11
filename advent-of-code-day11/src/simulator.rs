use std::convert::TryFrom;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

use bounded_integer::BoundedU32;
use colored::Colorize;
use eyre::eyre;
use itertools::Itertools;

use aoc_core::{Error, Result};

type Energy = BoundedU32<0, 9>;

enum DumboOctopus {
  Charging(Energy),
  Flashing,
}

impl DumboOctopus {
  fn update(&mut self) {
    *self = match self {
      Self::Charging(energy) if energy.get() == Energy::MAX_VALUE => Self::Flashing,
      Self::Charging(energy) => Self::Charging(*energy + 1),
      Self::Flashing => Self::Charging(unsafe { Energy::new_unchecked(Energy::MIN_VALUE) }),
    }
  }
}

impl fmt::Display for DumboOctopus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Charging(energy) if energy.get() == 0 => {
        write!(f, "{}", energy.to_string().bold())
      }
      Self::Charging(energy) => write!(f, "{}", energy),
      Self::Flashing => write!(f, "{}", "f".yellow().bold()),
    }
  }
}

impl TryFrom<char> for DumboOctopus {
  type Error = Error;

  fn try_from(value: char) -> Result<Self> {
    value
      .to_digit(10)
      .and_then(BoundedU32::new)
      .ok_or_else(|| {
        eyre!(
          "dumbo octopus energy should be included between {} and {}",
          Energy::MIN_VALUE,
          Energy::MAX_VALUE
        )
      })
      .map(Self::Charging)
  }
}

pub struct DumboOctopusSimulator(Vec<DumboOctopus>);

impl DumboOctopusSimulator {
  const GRID_SIZE: usize = 10;

  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let reader = BufReader::new(reader);

    let grid = reader
      .lines()
      .map(|line| {
        line.map(|s| {
          s.trim()
            .chars()
            .map(DumboOctopus::try_from)
            .collect::<Result<Vec<_>>>()
        })
      })
      .flatten()
      .collect::<Result<Vec<Vec<_>>>>()?;

    if grid.len() != Self::GRID_SIZE || grid.iter().any(|row| row.len() != Self::GRID_SIZE) {
      Err(eyre!("expected {0} by {0} grid", Self::GRID_SIZE))
    } else {
      // turn 2-dimension array into 1-dimension
      let grid = grid.into_iter().flatten().collect();

      Ok(Self(grid))
    }
  }

  pub fn simulate(&mut self, cycles: usize, debug: bool) -> usize {
    (0..cycles).into_iter().fold(0, |acc, _| {
      if debug {
        println!("{}\n", self);
      }
      self.0.iter_mut().for_each(DumboOctopus::update);

      acc + self.propagate_flashes()
    })
  }

  pub fn simulate_until_synchronization(&mut self, debug: bool) -> usize {
    let mut cycles = 0;

    while !self.0.iter().all(
      |dumbo_octopus| matches!(dumbo_octopus, DumboOctopus::Charging(energy) if energy.get() == 0),
    ) {
      self.simulate(1, debug);
      cycles += 1;
    }

    cycles
  }

  fn propagate_flashes(&mut self) -> usize {
    let indexes = self
      .0
      .iter()
      .enumerate()
      .filter(|(_, dumbo_octopus)| matches!(dumbo_octopus, DumboOctopus::Flashing))
      .map(|(i, _)| i)
      .collect::<Vec<_>>();

    let flash_count = indexes.len();

    for i in indexes.into_iter() {
      for j in self.get_adjacent_indexes(i).into_iter() {
        match unsafe { self.0.get_unchecked_mut(j) } {
          DumboOctopus::Charging(energy) if energy.get() == 0 => continue,
          DumboOctopus::Flashing => continue,
          dumbo_octopus => dumbo_octopus.update(),
        }
      }

      self.0[i].update();
    }

    if flash_count != 0 {
      flash_count + self.propagate_flashes()
    } else {
      0
    }
  }

  fn get_adjacent_indexes(&mut self, i: usize) -> Vec<usize> {
    let x = (i % Self::GRID_SIZE) as isize;
    let y = (i / Self::GRID_SIZE) as isize;

    [
      (x - 1, y),
      (x + 1, y),
      (x, y - 1),
      (x, y + 1),
      (x - 1, y - 1),
      (x + 1, y - 1),
      (x - 1, y + 1),
      (x + 1, y + 1),
    ]
    .into_iter()
    .filter(|(x, y)| {
      *x >= 0 && *x < Self::GRID_SIZE as isize && *y >= 0 && *y < Self::GRID_SIZE as isize
    })
    .map(|(x, y)| x as usize + y as usize * Self::GRID_SIZE)
    .collect()
  }
}

impl fmt::Display for DumboOctopusSimulator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      self
        .0
        .chunks(Self::GRID_SIZE)
        .format_with("\n", |dumbo_octopuses, f| {
          f(&dumbo_octopuses
            .iter()
            .format_with("", |dumbo_octopus, g| g(&dumbo_octopus)))
        })
    )
  }
}
