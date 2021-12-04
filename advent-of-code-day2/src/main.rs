mod submarine;

use std::fs::File;
use std::path::PathBuf;

use structopt::StructOpt;

use aoc_core::{Puzzle, Result};
use submarine::{Command, Console, FixedSubmarine, Submarine};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(commands: Vec<Command>, puzzle: Puzzle) -> isize {
  let mut submarine: Box<dyn Console> = match puzzle {
    Puzzle::Part1 => Box::new(Submarine::default()),
    Puzzle::Part2 => Box::new(FixedSubmarine::default()),
  };

  commands
    .iter()
    .for_each(|command| submarine.interpret(command));

  submarine.get_depth() * submarine.get_horizontal_position()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let commands = aoc_core::read_lines(File::open(opt.filename)?)?;
  let result = solve_puzzle(commands, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_DATA: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2
";

  const EXAMPLE_RESPONSE_PART1: isize = 150;
  const EXAMPLE_RESPONSE_PART2: isize = 900;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let commands = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(commands, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_examples_input_part2() -> Result<()> {
    let commands = aoc_core::read_lines(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(commands, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
