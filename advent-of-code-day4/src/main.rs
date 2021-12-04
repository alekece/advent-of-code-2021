mod bingo;

use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use bingo::BingoSubsystem;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn solve_puzzle(mut bingo: BingoSubsystem, puzzle: Puzzle, debug: bool) -> usize {
  let mut last_win = None;

  while let Some((drawn_number, board)) = bingo.draw_number() {
    if debug {
      println!("Bingo status (drawn number {}):\n{}", drawn_number, bingo);
    }

    if let Some(board) = board {
      if debug {
        println!("Found winning board:\n{}", board);
      }

      last_win = Some((drawn_number, board));

      if let Puzzle::Part1 = puzzle {
        break;
      }
    }
  }

  last_win
    .map(|(drawn_number, board)| drawn_number * board.sum_unmarked_cells())
    .unwrap_or_default()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let bingo = BingoSubsystem::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(bingo, opt.puzzle, opt.debug);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

  const EXAMPLE_RESPONSE_PART1: usize = 4512;
  const EXAMPLE_RESPONSE_PART2: usize = 1924;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let bingo = BingoSubsystem::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(bingo, Puzzle::Part1, true);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let bingo = BingoSubsystem::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(bingo, Puzzle::Part2, true);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
