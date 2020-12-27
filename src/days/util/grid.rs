use std::fmt::{Display, Error as FmtError, Formatter};

/// Kind of a cheat. I copied this code from one of my other projects.
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Grid {
    rows: u32,
    columns: u32,
    cells: Vec<char>,
}

const NEIGHBOR_DELTAS: [(i16, i16); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Grid {
    /// Creates a new `Grid` with the given dimensions of cells and cell values.
    ///
    /// # Example
    /// ```
    /// # use grid::Grid;
    /// let cells: Vec<u32> = vec![0; 100];
    /// let grid = Grid::new(10, 10, cells);
    /// ```
    pub fn new(rows: u32, columns: u32, cells: Vec<char>) -> Result<Self, &'static str> {
        if (rows as u32 * columns as u32) as usize != cells.len() {
            return Err("Number of rows and columns doesn't fit supplied collection of cells");
        }

        Ok(Self {
            rows,
            columns,
            cells,
        })
    }

    /// Lists the values of the neighbors of the cell at the given coordinates.
    pub fn neighbors(&self, row: u32, column: u32) -> Vec<char> {
        let mut result = vec![];

        NEIGHBOR_DELTAS
            .iter()
            .filter_map(|(r, c)| {
                let r = row as i16 + *r;
                let c = column as i16 + *c;
                if r < 0 || c < 0 {
                    None
                } else {
                    Some((r as u32, c as u32))
                }
            })
            .for_each(|(r, c)| {
                if let Some(index) = self.linear_index(r, c) {
                    result.push(self.cells[index].clone());
                }
            });

        result
    }

    /// Lists the values of the nearest visible neighbors of the cell at the given coordinates.
    pub fn visible_neighbors(&self, start_row: u32, start_column: u32) -> Vec<char> {
        let mut result = vec![];

        NEIGHBOR_DELTAS
            .iter()
            .filter_map(|(r, c)| {
                let mut row = start_row as i16 + *r;
                let mut column = start_column as i16 + *c;
                while row < self.rows as i16
                    && column < self.columns as i16
                    && row >= 0
                    && column >= 0
                {
                    match self.get(row as u32, column as u32) {
                        '.' => {
                            row += *r;
                            column += *c;
                        }
                        _ => return Some((row as u32, column as u32)),
                    }
                }
                None
            })
            .for_each(|(r, c)| {
                if let Some(index) = self.linear_index(r, c) {
                    result.push(self.cells[index].clone());
                }
            });

        result
    }

    fn linear_index(&self, row: u32, column: u32) -> Option<usize> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        let index = self.columns as u32 * row as u32 + column as u32;
        Some(index as usize)
    }

    /// Returns the value of the cell at the given coordinates.
    pub fn get(&self, row: u32, column: u32) -> char {
        self.cells[self.linear_index(row, column).unwrap()].clone()
    }

    /// Changes the value of the cell at the given coordinates.
    pub fn set(&mut self, row: u32, column: u32, value: char) {
        if let Some(index) = self.linear_index(row, column) {
            self.cells[index] = value.clone();
        }
    }

    /// Returns the number of rows in the grid.
    pub fn get_rows(&self) -> u32 {
        self.rows
    }

    /// Returns the number of columns in the grid.
    pub fn get_columns(&self) -> u32 {
        self.columns
    }

    pub fn count_cells_in_state(&self, state: char) -> u32 {
        self.cells.iter().filter(|c| **c == state).count() as u32
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        for line in self.cells.as_slice().chunks(self.columns as usize) {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
