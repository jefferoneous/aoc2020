use std::fmt::{Display, Error as FmtError, Formatter};

/// An unnecessarily generic grid of cells
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Grid<T: Default + Clone> {
    rows: u32,
    columns: u32,
    cells: Vec<T>,
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

impl<T: Default + Clone + PartialEq> Grid<T> {
    /// Creates a new `Grid` with the given dimensions of cells using the default value of the cell type.
    ///
    /// # Example
    /// ```
    /// # use grid::Grid;
    /// let grid: Grid<u32> = Grid::new(100, 100);
    /// ```
    pub fn new(rows: u32, columns: u32) -> Self {
        Self {
            rows,
            columns,
            cells: vec![T::default(); (rows as u32 * columns as u32) as usize],
        }
    }

    /// Creates a new `Grid` with the given dimensions of cells and cell values.
    ///
    /// # Example
    /// ```
    /// # use grid::Grid;
    /// let cells: Vec<u32> = vec![0; 100];
    /// let grid = Grid::with_cells(10, 10, cells);
    /// ```
    pub fn with_cells(rows: u32, columns: u32, cells: Vec<T>) -> Result<Self, &'static str> {
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
    pub fn neighbors(&self, row: u32, column: u32) -> Vec<T> {
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

    fn linear_index(&self, row: u32, column: u32) -> Option<usize> {
        if row >= self.rows || column >= self.columns {
            return None;
        }

        let index = self.columns as u32 * row as u32 + column as u32;
        Some(index as usize)
    }

    /// Returns the value of the cell at the given coordinates.
    pub fn get(&self, row: u32, column: u32) -> T {
        self.cells[self.linear_index(row, column).unwrap()].clone()
    }

    /// Changes the value of the cell at the given coordinates.
    pub fn set(&mut self, row: u32, column: u32, value: T) {
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

    pub fn count_cells_in_state(&self, state: T) -> u32 {
        self.cells.iter().filter(|c| **c == state).count() as u32
    }
}

impl<T: Default + Clone + Display> Display for Grid<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_has_default_values_in_cells() {
        let grid: Grid<u32> = Grid::new(3, 3);
        assert_eq!(grid.cells, vec![0; 9]);
    }

    #[test]
    fn neighbors_is_correct_for_each_cell() {
        // 20 21 22
        // 23 24 25
        // 26 27 28
        let initial_cells = (20..=28).collect::<Vec<u32>>();

        let expected_neighbors = vec![
            vec![21, 23, 24],
            vec![20, 22, 23, 24, 25],
            vec![21, 24, 25],
            vec![20, 21, 24, 26, 27],
            vec![20, 21, 22, 23, 25, 26, 27, 28],
            vec![21, 22, 24, 27, 28],
            vec![23, 24, 27],
            vec![23, 24, 25, 26, 28],
            vec![24, 25, 27],
        ];

        let grid: Grid<u32> = Grid::with_cells(3, 3, initial_cells).unwrap();

        for r in 0..3 {
            for c in 0..3 {
                assert_eq!(
                    grid.neighbors(r, c),
                    expected_neighbors[(r * 3 + c) as usize],
                    "(r, c) = ({}, {})",
                    r,
                    c
                );
            }
        }
    }
}
