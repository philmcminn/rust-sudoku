pub struct Table<T> {
    elements: Vec<Option<T>>,
    num_rows: usize,
    num_cols: usize,
    num_elements: usize,
    num_set_elements: usize
}

impl<T> Table<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Table<T> {
        let num_elements = num_rows * num_cols;
        let elements = std::iter::repeat_with(|| None).take(num_elements).collect();
        let num_set_elements = 0;
        let mat = Table {
            elements,
            num_rows,
            num_cols,
            num_elements,
            num_set_elements
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

    pub fn element(&self, row: usize, col: usize) -> &Option<T> {
        let index = self.element_index(row, col);
        &self.elements[index]
    }

    pub fn set_element(&mut self, row: usize, col: usize, val: T) {
        let index = self.element_index(row, col);
        if let None = self.elements[index] {
            self.num_set_elements += 1;
        }
        self.elements[index] = Some(val)
    }

    pub fn unset_element(&mut self, row: usize, col: usize) {
        let index = self.element_index(row, col);
        if let Some(_) = self.elements[index] {
            self.num_set_elements -= 1;
        }
        self.elements[index] = None;
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_set_elements(&self) -> usize {
        self.num_set_elements
    }

    pub fn is_full(&self) -> bool {
        self.num_set_elements == self.num_elements
    }
}

//mod tests {
//}