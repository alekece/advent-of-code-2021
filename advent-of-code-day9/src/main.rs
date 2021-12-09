mod heightmap;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use heightmap::{Cell, HeightMap};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn solve_puzzle(heightmap: HeightMap, puzzle: Puzzle, debug: bool) -> usize {
  if debug {
    println!("{}", heightmap);
  }

  match puzzle {
    Puzzle::Part1 => heightmap
      .get_low_cells()
      .into_iter()
      .map(Cell::get_risk_level)
      .sum(),
    Puzzle::Part2 => {
      let mut basins = heightmap.get_basins();

      basins.sort_by_key(|basin| basin.len());

      basins
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |acc, basin| acc * basin.len())
    }
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let heightmap = HeightMap::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(heightmap, opt.puzzle, opt.debug);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678
";

  const EXAMPLE_RESPONSE_PART1: usize = 15;
  const EXAMPLE_RESPONSE_PART2: usize = 1134;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let heightmap = HeightMap::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(heightmap, Puzzle::Part1, true);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let heightmap = HeightMap::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(heightmap, Puzzle::Part2, true);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
