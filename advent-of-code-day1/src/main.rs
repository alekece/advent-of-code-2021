use std::fs::File;
use std::path::PathBuf;

use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn solve_puzzle(depths: Vec<usize>, puzzle: Puzzle, debug: bool) -> usize {
  let depths = match puzzle {
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

      if debug {
        println!("{} ({})", depth, message);
      }

      (Some(depth), counter)
    });

  counter
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let depths = aoc_core::read_lines(File::open(opt.filename)?)?;
  let result = solve_puzzle(depths, opt.puzzle, opt.debug);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_DATA: &str = r"199
200
208
210
200
207
240
269
260
263
";

  const EXAMPLE_RESPONSE_PART1: usize = 7;
  const EXAMPLE_RESPONSE_PART2: usize = 5;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let depths = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let counter = solve_puzzle(depths, Puzzle::Part1, true);

    assert_eq!(EXAMPLE_RESPONSE_PART1, counter);

    Ok(())
  }

  #[test]
  fn it_solves_examples_input_part2() -> Result<()> {
    let depths = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let counter = solve_puzzle(depths, Puzzle::Part2, true);

    assert_eq!(EXAMPLE_RESPONSE_PART2, counter);

    Ok(())
  }
}
