use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(mut positions: Vec<isize>, puzzle: Puzzle) -> isize {
  match puzzle {
    Puzzle::Part1 => {
      positions.sort_unstable();

      let median_position = positions[positions.len() / 2];

      positions.iter().fold(0, |fuels, position| {
        fuels + (position - median_position).abs()
      })
    }
    Puzzle::Part2 => {
      let average_position = positions.iter().sum::<isize>() as f32 / positions.len() as f32;

      let calculate_fuels = |average_position: isize| {
        positions.iter().fold(0, |fuels, position| {
          fuels + (1..=(position - average_position).abs()).sum::<isize>()
        })
      };

      calculate_fuels(average_position.floor() as isize)
        .min(calculate_fuels(average_position.ceil() as isize))
    }
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let crabs = aoc_core::split_line(File::open(opt.filename)?, ",")?;
  let result = solve_puzzle(crabs, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"16,1,2,0,4,2,7,1,2,14";

  const EXAMPLE_RESPONSE_PART1: isize = 37;
  const EXAMPLE_RESPONSE_PART2: isize = 168;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let positions = aoc_core::split_line(EXAMPLE_DATA.as_bytes(), ",")?;
    let result = solve_puzzle(positions, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let positions = aoc_core::split_line(EXAMPLE_DATA.as_bytes(), ",")?;
    let result = solve_puzzle(positions, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
