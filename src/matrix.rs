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