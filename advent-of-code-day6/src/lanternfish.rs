use std::io::{BufRead, BufReader, Read};

use aoc_core::Result;

const BIRTH_CYCLE: usize = 6;
const FIRST_BIRTH_CYCLE: usize = BIRTH_CYCLE + 2;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Lanternfish(usize);

impl Lanternfish {
  pub fn update(&mut self) -> Option<Self> {
    if self.0 == 0 {
      self.0 = BIRTH_CYCLE;

      Some(Self(FIRST_BIRTH_CYCLE))
    } else {
      self.0 -= 1;

      None
    }
  }
}

pub struct LanternfishSimulator(Vec<(Lanternfish, usize)>);

impl LanternfishSimulator {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;

    let lanternfishs = buffer
      .split(',')
      .map(|s| {
        s.trim()
          .parse::<usize>()
          .map(|x| (Lanternfish(x), 1))
          .map_err(Into::into)
      })
      .collect::<Result<Vec<_>>>()?;

    let mut simulator = LanternfishSimulator(Vec::with_capacity(FIRST_BIRTH_CYCLE));

    simulator.populate(lanternfishs);

    Ok(simulator)
  }

  fn populate(&mut self, lanternfishs: Vec<(Lanternfish, usize)>) {
    for (lanternfish, counter) in lanternfishs.into_iter() {
      if let Some((_, current_counter)) = self
        .0
        .iter_mut()
        .find(|(current_lanternfish, _)| lanternfish == *current_lanternfish)
      {
        *current_counter += counter;
      } else {
        self.0.push((lanternfish, counter));
      };
    }
  }

  pub fn simulate(&mut self, days: usize) {
    for _ in (0..days).into_iter() {
      let new_lanternfishs = self
        .0
        .iter_mut()
        .map(|(lanternfish, counter)| {
          lanternfish
            .update()
            .map(|new_lanternfish| (new_lanternfish, *counter))
        })
        .flatten()
        .collect();

      self.populate(new_lanternfishs);
    }
  }

  pub fn get_population(&self) -> usize {
    self.0.iter().map(|(_, counter)| counter).sum()
  }
}
