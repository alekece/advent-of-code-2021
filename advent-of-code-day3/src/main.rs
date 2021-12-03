mod bitset;
mod report;

use std::path::PathBuf;

use aoc_core::{Puzzle, Result};
use structopt::StructOpt;

use report::Report;

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
  let report = Report::from_file(opt.filename)?;
  let report = report.decode();

  let result = if let Puzzle::Part1 = opt.puzzle {
    report.compute_power_consumption()
  } else {
    report.compute_life_support_rating()
  };

  println!("{}", result);

  Ok(())
}
