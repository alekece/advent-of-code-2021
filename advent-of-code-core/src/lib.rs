use std::io::{BufRead, BufReader, Read};
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

pub fn read_lines<T>(reader: impl Read) -> Result<Vec<T>>
where
  T: FromStr,
  T::Err: Into<Error>,
{
  let reader = BufReader::new(reader);

  reader
    .lines()
    .map(|line| line.map(|x| x.parse::<T>().map_err(Into::into)))
    .flatten()
    .collect::<Result<Vec<_>>>()
}

pub fn split_line<T>(reader: impl Read, separator: &str) -> Result<Vec<T>>
where
  T: FromStr,
  T::Err: Into<Error>,
{
  let mut reader = BufReader::new(reader);
  let mut buffer = String::new();

  reader.read_line(&mut buffer)?;

  buffer
    .split(separator)
    .map(|x| x.trim().parse::<T>().map_err(Into::into))
    .collect::<Result<Vec<_>>>()
}
