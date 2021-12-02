use std::path::PathBuf;

use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();
  let depths = aoc_core::read_lines_as::<usize>(opt.filename)?;

  let depths = match opt.puzzle {
    Puzzle::Part1 => depths,
    Puzzle::Part2 => depths
      .windows(3)
      .map(|depths| depths.iter().sum())
      .collect(),
  };

  let (_, counter) = depths
    .iter()
    .fold((None, 0), |(previous_depth, counter), depth| {
      let (message, counter) = match previous_depth {
        None => ("N/A - previous measurement", 0),
        Some(previous_depth) if depth > previous_depth => ("increased", counter + 1),
        _ => ("decreased", counter),
      };

      if opt.debug {
        println!("{} ({})", depth, message);
      }

      (Some(depth), counter)
    });

  println!(
    "There are {} measurements that are larger than the previous measurement",
    counter
  );

  Ok(())
}
