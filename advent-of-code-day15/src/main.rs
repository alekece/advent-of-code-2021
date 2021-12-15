mod pathfinder;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use pathfinder::Finder;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(mut finder: Finder, puzzle: Puzzle) -> usize {
  if let Puzzle::Part2 = puzzle {
    finder.repeat_map(5);
  }

  finder
    .find_safest_path()
    .iter()
    .map(|cell| cell.risk.get() as usize)
    .sum()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let finder = Finder::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(finder, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

  const EXAMPLE_RESPONSE_PART1: usize = 40;
  const EXAMPLE_RESPONSE_PART2: usize = 315;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let finder = Finder::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(finder, Puzzle::Part1);

    assert_eq!(result, EXAMPLE_RESPONSE_PART1);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let finder = Finder::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(finder, Puzzle::Part2);

    assert_eq!(result, EXAMPLE_RESPONSE_PART2);

    Ok(())
  }
}
