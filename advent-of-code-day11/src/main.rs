mod simulator;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use simulator::DumboOctopusSimulator;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn solve_puzzle(mut simulator: DumboOctopusSimulator, puzzle: Puzzle, debug: bool) -> usize {
  match puzzle {
    Puzzle::Part1 => simulator.simulate(100, debug),
    Puzzle::Part2 => simulator.simulate_until_synchronization(debug),
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let simulator = DumboOctopusSimulator::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(simulator, opt.puzzle, opt.debug);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

  const EXAMPLE_RESPONSE_PART1: usize = 1656;
  const EXAMPLE_RESPONSE_PART2: usize = 195;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let simulator = DumboOctopusSimulator::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(simulator, Puzzle::Part1, true);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let simulator = DumboOctopusSimulator::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(simulator, Puzzle::Part2, true);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
