mod digit;

use std::fs::File;
use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use digit::DisplayInterpreter;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(display: DisplayInterpreter, puzzle: Puzzle) -> usize {
  match puzzle {
    Puzzle::Part1 => display.count_predetermined_output_digits(),
    Puzzle::Part2 => display.decode_output_digits().iter().sum(),
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let display = DisplayInterpreter::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(display, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

  const EXAMPLE_RESPONSE_PART1: usize = 26;
  const EXAMPLE_RESPONSE_PART2: usize = 61229;

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    let display = DisplayInterpreter::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(display, Puzzle::Part1);

    assert_eq!(EXAMPLE_RESPONSE_PART1, result);

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    let display = DisplayInterpreter::from_reader(EXAMPLE_DATA.as_bytes())?;
    let result = solve_puzzle(display, Puzzle::Part2);

    assert_eq!(EXAMPLE_RESPONSE_PART2, result);

    Ok(())
  }
}
