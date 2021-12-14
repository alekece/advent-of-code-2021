mod polymer;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use polymer::Polymer;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(mut polymer: Polymer, puzzle: Puzzle) -> u128 {
  let cycle = match puzzle {
    Puzzle::Part1 => 10,
    Puzzle::Part2 => 40,
  };

  polymer.polymerize(cycle)
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let polymer = Polymer::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(polymer, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

  const EXAMPLE_RESPONSE_PART1: u128 = 1588;
  const EXAMPLE_RESPONSE_PART2: u128 = 2188189693529;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let polymer = Polymer::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(polymer, Puzzle::Part1);

    assert_eq!(result, EXAMPLE_RESPONSE_PART1);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let polymer = Polymer::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(polymer, Puzzle::Part2);

    assert_eq!(result, EXAMPLE_RESPONSE_PART2);

    Ok(())
  }
}
