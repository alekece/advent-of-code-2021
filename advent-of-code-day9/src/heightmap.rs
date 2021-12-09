use std::convert::TryFrom;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

use bounded_integer::BoundedU32;
use colored::Colorize;
use eyre::eyre;

use aoc_core::{Error, Result};

#[derive(PartialEq, Eq)]
pub struct Cell {
  x: usize,
  y: usize,
  height: BoundedU32<0, 9>,
}

impl Cell {
  pub fn get_risk_level(&self) -> usize {
    (self.height.get() + 1) as usize
  }
}

impl TryFrom<char> for Cell {
  type Error = Error;

  fn try_from(value: char) -> Result<Self> {
    value
      .to_digit(10)
      .and_then(BoundedU32::new)
      .ok_or_else(|| eyre!("invalid '{}' height", value))
      .map(|height| Self { x: 0, y: 0, height })
  }
}

pub struct HeightMap {
  width: usize,
  height: usize,
  cells: Vec<Cell>,
}

impl fmt::Display for HeightMap {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let basins = self.get_basins();

    for cell in self.cells.iter() {
      if cell.x == 0 {
        writeln!(f)?;
      }

      if self.is_low_height(cell) {
        write!(f, "{}", cell.height.get().to_string().green())?;
      } else if basins.iter().any(|cells| cells.contains(&cell)) {
        write!(f, "{}", cell.height.get().to_string().red())?;
      } else {
        write!(f, "{}", (cell.height.get() as u8 + 48) as char)?;
      }
    }

    Ok(())
  }
}

impl HeightMap {
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
      return Err(eyre!(
        "invalid heightmap: row length should be all the same"
      ));
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

  pub fn get_basins(&self) -> Vec<Vec<&Cell>> {
    self
      .get_low_cells()
      .into_iter()
      .map(|cell| self.discover_basin(vec![], cell))
      .collect()
  }

  fn discover_basin<'a>(
    &'a self,
    mut discovered_cells: Vec<&'a Cell>,
    cell: &'a Cell,
  ) -> Vec<&'a Cell> {
    discovered_cells.push(cell);

    for adjacent_cell in self.get_adjacent_cells(cell).into_iter() {
      if adjacent_cell.height.get() != 9 && !discovered_cells.contains(&adjacent_cell) {
        discovered_cells = self.discover_basin(discovered_cells, adjacent_cell);
      }
    }

    discovered_cells
  }

  pub fn get_low_cells(&self) -> Vec<&Cell> {
    self
      .cells
      .iter()
      .filter(|cell| self.is_low_height(cell))
      .collect()
  }

  fn is_low_height(&self, cell: &Cell) -> bool {
    let adjacent_cells = [
      self.cell_at(cell.x as isize - 1, cell.y as isize),
      self.cell_at(cell.x as isize + 1, cell.y as isize),
      self.cell_at(cell.x as isize, cell.y as isize - 1),
      self.cell_at(cell.x as isize, cell.y as isize + 1),
    ];

    adjacent_cells
      .into_iter()
      .flatten()
      .all(|adjacent_cell| cell.height.get() < adjacent_cell.height.get())
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
