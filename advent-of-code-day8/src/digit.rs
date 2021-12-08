use eyre::eyre;
use itertools::Itertools;
use std::io::Read;
use std::str::FromStr;

use aoc_core::{Error, Result};

struct SignalReport {
  unique_digits: Vec<String>,
  output_digits: Vec<String>,
}

fn split_signals(s: &str) -> Vec<String> {
  s.split_whitespace()
    .map(|s| s.chars().sorted().collect::<String>())
    .collect()
}

impl FromStr for SignalReport {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let (unique_digits, output_digits) = s
      .split_once('|')
      .map(|(unique_digits, output_digits)| {
        (split_signals(unique_digits), split_signals(output_digits))
      })
      .ok_or_else(|| eyre!("Missing '|' separator"))?;

    Ok(Self {
      unique_digits,
      output_digits,
    })
  }
}

pub struct DisplayInterpreter(Vec<SignalReport>);

impl DisplayInterpreter {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let signal_reports = aoc_core::read_lines(reader)?;

    Ok(Self(signal_reports))
  }

  pub fn count_predetermined_output_digits(&self) -> usize {
    let known_digit_lengths = [2, 3, 4, 7];

    self
      .0
      .iter()
      .map(|signal| {
        signal
          .output_digits
          .iter()
          .filter(|digit| known_digit_lengths.contains(&digit.len()))
          .count()
      })
      .sum()
  }

  pub fn decode_output_digits(&self) -> Vec<usize> {
    self
      .0
      .iter()
      .map(|signal| {
        for wires in ['a', 'b', 'c', 'd', 'e', 'f', 'g']
          .iter()
          .copied()
          .permutations(7)
        {
          if verify_wires(&wires[..], &signal.unique_digits[..]) {
            return decode_digits(&wires[..], &signal.output_digits[..]);
          }
        }

        0
      })
      .collect()
  }
}

fn decode_digits(wires: &[char], digits: &[String]) -> usize {
  digits.iter().fold(0, |acc, digit| {
    for (value, mask) in create_masks(digit) {
      let wires = apply_mask(wires, mask);

      if wires.eq(digit) {
        return (acc * 10) + value;
      }
    }

    acc
  })
}

fn verify_wires(wires: &[char], digits: &[String]) -> bool {
  digits.iter().all(|digit| {
    for (_, mask) in create_masks(digit) {
      let wires = apply_mask(wires, mask);

      if wires.eq(digit) {
        return true;
      }
    }

    false
  })
}

fn apply_mask(wires: &[char], mask: [u8; 7]) -> String {
  wires
    .iter()
    .enumerate()
    .filter_map(|(i, c)| (mask[i] == 1).then(|| *c))
    .sorted()
    .collect::<String>()
}

fn create_masks(digits: &str) -> Vec<(usize, [u8; 7])> {
  match digits.len() {
    2 => vec![(1, [0, 0, 1, 0, 0, 1, 0])],
    3 => vec![(7, [1, 0, 1, 0, 0, 1, 0])],
    4 => vec![(4, [0, 1, 1, 1, 0, 1, 0])],
    7 => vec![(8, [1, 1, 1, 1, 1, 1, 1])],
    5 => vec![
      (2, [1, 0, 1, 1, 1, 0, 1]),
      (3, [1, 0, 1, 1, 0, 1, 1]),
      (5, [1, 1, 0, 1, 0, 1, 1]),
    ],
    6 => vec![
      (0, [1, 1, 1, 0, 1, 1, 1]),
      (6, [1, 1, 0, 1, 1, 1, 1]),
      (9, [1, 1, 1, 1, 0, 1, 1]),
    ],
    _ => unreachable!(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_verifies_digits() -> Result<()> {
    let input =
      "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let display = DisplayInterpreter::from_reader(input.as_bytes())?;
    let wires = vec!['d', 'e', 'a', 'f', 'g', 'b', 'c'];

    assert!(verify_wires(&wires[..], &display.0[0].unique_digits[..]));
    assert_eq!(display.decode_output_digits().iter().sum::<usize>(), 5353);

    Ok(())
  }
}
