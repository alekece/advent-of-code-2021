use std::io::Read;

use aoc_core::Result;

use crate::bitset::BitSet;

pub struct Report(Vec<BitSet>);

impl Report {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let bit_sets = aoc_core::read_lines(reader)?;

    Ok(Self(bit_sets))
  }

  pub fn decode(self) -> DecodedReport {
    let max_bit_set_len = self
      .0
      .iter()
      .map(|bit_set| bit_set.len())
      .max()
      .unwrap_or_default();

    let bit_counters = self
      .0
      .iter()
      .fold(vec![0; max_bit_set_len], |mut bit_counters, bit_set| {
        for (i, bit) in bit_set.iter().enumerate() {
          bit_counters[i] += usize::from(*bit);
        }

        bit_counters
      });

    // pivot value to consider a bit as the most common bit
    let bit_counter_pivot = (self.0.len() as f32 / 2.0).ceil() as usize;

    let most_common_bits = BitSet(
      bit_counters
        .into_iter()
        .map(|counter| counter >= bit_counter_pivot)
        .collect(),
    );

    DecodedReport {
      most_common_bits,
      bit_sets: self.0,
    }
  }
}

pub struct DecodedReport {
  pub(crate) most_common_bits: BitSet,
  pub(crate) bit_sets: Vec<BitSet>,
}

impl DecodedReport {
  pub fn compute_power_consumption(&self) -> usize {
    let gamma_rate = self
      .most_common_bits
      .iter()
      .fold(0_usize, |gamma_rate, bit| {
        let bit = if *bit { 1 } else { 0 };

        (gamma_rate << 1) + bit
      });

    let epsilon_rate = {
      let mask = (1 << (usize::BITS - gamma_rate.leading_zeros())) - 1;

      gamma_rate ^ mask
    };

    gamma_rate * epsilon_rate
  }

  pub fn compute_life_support_rating(&self) -> usize {
    let oxigen_generator_rating = compute_life_support_rating(|bit| bit, 0, self);
    let c02_scrubber_rating = compute_life_support_rating(|bit| !bit, 0, self);

    oxigen_generator_rating * c02_scrubber_rating
  }
}

fn compute_life_support_rating(
  f: impl Fn(bool) -> bool,
  i: usize,
  report: &DecodedReport,
) -> usize {
  let bit = f(report.most_common_bits[i]);

  let found_bit_sets = report
    .bit_sets
    .iter()
    .filter(|bit_set| bit_set[i] == bit)
    .cloned()
    .collect::<Vec<_>>();

  match found_bit_sets.len() {
    1 => found_bit_sets[0].to_usize(),
    0 => compute_life_support_rating(f, i + 1, report),
    _ => {
      let report = Report(found_bit_sets).decode();

      compute_life_support_rating(f, i + 1, &report)
    }
  }
}
