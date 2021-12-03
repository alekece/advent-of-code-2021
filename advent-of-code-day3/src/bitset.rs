use derive_more::Deref;
use eyre::eyre;
use std::str::FromStr;

#[derive(Debug, Deref, Clone)]
pub struct BitSet(pub(crate) Vec<bool>);

impl FromStr for BitSet {
  type Err = aoc_core::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(
      s.chars()
        .enumerate()
        .map(|(i, c)| match c {
          '0' => Ok(false),
          '1' => Ok(true),
          _ => Err(eyre!("Invalid bit '{}' value at {} index", c, i)),
        })
        .collect::<aoc_core::Result<Vec<_>>>()?,
    ))
  }
}

impl BitSet {
  pub fn to_usize(&self) -> usize {
    self.0.iter().fold(0_usize, |acc, v| {
      let v = if *v { 1 } else { 0 };

      (acc << 1) + v
    })
  }
}

impl std::fmt::Display for BitSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for bit in self.0.iter() {
      write!(f, "{}", *bit as usize)?;
    }

    Ok(())
  }
}
