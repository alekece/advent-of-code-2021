use std::collections::VecDeque;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Read};

use bounded_integer::BoundedU32;
use eyre::eyre;

use aoc_core::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Cell {
  pub x: usize,
  pub y: usize,
  pub risk: BoundedU32<1, 9>,
}

impl Cell {
  fn get_index(&self, finder: &Finder) -> usize {
    self.x + self.y * finder.width
  }
}

impl TryFrom<char> for Cell {
  type Error = Error;

  fn try_from(c: char) -> Result<Self> {
    c.to_digit(10)
      .and_then(BoundedU32::new)
      .ok_or_else(|| eyre!("invalid '{}' risk level", c))
      .map(|risk| Self { x: 0, y: 0, risk })
  }
}

pub struct Finder {
  width: usize,
  height: usize,
  cells: Vec<Cell>,
}

impl Finder {
  pub fn from_reader(reader: impl Read) -> Result<Self> {
    let reader = BufReader::new(reader);

    let cells = reader
      .lines()
      .enumerate()
      .map(|(y, line)| {
        line.map(|s| {
          s.trim()
            .chars()
            .enumerate()
            .map(|(x, c)| {
              Ok(Cell {
                x,
                y,
                ..Cell::try_from(c)?
              })
            })
            .collect::<Result<Vec<_>>>()
        })
      })
      .flatten()
      .collect::<Result<Vec<Vec<_>>>>()?;

    if cells.windows(2).any(|rows| rows[0].len() != rows[1].len()) {
      return Err(eyre!("invalid map: row length should be all the same"));
    }

    let width = cells[0].len();
    let height = cells.len();
    let cells = cells.into_iter().flatten().collect::<Vec<_>>();

    Ok(Self {
      width,
      height,
      cells,
    })
  }

  pub fn repeat_map(&mut self, cycle: usize) {
    self.cells = self
      .cells
      .chunks(self.width)
      .map(|cells| {
        (0..cycle)
          .into_iter()
          .map(|n| {
            let width = self.width;

            cells.iter().map(move |cell| Cell {
              x: cell.x + (width * n),
              y: cell.y,
              risk: compute_risk(cell, n),
            })
          })
          .flatten()
      })
      .flatten()
      .collect::<Vec<_>>();

    self.width *= cycle;

    self.cells = (0..cycle)
      .into_iter()
      .map(|n| {
        let height = self.height;

        self.cells.iter().map(move |cell| Cell {
          y: cell.y + (height * n),
          x: cell.x,
          risk: compute_risk(cell, n),
        })
      })
      .flatten()
      .collect::<Vec<_>>();

    self.height *= cycle;
  }

  pub fn find_safest_path(&self) -> Vec<&Cell> {
    let mut weights = vec![usize::MAX; self.cells.len()];
    let start_cell = self.cells.first().unwrap();

    weights[0] = 0;

    self.weight_cells(VecDeque::from([(0, start_cell)]), &mut weights);

    let mut cell = self.cells.last().unwrap();
    let mut path = vec![];

    while cell != start_cell {
      path.push(cell);

      cell = self
        .get_adjacent_cells(cell)
        .into_iter()
        .min_by_key(|adjacent_cell| weights[adjacent_cell.get_index(self)])
        .unwrap();
    }

    path
  }

  fn weight_cells<'a>(&'a self, mut cells: VecDeque<(usize, &'a Cell)>, weights: &mut Vec<usize>) {
    while let Some((weight, cell)) = cells.pop_front() {
      let adjacent_cells = self
        .get_adjacent_cells(cell)
        .into_iter()
        .collect::<Vec<_>>();

      for cell in adjacent_cells {
        let current_weight = cell.risk.get() as usize + weight;
        let weight = &mut weights[cell.get_index(self)];

        if *weight > current_weight {
          *weight = current_weight;

          cells.push_back((*weight, cell));
        }
      }
    }
  }

  fn get_adjacent_cells(&self, cell: &Cell) -> Vec<&Cell> {
    let adjacent_cells = [
      self.cell_at(cell.x as isize - 1, cell.y as isize),
      self.cell_at(cell.x as isize + 1, cell.y as isize),
      self.cell_at(cell.x as isize, cell.y as isize - 1),
      self.cell_at(cell.x as isize, cell.y as isize + 1),
    ];

    adjacent_cells.into_iter().flatten().collect()
  }

  fn cell_at(&self, x: isize, y: isize) -> Option<&Cell> {
    (x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize).then(|| unsafe {
      self
        .cells
        .get_unchecked(x as usize + y as usize * self.width)
    })
  }
}

fn compute_risk(cell: &Cell, shift: usize) -> BoundedU32<1, 9> {
  let risk = cell.risk.get() as usize + shift;
  let risk = if risk > 9 { risk % 10 + 1 } else { risk };

  unsafe { BoundedU32::new_unchecked(risk as u32) }
}
