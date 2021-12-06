mod lanternfish;

use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use lanternfish::LanternfishSimulator;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(mut simulator: LanternfishSimulator, puzzle: Puzzle) -> usize {
  let days = match puzzle {
    Puzzle::Part1 => 80,
    Puzzle::Part2 => 256,
  };

  simulator.simulate(days);

  simulator.get_population()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let simulator = LanternfishSimulator::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(simulator, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"3,4,3,1,2";

  const EXAMPLE_RESPONSE_PART1: usize = 5934;
  const EXAMPLE_RESPONSE_PART2: usize = 26984457539;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let simulator = LanternfishSimulator::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(simulator, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let simulator = LanternfishSimulator::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(simulator, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
