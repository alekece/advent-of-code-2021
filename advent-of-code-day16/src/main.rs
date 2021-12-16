mod bits;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use bits::PacketDecoder;

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
}

fn solve_puzzle(decoder: PacketDecoder, puzzle: Puzzle) -> usize {
  match puzzle {
    Puzzle::Part1 => decoder.get_version(),
    Puzzle::Part2 => decoder.evaluate().unwrap(),
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let finder = PacketDecoder::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(finder, opt.puzzle);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA1: [&str; 4] = [
    "8A004A801A8002F478",
    "620080001611562C8802118E34",
    "C0015000016115A2E0802F182340",
    "A0016C880162017C3686B18A3D4780",
  ];

  const EXAMPLE_DATA2: [&str; 8] = [
    "C200B40A82",
    "04005AC33890",
    "880086C3E88112",
    "CE00C43D881120",
    "D8005AC2A8F0",
    "F600BC2D8F",
    "9C005AC2F8F0",
    "9C0141080250320F1802104A08",
  ];

  const EXAMPLE_RESPONSE_PART1: [usize; 4] = [16, 12, 23, 31];
  const EXAMPLE_RESPONSE_PART2: [usize; 8] = [3, 54, 7, 9, 1, 0, 0, 1];

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    for (data, expected_result) in EXAMPLE_DATA1.iter().zip(EXAMPLE_RESPONSE_PART1) {
      let finder = PacketDecoder::from_reader(data.as_bytes())?;
      let result = solve_puzzle(finder, Puzzle::Part1);

      assert_eq!(result, expected_result);
    }

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    for (data, expected_result) in EXAMPLE_DATA2.iter().zip(EXAMPLE_RESPONSE_PART2) {
      let finder = PacketDecoder::from_reader(data.as_bytes())?;
      let result = solve_puzzle(finder, Puzzle::Part2);

      assert_eq!(result, expected_result);
    }

    Ok(())
  }
}
