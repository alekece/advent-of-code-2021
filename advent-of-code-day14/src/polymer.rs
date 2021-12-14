use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufReader, Read};
use std::ops::AddAssign;

use aoc_core::Result;
use eyre::eyre;

pub struct Polymer {
  template: Vec<u8>,
  #[allow(clippy::type_complexity)]
  pair_insertions: HashMap<[u8; 2], (u8, Vec<HashMap<u8, u128>>)>,
}

impl Polymer {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer)?;

    let (template, pair_insertions) = buffer
      .split_once("\n\n")
      .map(|(template, pair_insertions)| {
        (
          template.trim().as_bytes().to_vec(),
          pair_insertions
            .trim()
            .split('\n')
            .map(|s| {
              match s
                .split_once("->")
                .map(|(a, b)| (a.trim().as_bytes(), b.trim().as_bytes()))
              {
                Some((&[a, b], &[c])) if a == b => Ok(([a, b], (c, vec![HashMap::from([(a, 2)])]))),
                Some((&[a, b], &[c])) => Ok(([a, b], (c, vec![HashMap::from([(a, 1), (b, 1)])]))),
                _ => Err(eyre!("invalid '{}' pair insertion", s)),
              }
            })
            .collect::<Result<HashMap<_, _>>>(),
        )
      })
      .ok_or_else(|| eyre!("invalid polymer formula"))?;

    let pair_insertions = pair_insertions?;

    Ok(Self {
      template,
      pair_insertions,
    })
  }

  pub fn polymerize(&mut self, cycle: usize) -> u128 {
    for n in 1..=cycle {
      self.pair_insertions = self
        .pair_insertions
        .clone()
        .into_iter()
        .map(|(polymers, (polymer, mut counters))| {
          let mut counter = merge_hashmap(
            self.pair_insertions[&[polymers[0], polymer]].1[n - 1].clone(),
            &self.pair_insertions[&[polymer, polymers[1]]].1[n - 1],
          );

          counter.entry(polymer).and_modify(|count| *count -= 1);

          counters.push(counter);

          (polymers, (polymer, counters))
        })
        .collect()
    }

    let final_counter =
      self
        .template
        .windows(2)
        .enumerate()
        .fold(HashMap::new(), |counter, (i, sequence)| {
          let mut counter = merge_hashmap(counter, &self.pair_insertions[sequence].1[cycle]);

          // remove duplicated polymer due to window slicing
          if i > 0 {
            counter.entry(sequence[0]).and_modify(|count| *count -= 1);
          }

          counter
        });

    let (min, max) = final_counter
      .into_iter()
      .fold((u128::MAX, u128::MIN), |(a, b), (_, count)| {
        (a.min(count), b.max(count))
      });

    max - min
  }
}

fn merge_hashmap<T, U>(mut lhs: HashMap<T, U>, rhs: &HashMap<T, U>) -> HashMap<T, U>
where
  T: Copy + Eq + Hash,
  U: Copy + AddAssign,
{
  for (key, value) in rhs.iter() {
    lhs
      .entry(*key)
      .and_modify(|x| *x += *value)
      .or_insert(*value);
  }

  lhs
}
