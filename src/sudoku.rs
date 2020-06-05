use std::cmp;
use std::fmt;
use std::ops::Range;

use core::iter::Iterator as StdIterator;

#[derive(Clone)]
pub struct Sudoku {
    cells: Vec<Option<usize>>,
    num_cells: usize,
    dimension: usize,
    block_dimension: usize
}

pub enum Region {
    Row(usize),
    Col(usize),
    Block(usize)
}

impl Sudoku {
    const H_SEP: char = '-';
    const V_SEP: char = '|';
    const EMPTY_CELL: char = '.';
    const NEW_LINE: char = '\n';
    const SPACE: char = ' ';

    pub fn new(dimension: usize) -> Self {
        let block_dimension = (dimension as f64).sqrt() as usize;
        if block_dimension.pow(2) != dimension {
            // TODO: propagate this as an error
            println!("Illegal Sudoku dimension: {}", dimension);
            std::process::exit(1);
        }

        let num_cells = dimension * dimension;

        Self {
            cells: vec![None; num_cells],
            num_cells,
            dimension,
            block_dimension
        }
    }

    pub fn from(str: &str) -> Self {
        let mut max_val = 0;
        let mut entries: Vec<Option<usize>> = Vec::new();
        let mut val_str = String::new();

        for c in str.chars() {
            if c.is_ascii_digit() {
                val_str.push(c);
            } else {
                if val_str.len() > 0 {
                    if let Ok(val) = val_str.parse::<usize>() {
                        // ignore zeros
                        if val > 0 {
                            entries.push(Some(val-1));
                            if val > max_val {
                                max_val = val;
                            }
                            val_str = String::new();
                        }
                    }
                }
                if c == Sudoku::EMPTY_CELL {
                    entries.push(None);
                }
            }
        }

        if val_str.len() > 0 {
            if let Ok(val) = val_str.parse::<usize>() {
                entries.push(Some(val-1));
                if val > max_val {
                    max_val = val;
                }
            }
        }

        // infer the dimension of this Sudoku
        let num_entries = entries.len() as f64;
        let dimension_inferred_from_entries = num_entries.sqrt().ceil() as usize;
        let dimension = cmp::max(max_val, dimension_inferred_from_entries);

        // create a new Sudoku instance and initialise the cells
        let mut sudoku = Sudoku::new(dimension);
        for (id, val) in entries.iter().enumerate() {
            sudoku.cells[id] = *val;
        }

        sudoku
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn block_dimension(&self) -> usize {
        self.block_dimension
    }

    pub fn num_cells(&self) -> usize {
        self.num_cells
    }

    fn cell_id(&self, row: usize, col: usize) -> usize {
        debug_assert!(row < self.dimension);
        debug_assert!(col < self.dimension);

        col + row * self.dimension
    }

    pub fn cell_value(&self, row: usize, col: usize) -> Option<usize> {
        let cell_index = self.cell_id(row, col);
        self.cells[cell_index]
    }

    pub fn set_cell_value(&mut self, row: usize, col: usize, val: usize) {
        debug_assert!(val < self.dimension);

        let cell_no = self.cell_id(row, col);
        self.cells[cell_no] = Some(val)
    }

    pub fn block_no(&self, row: usize, col: usize) -> usize {
        debug_assert!(row < self.dimension);
        debug_assert!(col < self.dimension);

        let block_row = row / self.block_dimension;
        let block_col = col / self.block_dimension;
        block_col + block_row * self.block_dimension
    }

    pub fn completed_cells(&self) -> Vec<(usize, usize, usize)> {
        let mut completed_cells = Vec::new();
        for col in 0..self.dimension {
            for row in 0..self.dimension {
                if let Some(val) = self.cell_value(row, col) {
                    completed_cells.push((row, col, val));
                }
            }
        }
        completed_cells
    }

    pub fn num_completed_cells(&self) -> usize {
        self.completed_cells().len()
    }

    pub fn is_completed(&self) -> bool {
        self.num_completed_cells() == self.num_cells
    }

    pub fn is_consistent(&self) -> bool {
        for i in 0..self.dimension {
            if !self.is_region_consistent(Region::Block(i)) {
                return false;
            }
            if !self.is_region_consistent(Region::Row(i)) {
                return false;
            }
            if !self.is_region_consistent(Region::Col(i)) {
                return false;
            }
        }
        true
    }

    fn is_region_consistent(&self, region: Region) -> bool {
        let mut completed = Vec::new();
        let mut uncompleted = 0;

        for cell in Iterator::region_iter(self, region) {
            let (row, col) = cell;
            if let Some(val) = self.cell_value(row, col) {
                if !completed.contains(&val) {
                    completed.push(val);
                }
            } else {
                uncompleted += 1;
            }
        }

        completed.len() + uncompleted == self.dimension
    }

    pub fn to_string(&self) -> String {
        let chars_per_cell = self.dimension.to_string().len();
        let chars_wide = ((chars_per_cell + 1) * self.dimension) + (self.block_dimension * 2) + 1;
        let horiz_rule = &Sudoku::H_SEP.to_string().repeat(chars_wide);
        let mut sud_str = String::new();

        for row in 0..self.dimension {
            // add the horizontal lines
            if row % self.block_dimension == 0 {
                sud_str.push_str(horiz_rule);
                sud_str.push(Sudoku::NEW_LINE);
            }

            for col in 0..self.dimension {
                // check if a block separator is needed
                if col % self.block_dimension == 0 {
                    if col > 0 {
                        sud_str.push(Sudoku::SPACE);
                    }
                    sud_str.push(Sudoku::V_SEP);
                }

                // add the contents of each cell
                sud_str.push(Sudoku::SPACE);

                let cell_str = match self.cell_value(row, col) {
                        Some(val) => (val + 1).to_string(),
                        None => Sudoku::EMPTY_CELL.to_string()
                };

                sud_str = format!("{}{:>w$}", sud_str, cell_str, w=chars_per_cell);
            }

            // close off the end of the row
            sud_str.push(Sudoku::SPACE);
            sud_str.push(Sudoku::V_SEP);
            sud_str.push(Sudoku::NEW_LINE);
        }

        sud_str.push_str(horiz_rule);
        sud_str
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Iterator {
    cols: Range<usize>,
    rows: Range<usize>,
    curr_id: Option<(usize, usize)>,
    next_id: Option<(usize, usize)>,
    dimension: usize
}

impl Iterator {

    pub fn new(sudoku: &Sudoku, cols: Range<usize>, rows: Range<usize>) -> Iterator {
        let mut sud_iter = Iterator {cols,
                                     rows,
                                     curr_id: None,
                                     next_id: None,
                                     dimension: sudoku.dimension};
        sud_iter.reset();
        sud_iter
    }

    pub fn block_iter(sudoku: &Sudoku, block: usize) -> Iterator {
        let col = (block % sudoku.block_dimension) * sudoku.block_dimension;
        let row = (block / sudoku.block_dimension) * sudoku.block_dimension;
        Iterator::new(sudoku, col..col + sudoku.block_dimension, row..row + sudoku.block_dimension)
    }

    pub fn block_for_cell_iter(sudoku: &Sudoku, cell: (usize, usize)) -> Iterator {
        let (col, row) = cell;
        let col = col / sudoku.block_dimension * sudoku.block_dimension;
        let row = row / sudoku.block_dimension * sudoku.block_dimension;
        Iterator::new(sudoku, col..col + sudoku.block_dimension, row..row + sudoku.block_dimension)
    }

    pub fn col_iter(sudoku: &Sudoku, col: usize) -> Iterator {
        Iterator::new(sudoku, col..col+1, 0..sudoku.dimension)
    }

    pub fn row_iter(sudoku: &Sudoku, row: usize) -> Iterator {
        Iterator::new(sudoku, 0..sudoku.dimension, row..row+1)
    }

    pub fn region_iter(sudoku: &Sudoku, region: Region) -> Iterator {
        match region {
            Region::Block(block) => Iterator::block_iter(sudoku, block),
            Region::Col(col) => Iterator::col_iter(sudoku, col),
            Region::Row(row) => Iterator::row_iter(sudoku, row)
        }
    }

    pub fn reset(&mut self) {
        self.next_id = None;

        if self.cols.start > self.dimension || self.rows.start > self.dimension {
            self.curr_id = None;
        }

        self.curr_id = Some((self.cols.start, self.rows.start));
    }
}

impl StdIterator for Iterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if let Some((col, row)) = self.curr_id {
            let this = self.curr_id;

            let mut next_col = col + 1;
            let mut next_row = row;
            if next_col >= self.cols.end || next_col >= self.dimension {
                next_col = self.cols.start;
                next_row = row + 1;
            }

            self.curr_id = {
                if next_row >= self.rows.end || next_row >= self.dimension {
                    None
                } else {
                    Some((next_col, next_row))
                }
            };

            this
        } else {
            None
        }
    }
}