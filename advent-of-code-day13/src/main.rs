mod instruction;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use instruction::TransparentPaper;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(mut paper: TransparentPaper, puzzle: Puzzle) -> usize {
  match puzzle {
    Puzzle::Part1 => {
      paper.fold_next();

      paper.count_visible_dots()
    }
    Puzzle::Part2 => {
      paper.fold();

      println!("{}", paper);

      0
    }
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let paper = TransparentPaper::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(paper, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

  const EXAMPLE_RESPONSE_PART1: usize = 17;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let paper = TransparentPaper::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(paper, Puzzle::Part1);

    assert_eq!(result, EXAMPLE_RESPONSE_PART1);

    Ok(())
  }
}
