mod vent;

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use vent::{Point, Vent};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(vents: Vec<Vent>, puzzle: Puzzle) -> usize {
  let mut intersections = HashMap::<Point, usize>::new();

  for point in vents
    .iter()
    .filter_map(|vent| match (&puzzle, vent.is_ortholinear()) {
      (Puzzle::Part1, true) | (Puzzle::Part2, _) => Some(vent.get_path()),
      _ => None,
    })
    .flatten()
  {
    intersections
      .entry(point)
      .and_modify(|counter| *counter += 1)
      .or_insert(1);
  }

  intersections.iter().fold(
    0,
    |count, (_, counter)| if *counter >= 2 { count + 1 } else { count },
  )
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let vents = aoc_core::read_lines(File::open(opt.filename)?)?;
  let result = solve_puzzle(vents, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

  const EXAMPLE_RESPONSE_PART1: usize = 5;
  const EXAMPLE_RESPONSE_PART2: usize = 12;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let vents = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(vents, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let vents = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(vents, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
