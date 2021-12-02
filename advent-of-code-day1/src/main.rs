use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use eyre::Result;
use structopt::StructOpt;
use strum::EnumString;

#[derive(EnumString)]
#[strum(serialize_all = "lowercase")]
enum Puzzle {
  Part1,
  Part2,
}

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn read_lines_as<T>(filename: PathBuf) -> Result<Vec<T>>
where
  T: FromStr,
  T::Err: Into<eyre::Error>,
{
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  reader
    .lines()
    .map(|line| line.map(|x| x.parse::<T>().map_err(Into::into)))
    .flatten()
    .collect::<Result<Vec<_>>>()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let depths = read_lines_as::<usize>(opt.filename)?;

  let depths = match opt.puzzle {
    Puzzle::Part1 => depths,
    Puzzle::Part2 => depths
      .windows(3)
      .map(|depths| depths.iter().sum())
      .collect(),
  };

  let (_, counter) = depths
    .iter()
    .fold((None, 0), |(previous_depth, counter), depth| {
      let (message, counter) = match previous_depth {
        None => ("N/A - previous measurement", 0),
        Some(previous_depth) if depth > previous_depth => ("increased", counter + 1),
        _ => ("decreased", counter),
      };

      if opt.debug {
        println!("{} ({})", depth, message);
      }

      (Some(depth), counter)
    });

  println!(
    "There are {} measurements that are larger than the previous measurement",
    counter
  );

  Ok(())
}
