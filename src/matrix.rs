struct Vertex {
    id: usize,
    active: bool,
    row: usize,
    col: usize,
    top_id: usize,
    left_id: usize,
    right_id: usize,
    bottom_id: usize
}

pub struct Matrix {
    vertices: Vec<Vertex>,
    active_cols: Vec<bool>,
    col_element_counts: Vec<usize>,
    edges_initialised: bool,
    num_rows: usize,
    num_cols: usize
}

impl Matrix {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        // instantiate the matrix
        let mut mat = Self {
            vertices: vec![],
            active_cols: vec![false; num_cols],
            col_element_counts: vec![0; num_cols],
            edges_initialised: false,
            num_rows,
            num_cols
        };

        // instantiate vertices
        for _ in 0..=num_rows {
            for _ in 0..=num_cols {
                let v = Vertex{id: 0,
                               active: false,
                               row: 0,
                               col: 0,
                               top_id: 0,
                               left_id: 0,
                               right_id: 0,
                               bottom_id: 0};
                mat.vertices.push(v);
            }
        }

        mat
    }

    fn row_vertex_id(&self, row: usize) -> usize {
        debug_assert!(row < self.num_rows,
                      "Row {} is out of bounds (num rows: {})",
                      row, self.num_rows);

        (row + 1) * (self.num_cols + 1)
    }

    fn col_vertex_id(&self, col: usize) -> usize {
        debug_assert!(col < self.num_cols,
                      "Col {} is out of bounds (num cols: {})",
                      col, self.num_cols);

        col + 1
    }

    fn element_vertex_id(&self, row: usize, col: usize) -> usize {
        debug_assert!(row < self.num_rows,
                      "Element row {} is out of bounds (num rows: {})",
                      row, self.num_rows);
        debug_assert!(col < self.num_cols,
                      "Element col {} is out of bounds (num cols: {})",
                      col, self.num_cols);

        (col + 1) + (row + 1) * (self.num_cols + 1)
    }

    pub fn set_element(&mut self, row: usize, col: usize) {
        let id = self.element_vertex_id(row, col);
        self.vertices[id].active = true;
        self.edges_initialised = false;
    }

    fn initialise_edges(&mut self) {
        // set the left-right/row info for each active vertex
        for row in 0..self.num_rows {
            let mut first: Option<usize> = None;
            let mut last: Option<usize> = None;

            for col in 0..self.num_cols {
                let elem_id = self.element_vertex_id(row, col);

                if self.vertices[elem_id].active {
                    self.vertices[elem_id].id = elem_id;
                    self.vertices[elem_id].row = row;

                    if let Some(last_id) = last {
                        self.vertices[elem_id].left_id = last_id;
                        self.vertices[last_id].right_id = elem_id;
                    } else {
                        first = Some(elem_id);
                    }
                    last = Some(elem_id);
                }
            }

            if let Some(first_id) = first {
                if let Some(last_id) = last {
                    // initialise and set the row vertex
                    let row_id = self.row_vertex_id(row);
                    self.vertices[row_id].left_id = last_id;
                    self.vertices[row_id].right_id = first_id;

                    // set left/right ids to the row vertex
                    // first and last vertex in the row
                    self.vertices[first_id].left_id = row_id;
                    self.vertices[last_id].right_id = row_id;
                }
            }
        }

        // set the top-bottom/col info for each active vertex
        for col in 0..self.num_cols {
            let mut first: Option<usize> = None;
            let mut last: Option<usize> = None;

            for row in 0..self.num_rows {
                let elem_id = self.element_vertex_id(row, col);

                if self.vertices[elem_id].active {
                    // set the col for the vertex
                    self.vertices[elem_id].col = col;

                    // increment the count of nodes in this column
                    self.col_element_counts[col] += 1;

                    // set top/right indices for each element
                    if let Some(last_id) = last {
                        self.vertices[elem_id].top_id = last_id;
                        self.vertices[last_id].bottom_id = elem_id;
                    } else {
                        first = Some(elem_id);
                    }
                    last = Some(elem_id);
                }
            }

            if let Some(first_id) = first {
                if let Some(last_id) = last {
                    // initialise and set the row vertex
                    let col_id = self.col_vertex_id(col);
                    self.active_cols[col] = true;
                    self.vertices[col_id].top_id = last_id;
                    self.vertices[col_id].bottom_id = first_id;

                    // set left/right ids to the col vertex
                    // first and last vertex in the col
                    self.vertices[first_id].top_id = col_id;
                    self.vertices[last_id].bottom_id = col_id;
                }
            }
        }

        // set the internal flag to avoid re-calling this
        // method every time the cover method is called
        self.edges_initialised = true;
    }

    pub fn find_solutions(&mut self, terminate_on_first: bool) -> Vec<Vec<usize>> {
        if !self.edges_initialised {
            self.initialise_edges();
        }
        let mut solutions: Vec<Vec<usize>> = Vec::new();
        let mut candidate: Vec<usize> = Vec::new();
        self.reduce_matrix(&mut candidate, &mut solutions, terminate_on_first);
        solutions
    }

    fn reduce_matrix(&mut self,
                     candidate: &mut Vec<usize>,
                     solutions: &mut Vec<Vec<usize>>,
                     terminate_on_first: bool) {

        // if we just want one solution, and we already have one, return
        if solutions.len() > 0 && terminate_on_first {
            return;
        }

        // get the column with the fewest remaining active rows
        let mut min = self.num_rows;
        let mut min_col: Option<usize> = None;
        for col in 0..self.num_cols {
            let count = self.col_element_counts[col];
            if count > 0 && count < min {
                min = count;
                min_col = Some(col);
            }
        }

        // if there is a column to explore, then explore it
        if let Some(col) = min_col {

            // work our way down the column
            let col_id = self.col_vertex_id(col);
            let mut elem_id = self.vertices[col_id].bottom_id;

            while elem_id != col_id {
                let row = self.vertices[elem_id].row;
                let (eliminated_vertices, eliminated_columns) = self.eliminate_row(row);
                candidate.push(row);

                self.reduce_matrix(candidate, solutions, terminate_on_first);

                candidate.pop();
                self.restore(eliminated_vertices, eliminated_columns);

                // update the elem_id for the next iteration
                elem_id = self.vertices[elem_id].bottom_id;
            }
        } else {
            // there were no columns. Is the matrix empty?
            // if not, return empty-handed --- this is a dead-end.
            for col in self.active_cols.iter() {
                if *col {
                    return;
                }
            }
            // the matrix is empty --- we have a solution!
            solutions.push(candidate.clone());
        }
    }

    pub fn eliminate_row(&mut self, row: usize) -> (Vec<usize>, Vec<usize>) {
        if !self.edges_initialised {
            self.initialise_edges();
        }

        let mut eliminated_vertices: Vec<usize> = Vec::new();
        let mut eliminated_columns: Vec<usize> = Vec::new();

        let row_id = self.row_vertex_id(row);
        let mut row_elem_id = self.vertices[row_id].right_id;

        while row_elem_id != row_id {
            let col = self.vertices[row_elem_id].col;
            let col_id = self.col_vertex_id(col);

            // cover the column
            eliminated_columns.push(col);
            self.active_cols[col] = false;

            // cover elements in the columns of this row
            let mut col_elem_id = self.vertices[col_id].bottom_id;
            while col_elem_id != col_id {
                self.cover_vertex(col_elem_id);
                eliminated_vertices.push(col_elem_id);

                // cover elements on the row intersecting this column element
                let intersect_row = self.vertices[col_elem_id].row;

                let intersect_row_id = self.row_vertex_id(intersect_row);
                let mut intersect_row_elem_id = self.vertices[intersect_row_id].right_id;

                while intersect_row_elem_id != intersect_row_id {
                    self.cover_vertex(intersect_row_elem_id);
                    eliminated_vertices.push(intersect_row_elem_id);
                    intersect_row_elem_id = self.vertices[intersect_row_elem_id].right_id;
                }
                col_elem_id = self.vertices[col_elem_id].bottom_id;
            }
            row_elem_id = self.vertices[row_elem_id].right_id;
        }

        (eliminated_vertices, eliminated_columns)
    }

    fn restore(&mut self, eliminated_vertices: Vec<usize>, eliminated_columns: Vec<usize>) {
        for vertex in eliminated_vertices {
            self.uncover_vertex(vertex);
        }
        for col in eliminated_columns {
            self.active_cols[col] = true;
        }
    }

    fn cover_vertex(&mut self, id: usize) {
        let top_id = self.vertices[id].top_id;
        let bottom_id = self.vertices[id].bottom_id;
        let left_id = self.vertices[id].left_id;
        let right_id = self.vertices[id].right_id;

        self.vertices[top_id].bottom_id = bottom_id;
        self.vertices[bottom_id].top_id = top_id;
        self.vertices[left_id].right_id = right_id;
        self.vertices[right_id].left_id = left_id;

        let col = self.vertices[id].col;
        self.col_element_counts[col] -= 1;
    }

    fn uncover_vertex(&mut self, id: usize) {
        let top_id = self.vertices[id].top_id;
        let bottom_id = self.vertices[id].bottom_id;
        let left_id = self.vertices[id].left_id;
        let right_id = self.vertices[id].right_id;

        self.vertices[top_id].bottom_id = id;
        self.vertices[bottom_id].top_id = id;
        self.vertices[left_id].right_id = id;
        self.vertices[right_id].left_id = id;

        let col = self.vertices[id].col;
        self.col_element_counts[col] += 1;
    }
}

mod tests {
    use super::Matrix;

    #[test]
    fn test_matrix_new() {
        let num_rows = 3;
        let num_cols = 3;
        let num_vertices = (num_rows + 1) * (num_cols + 1);
        let mat = Matrix::new(num_rows, num_cols);
        assert_eq!(num_vertices, mat.vertices.len());
    }

    #[test]
    fn test_matrix_element_vertex_id() {
        let mat = Matrix::new(3, 3);
        assert_eq!(5, mat.element_vertex_id(0, 0));
        assert_eq!(6, mat.element_vertex_id(0, 1));
        assert_eq!(7, mat.element_vertex_id(0, 2));
        assert_eq!(9, mat.element_vertex_id(1, 0));
        assert_eq!(10, mat.element_vertex_id(1, 1));
        assert_eq!(11, mat.element_vertex_id(1, 2));
        assert_eq!(13, mat.element_vertex_id(2, 0));
        assert_eq!(14, mat.element_vertex_id(2, 1));
        assert_eq!(15, mat.element_vertex_id(2, 2));
    }

    #[test]
    fn test_matrix_row_vertex_id() {
        let mat = Matrix::new(3, 3);
        assert_eq!(4, mat.row_vertex_id(0));
        assert_eq!(8, mat.row_vertex_id(1));
        assert_eq!(12, mat.row_vertex_id(2));
    }

    #[test]
    fn test_matrix_col_vertex_id() {
        let mat = Matrix::new(3, 3);
        assert_eq!(1, mat.col_vertex_id(0));
        assert_eq!(2, mat.col_vertex_id(1));
        assert_eq!(3, mat.col_vertex_id(2));
    }

    #[test]
    #[should_panic]
    fn test_matrix_element_vertex_id_panic_col() {
        let mat = Matrix::new(3, 3);
        mat.element_vertex_id(4, 0);
    }

    #[test]
    #[should_panic]
    fn test_matrix_element_vertex_id_panic_row() {
        let mat = Matrix::new(3, 3);
        mat.element_vertex_id(0, 4);
    }

    #[test]
    fn test_matrix_set_element() {
        let mut mat = Matrix::new(3, 3);
        mat.set_element(0, 1);
        assert_eq!(false, mat.vertices[mat.element_vertex_id(0, 0)].active);
        assert_eq!(true, mat.vertices[mat.element_vertex_id(0, 1)].active);
    }

    #[test]
    fn test_matrix_indices_one_element() {
        let mut mat = Matrix::new(6, 3);
        mat.set_element(3, 1);
        mat.initialise_edges();

        let col1 = mat.col_vertex_id(1);
        let row3 = mat.row_vertex_id(3);
        let elem_i3_j1 = mat.element_vertex_id(3, 1);

        assert_eq!(col1, mat.vertices[elem_i3_j1].top_id, "(3, 1) top");
        assert_eq!(col1, mat.vertices[elem_i3_j1].bottom_id, "(3, 1) bottom");

        assert_eq!(row3, mat.vertices[elem_i3_j1].left_id, "(3, 1) left");
        assert_eq!(row3, mat.vertices[elem_i3_j1].right_id, "(3, 1) right");

        assert_eq!(1, mat.col_element_counts[1]);
    }

    #[test]
    fn test_matrix_col_indices() {
        let mut mat = Matrix::new(6, 3);
        mat.set_element(1, 1);
        mat.set_element(3, 1);
        mat.set_element(5, 1);
        mat.initialise_edges();

        let col1 = mat.col_vertex_id(1);
        let elem_i1_j1 = mat.element_vertex_id(1, 1);
        let elem_i3_j1 = mat.element_vertex_id(3, 1);
        let elem_i5_j1 = mat.element_vertex_id(5, 1);

        // check col vertex
        assert_eq!(elem_i5_j1, mat.vertices[col1].top_id, "col top");
        assert_eq!(elem_i1_j1, mat.vertices[col1].bottom_id, "col bottom");

        // check (1, 1)
        assert_eq!(col1, mat.vertices[elem_i1_j1].top_id, "(1, 1) top");
        assert_eq!(elem_i3_j1, mat.vertices[elem_i1_j1].bottom_id, "(1, 1) bottom");

        // check (3, 1)
        assert_eq!(elem_i1_j1, mat.vertices[elem_i3_j1].top_id, "(3, 1) top");
        assert_eq!(elem_i5_j1, mat.vertices[elem_i3_j1].bottom_id, "(3, 1) bottom");

        // check (5, 1)
        assert_eq!(elem_i3_j1, mat.vertices[elem_i5_j1].top_id, "(5, 1) top");
        assert_eq!(col1, mat.vertices[elem_i5_j1].bottom_id, "(5, 1) bottom");

        assert_eq!(3, mat.col_element_counts[1]);
    }

    #[test]
    fn test_matrix_row_indices() {
        let mut mat = Matrix::new(3, 6);
        mat.set_element(1, 1);
        mat.set_element(1, 3);
        mat.set_element(1, 5);
        mat.initialise_edges();

        let row1 = mat.row_vertex_id(1);
        let elem_i1_j1 = mat.element_vertex_id(1, 1);
        let elem_i1_j3 = mat.element_vertex_id(1, 3);
        let elem_i1_j5 = mat.element_vertex_id(1, 5);

        // check row vertex
        assert_eq!(elem_i1_j5, mat.vertices[row1].left_id, "row left");
        assert_eq!(elem_i1_j1, mat.vertices[row1].right_id, "row right");

        // check (1, 1)
        assert_eq!(row1, mat.vertices[elem_i1_j1].left_id, "(1, 1) left");
        assert_eq!(elem_i1_j3, mat.vertices[elem_i1_j1].right_id, "(1, 1) right");

        // check (3, 1)
        assert_eq!(elem_i1_j1, mat.vertices[elem_i1_j3].left_id, "(1, 3) left");
        assert_eq!(elem_i1_j5, mat.vertices[elem_i1_j3].right_id, "(1, 3) right");

        // check (5, 1)
        assert_eq!(elem_i1_j3, mat.vertices[elem_i1_j5].left_id, "(1, 5) left");
        assert_eq!(row1, mat.vertices[elem_i1_j5].right_id, "(1, 5) right");
    }

    #[test]
    fn test_matrix_cover_elements() {
        let mut mat = Matrix::new(6, 2);

        mat.set_element(1, 1);
        mat.set_element(3, 1);
        mat.set_element(5, 1);
        mat.initialise_edges();

        let col1 = mat.col_vertex_id(1);
        let elem_i1_j1 = mat.element_vertex_id(1, 1);
        let elem_i3_j1 = mat.element_vertex_id(3, 1);
        let elem_i5_j1 = mat.element_vertex_id(5, 1);

        mat.cover_vertex(elem_i1_j1);

        assert_eq!(2, mat.col_element_counts[1]);
        assert_eq!(elem_i3_j1, mat.vertices[col1].bottom_id);
        assert_eq!(col1, mat.vertices[elem_i3_j1].top_id);

        mat.cover_vertex(elem_i3_j1);

        assert_eq!(1, mat.col_element_counts[1]);
        assert_eq!(col1, mat.vertices[elem_i5_j1].bottom_id);
        assert_eq!(col1, mat.vertices[elem_i5_j1].top_id);

        mat.cover_vertex(elem_i5_j1);

        assert_eq!(0, mat.col_element_counts[1]);
    }

    #[test]
    fn test_matrix_uncover_elements() {
        let mut mat = Matrix::new(6, 2);

        mat.set_element(1, 1);
        mat.set_element(3, 1);
        mat.set_element(5, 1);
        mat.initialise_edges();

        let col1 = mat.col_vertex_id(1);
        let elem_i1_j1 = mat.element_vertex_id(1, 1);
        let elem_i3_j1 = mat.element_vertex_id(3, 1);
        let elem_i5_j1 = mat.element_vertex_id(5, 1);

        mat.cover_vertex(elem_i1_j1);
        mat.cover_vertex(elem_i3_j1);
        mat.cover_vertex(elem_i5_j1);

        mat.uncover_vertex(elem_i5_j1);

        assert_eq!(1, mat.col_element_counts[1]);
        assert_eq!(col1, mat.vertices[elem_i5_j1].bottom_id);
        assert_eq!(col1, mat.vertices[elem_i5_j1].top_id);

        mat.uncover_vertex(elem_i3_j1);

        assert_eq!(2, mat.col_element_counts[1]);
        assert_eq!(elem_i3_j1, mat.vertices[col1].bottom_id);
        assert_eq!(col1, mat.vertices[elem_i3_j1].top_id);

        mat.uncover_vertex(elem_i5_j1);
    }

    // TODO: more tests
    // fn example_matrix() -> Matrix {
    //     let mut mat = Matrix::new(9, 6);
    //     mat.set_element(0, 0);
    //     mat.set_element(0, 3);
    //     mat.set_element(1, 1);
    //     mat.set_element(1, 3);
    //     mat.set_element(2, 2);
    //     mat.set_element(2, 3);
    //     mat.set_element(3, 0);
    //     mat.set_element(3, 4);
    //     mat.set_element(4, 1);
    //     mat.set_element(4, 4);
    //     mat.set_element(5, 2);
    //     mat.set_element(5, 4);
    //     mat.set_element(6, 0);
    //     mat.set_element(6, 5);
    //     mat.set_element(7, 1);
    //     mat.set_element(7, 5);
    //     mat.set_element(8, 2);
    //     mat.set_element(8, 5);
    //     mat
    // }
}