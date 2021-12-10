mod parser;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use parser::Parser;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(parser: Parser, puzzle: Puzzle) -> usize {
  match puzzle {
    Puzzle::Part1 => parser.compute_syntax_error_score(),
    Puzzle::Part2 => parser.compute_completion_score(),
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let parser = Parser::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(parser, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

  const EXAMPLE_RESPONSE_PART1: usize = 26397;
  const EXAMPLE_RESPONSE_PART2: usize = 288957;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let parser = Parser::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(parser, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let parser = Parser::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(parser, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
