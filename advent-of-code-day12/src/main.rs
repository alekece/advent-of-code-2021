mod graph;
mod string;

use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use aoc_core::{Puzzle, Result};

use graph::{Graph, SearchPolicy};

#[derive(StructOpt)]
struct Opt {
  #[structopt(short, long)]
  filename: PathBuf,
  #[structopt(short, long)]
  puzzle: Puzzle,
  #[structopt(short, long)]
  debug: bool,
}

fn solve_puzzle(graph: Graph, puzzle: Puzzle, debug: bool) -> usize {
  let max_small_node_visit = match puzzle {
    Puzzle::Part1 => 1,
    Puzzle::Part2 => 2,
  };

  let policy = SearchPolicy {
    max_small_node_visit,
  };

  graph
    .get_all_paths(policy)
    .into_iter()
    .inspect(|path| {
      if debug {
        println!("{}", path);
      }
    })
    .count()
}

fn main() -> Result<()> {
  color_eyre::install()?;

  let opt = Opt::from_args();

  let graph = Graph::from_reader(File::open(opt.filename)?)?;
  let result = solve_puzzle(graph, opt.puzzle, opt.debug);

  println!("{}", result);

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  const EXAMPLE_DATA: [&str; 3] = [
    r"start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
  ];

  const EXAMPLE_RESPONSE_PART1: [usize; 3] = [10, 19, 226];
  const EXAMPLE_RESPONSE_PART2: [usize; 3] = [36, 103, 3509];

  #[test]
  fn it_solves_example_input_part1() -> Result<()> {
    for (data, expected_result) in EXAMPLE_DATA.iter().zip(EXAMPLE_RESPONSE_PART1) {
      let graph = Graph::from_reader(data.as_bytes())?;
      let result = solve_puzzle(graph, Puzzle::Part1, true);

      assert_eq!(result, expected_result);
    }

    Ok(())
  }

  #[test]
  fn it_solves_example_input_part2() -> Result<()> {
    for (data, expected_result) in EXAMPLE_DATA.iter().zip(EXAMPLE_RESPONSE_PART2) {
      let graph = Graph::from_reader(data.as_bytes())?;
      let result = solve_puzzle(graph, Puzzle::Part2, true);

      assert_eq!(result, expected_result);
    }

    Ok(())
  }
}
