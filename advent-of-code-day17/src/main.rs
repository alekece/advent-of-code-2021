mod prob;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use prob::ProbLauncher;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(launcher: ProbLauncher, puzzle: Puzzle) -> usize {
  match puzzle {
    Puzzle::Part1 => launcher.compute_coolest_launch(),
    Puzzle::Part2 => launcher.calculate_successful_launches(),
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let launcher = ProbLauncher::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(launcher, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = "target area: x=20..30, y=-10..-5";

  const EXAMPLE_RESPONSE_PART1: usize = 45;
  const EXAMPLE_RESPONSE_PART2: usize = 112;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let launcher = ProbLauncher::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(launcher, Puzzle::Part1);
    assert_eq!(result, EXAMPLE_RESPONSE_PART1);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let launcher = ProbLauncher::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(launcher, Puzzle::Part2);
    assert_eq!(result, EXAMPLE_RESPONSE_PART2);

    Ok(())
  }
}
