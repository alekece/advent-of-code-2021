mod bitset;
mod report;

use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use report::Report;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(report: Report, puzzle: Puzzle) -> usize {
  let report = report.decode();

  if let Puzzle::Part1 = puzzle {
    report.compute_power_consumption()
  } else {
    report.compute_life_support_rating()
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let report = Report::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(report, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_DATA: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

  const EXAMPLE_RESPONSE_PART1: usize = 198;
  const EXAMPLE_RESPONSE_PART2: usize = 230;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let report = Report::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(report, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_examples_input_part2() -> Result<()> {
    let report = Report::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(report, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
