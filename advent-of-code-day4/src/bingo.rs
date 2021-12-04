use std::fmt;
use std::io::{BufRead, BufReader, Read};

use colored::Colorize;
use eyre::eyre;
use itertools::Itertools;

use aoc_core::Result;

#[derive(Clone)]
struct Cell {
  marked: bool,
  value: usize,
}

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.marked {
      write!(f, "{:>3}", self.value.to_string().green().bold())
    } else {
      write!(f, "{:3}", self.value)
    }
  }
}

#[derive(Clone)]
pub struct Board<const N: usize> {
  cells: Vec<Cell>,
}

impl<const N: usize> fmt::Display for Board<N> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for cells in self.cells.chunks(Self::WIDTH) {
      for cell in cells {
        write!(f, "{}", cell)?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl<const N: usize> Board<N> {
  const WIDTH: usize = N;
  const SIZE: usize = N * N;

  fn from_str(s: &str) -> Result<Self> {
    let cells = s
      .split_whitespace()
      .map(|s| s.parse::<usize>().map_err(Into::into))
      .map_ok(|value| Cell {
        marked: false,
        value,
      })
      .collect::<Result<Vec<_>>>()?;

    if cells.len() != Self::SIZE {
      return Err(eyre!(
        "Invalid board size (expected {} but got {})",
        Self::SIZE,
        cells.len()
      ));
    }

    Ok(Self { cells })
  }

  fn mark_cell(&mut self, value: usize) -> bool {
    if let Some(cell) = self.cells.iter_mut().find(|cell| cell.value == value) {
      cell.marked = true;

      true
    } else {
      false
    }
  }

  fn is_complete(&self) -> bool {
    self
      .cells
      .chunks(Self::WIDTH)
      .any(|row| row.iter().all(|cell| cell.marked))
      || (0..Self::WIDTH).into_iter().any(|i| {
        self
          .cells
          .iter()
          .skip(i)
          .step_by(Self::WIDTH)
          .all(|cell| cell.marked)
      })
  }

  pub fn sum_unmarked_cells(&self) -> usize {
    self
      .cells
      .iter()
      .filter(|cell| !cell.marked)
      .map(|cell| cell.value)
      .sum()
  }
}

pub struct BingoSubsystem {
  cursor: usize,
  drawn_numbers: Vec<usize>,
  boards: Vec<Board<5>>,
}

impl fmt::Display for BingoSubsystem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, n) in self.drawn_numbers.iter().enumerate() {
      if self.cursor > i {
        write!(f, "{},", n.to_string().strikethrough())?;
      } else {
        write!(f, "{},", n)?;
      }
    }
    writeln!(f)?;

    for board in self.boards.iter() {
      write!(f, "{}", board)?;
      writeln!(f)?;
    }

    Ok(())
  }
}

impl BingoSubsystem {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let mut reader = BufReader::new(reader);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;

    let drawn_numbers = buffer
      .split(',')
      .map(|s| s.trim().parse::<usize>().map_err(Into::into))
      .collect::<Result<Vec<_>>>()?;

    buffer.clear();
    reader.read_to_string(&mut buffer)?;

    let boards = buffer
      .split("\n\n")
      .map(Board::from_str)
      .collect::<Result<Vec<_>>>()?;

    Ok(Self {
      cursor: 0,
      drawn_numbers,
      boards,
    })
  }

  pub fn draw_number(&mut self) -> Option<(usize, Option<Board<5>>)> {
    if self.cursor >= self.drawn_numbers.len() {
      return None;
    }

    let mut winning_board = None;
    let drawn_number = self.drawn_numbers[self.cursor];

    self.cursor += 1;

    for board in self.boards.iter_mut() {
      if !board.is_complete() {
        board.mark_cell(drawn_number);

        if winning_board.is_none() && board.is_complete() {
          winning_board = Some(board.clone());
        }
      }
    }

    Some((drawn_number, winning_board))
  }
}
