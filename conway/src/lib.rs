#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![T::default(); rows * cols];
        Grid { rows, cols, grid }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[row * self.cols + col]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;
                if 0 <= new_row
                    && new_row < self.rows as isize
                    && 0 <= new_col
                    && new_col < self.cols as isize
                {
                    result.push((new_row as usize, new_col as usize));
                }
            }
        }
        result
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let (rows, cols) = self.grid.size();
        let mut new_grid = Grid::new(rows, cols);
        for row in 0..rows {
            for col in 0..cols {
                let cell = *self.grid.get(row, col);
                let cool_neighbors = self
                    .grid
                    .neighbours(row, col)
                    .iter()
                    .filter(|&&(r, c)| *self.grid.get(r, c) == Cell::Alive)
                    .count();
                let mut new_cell = Cell::Dead;
                if cool_neighbors == 3 || cool_neighbors == 2 && cell == Cell::Alive {
                    new_cell = Cell::Alive;
                }
                new_grid.set(new_cell, row, col);
            }
        }
        self.grid = new_grid;
    }
}
