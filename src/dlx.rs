use super::matrix::Matrix;

struct Vertex {
    id: usize,
    row: usize,
    col: usize,
    top_id: usize,
    left_id: usize,
    right_id: usize,
    bottom_id: usize
}


pub struct DLX {
    vertices: Vec<Vertex>,
}

impl DLX {
    pub fn new(matrix: Matrix<bool>) -> Self {
        let mut vertices = Vec::new();

        // set the left-right/row info for each active vertex
        // for row in 0..matrix.num_rows() {
        //     let mut first: Option<usize> = None;
        //     let mut last: Option<usize> = None;
        //
        //     for col in 0..matrix.num_cols() {
        //         let elem_id = vertices.len();
        //
        //         if self.vertices[elem_id].active {
        //             self.vertices[elem_id].id = elem_id;
        //             self.vertices[elem_id].row = row;
        //
        //             if let Some(last_id) = last {
        //                 self.vertices[elem_id].left_id = last_id;
        //                 self.vertices[last_id].right_id = elem_id;
        //             } else {
        //                 first = Some(elem_id);
        //             }
        //             last = Some(elem_id);
        //         }
        //     }
        //
        //     if let Some(first_id) = first {
        //         if let Some(last_id) = last {
        //             // initialise and set the row vertex
        //             let row_id = self.row_vertex_id(row);
        //             self.vertices[row_id].left_id = last_id;
        //             self.vertices[row_id].right_id = first_id;
        //
        //             // set left/right ids to the row vertex
        //             // first and last vertex in the row
        //             self.vertices[first_id].left_id = row_id;
        //             self.vertices[last_id].right_id = row_id;
        //         }
        //     }
        // }

        DLX {
            vertices
        }
    }
}