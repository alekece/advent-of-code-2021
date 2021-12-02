use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use strum::EnumString;

pub type Error = eyre::Error;
pub type Result<T> = eyre::Result<T>;

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Puzzle {
  Part1,
  Part2,
}

pub fn read_lines_as<T>(filename: PathBuf) -> Result<Vec<T>>
where
  T: FromStr,
  T::Err: Into<Error>,
{
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  reader
    .lines()
    .map(|line| line.map(|x| x.parse::<T>().map_err(Into::into)))
    .flatten()
    .collect::<Result<Vec<_>>>()
}
