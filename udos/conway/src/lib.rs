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
        Self {
            rows: rows,
            cols: cols,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        Self {
            rows: rows,
            cols: cols,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.rows, self.cols);
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        return &(self.grid[row * self.cols + col]);
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[row * self.cols + col] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let steps = vec![-1, 0, 1];
        let mut result: Vec<(usize, usize)> = vec![];
        for i in steps {
            for j in steps {
                if i == 0 && j == 0 {
                    continue;
                }
                let xrow = row as isize + i;
                let xcol = col as isize + j;
                if xrow >= 0 && xrow < self.rows && xcol >= 0 && xcol < self.cols {
                    result.push((xrow, xcol));
                }
            }
        }
        return result;
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
        Self{ grid: grid, };
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        return &self.grid;
    }

    pub fn step(&mut self) {
        let (xrows, xcols) = self.grid.size();
        let mut result = Grid<Cell>::new(xrows, xcols);
        for i in 0..xrows {
            for j in 0..xcols {
                let neighbour = self.grid.neighbours(i, j);
                let mut counter: i32 = 0;
                for pair in neighbour {
                    if self.grid.get(pair.0, pair.1) == &Cell::Alive {
                        counter += 1;
                    }
                }
                let mut new_status = Cell::Dead;
                if counter == 3 {
                    new_status = Cell::Alive;
                }
                if self.grid.get(i, j) == &Cell::Alive && counter == 2 {
                    new_status = Cell::Alive;
                }
                result.set(new_status, i, j);
            }
        }
        self.grid = result;
    }
}
