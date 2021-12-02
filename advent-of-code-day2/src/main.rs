mod submarine;

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

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let commands = aoc_core::read_lines_as::<Command>(opt.filename)?;

  let mut submarine: Box<dyn Console> = match opt.puzzle {
    Puzzle::Part1 => Box::new(Submarine::default()),
    Puzzle::Part2 => Box::new(FixedSubmarine::default()),
  };

  commands.iter().for_each(|command| submarine.interpret(command));

  println!(
    "{}",
    submarine.get_depth() * submarine.get_horizontal_position()
  );

  Ok(())
}
