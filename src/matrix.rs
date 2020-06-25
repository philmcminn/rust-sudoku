use std::ops::Range;

use core::iter::Iterator as StdIterator;

#[derive(Clone)]
pub struct Matrix<T: Clone> {
    elements: Vec<T>,
    num_rows: usize,
    num_cols: usize,
    num_elements: usize
}

impl<T: Clone> Matrix<T> {
    pub fn new(default: T, num_rows: usize, num_cols: usize) -> Self {
        let num_elements = num_rows * num_cols;
        let elements = vec![default; num_elements];
        let mat = Self {
            elements,
            num_rows,
            num_cols,
            num_elements
        };
        mat
    }

    fn element_index(&self, row: usize, col: usize) -> usize {
        if row >= self.num_rows {
            panic!("Trying to access a row {} that is out of bounds (0..{})",
                   row, self.num_rows);
        }
        if col >= self.num_cols {
            panic!("Trying to access a col {} that is out of bounds (0..{})",
                   col, self.num_cols);
        }
        col + (row * self.num_cols)
    }

    pub fn element(&self, row: usize, col: usize) -> &T {
        let index = self.element_index(row, col);
        &self.elements[index]
    }

    pub fn set_element(&mut self, row: usize, col: usize, val: T) {
        let index = self.element_index(row, col);
        self.elements[index] = val
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_elements(&self) -> usize {
        self.num_elements
    }
}


pub struct Iterator {
    rows: Range<usize>,
    cols: Range<usize>,
    curr_id: Option<(usize, usize)>,
    next_id: Option<(usize, usize)>,
    num_rows: usize,
    num_cols: usize
}

impl Iterator {

    pub fn new<T: Clone>(matrix: &Matrix<T>, cols: Range<usize>, rows: Range<usize>) -> Iterator {
        let mut matrix_iter = Iterator {rows,
                                        cols,
                                        curr_id: None,
                                        next_id: None,
                                        num_rows: matrix.num_rows,
                                        num_cols: matrix.num_cols};
        matrix_iter.reset();
        matrix_iter
    }

    pub fn with_col<T: Clone>(matrix: &Matrix<T>, col: usize) -> Iterator {
        Iterator::new(matrix, col..col+1, 0..matrix.num_rows())
    }

    pub fn with_row<T: Clone>(matrix: &Matrix<T>, row: usize) -> Iterator {
        Iterator::new(matrix, 0..matrix.num_cols, row..row+1)
    }

    pub fn reset(&mut self) {
        self.next_id = None;

        if self.cols.start > self.num_cols || self.rows.start > self.num_rows {
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
            if next_col >= self.cols.end || next_col >= self.num_cols {
                next_col = self.cols.start;
                next_row = row + 1;
            }

            self.curr_id = {
                if next_row >= self.rows.end || next_row >= self.num_rows {
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