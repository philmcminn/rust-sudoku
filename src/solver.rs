use super::dlx::DLX;
use super::matrix::Matrix;
use super::Sudoku;

pub fn solve(sudoku: &Sudoku, terminate_on_first: bool) { // -> Vec<Sudoku> {
    // initialise the matrix
    let (num_rows, num_cols) = matrix_dimensions(sudoku);
    let mut matrix = Matrix::new(false, num_rows, num_cols);

    // populate the matrix and eliminate any rows corresponding
    // to completed positions in the initial provided Sudoku
    populate_matrix(&mut matrix, sudoku);
    // eliminate_rows_for_completed_cells(&mut dlx, sudoku);

    // find the solution(s)
    // let solutions = mat.find_solutions(terminate_on_first);

    // convert the solutions into completed Sudoku(s)
    // let mut completed_sudokus = Vec::new();
    // for solution in solutions {
    //     let completed_solution = complete_sudoku(&sudoku, &solution);
    //     completed_sudokus.push(completed_solution);
    // }

    // return the result
    // completed_sudokus
}

fn matrix_dimensions(sudoku: &Sudoku) -> (usize, usize) {
    let num_rows = sudoku.dimension().pow(3);
    let num_cols = 4 * sudoku.dimension().pow(2);
    (num_rows, num_cols)
}

fn populate_matrix(matrix: &mut Matrix<bool>, sudoku: &Sudoku) {
    let region_width = sudoku.dimension().pow(2);

    for row in 0..sudoku.dimension() {
        for col in 0..sudoku.dimension() {
            for val in 1..=sudoku.dimension() {
                // current row in the matrix
                let mat_row = matrix_row_for_cell_value(sudoku, row, col, val);

                // cells
                let mat_col = col + row * sudoku.dimension();
                matrix.set_element(mat_row, mat_col, true);

                // rows
                let mat_col = region_width +
                              (row * sudoku.dimension()) + (val - 1);
                matrix.set_element(mat_row, mat_col, true);

                // cols
                let mat_col = (region_width * 2) +
                              (col * sudoku.dimension()) + (val - 1);
                matrix.set_element(mat_row, mat_col, true);

                // blocks
                let mat_col = (region_width * 3) +
                              (sudoku.block_no(row, col) * sudoku.dimension()) +
                              (val - 1);
                matrix.set_element(mat_row, mat_col, true);
            }
        }
    }
}

fn eliminate_rows_for_completed_cells(dlx: &mut DLX, sudoku: &Sudoku) {
    for (row, col, val) in sudoku.completed_cells() {
        let mat_row = matrix_row_for_cell_value(sudoku, row, col, val);
        dlx.eliminate_row(mat_row);
    }
}

// take a row, col, and value of a Sudoku cell and find the corresponding matrix row number
fn matrix_row_for_cell_value(sudoku: &Sudoku, row: usize, col: usize, val: usize) -> usize {
    (val - 1) + (sudoku.dimension() * (col + row * sudoku.dimension()))
}

// get the row, col and value of a Sudoku cell corresponding to a matrix row number
// (the reverse calculation of matrix_row_for_cell_value)
fn cell_value_for_matrix_row(sudoku: &Sudoku, mat_row: usize) -> (usize, usize, usize) {
    let row = mat_row / sudoku.dimension().pow(2);
    let col = (mat_row / sudoku.dimension()) % sudoku.dimension();
    let val = (mat_row % sudoku.dimension()) + 1;

    (row, col, val)
}

// complete a sudoku given a solution returned by the matrix
fn complete_sudoku(sudoku: &Sudoku, solution: &Vec<usize>) -> Sudoku {
    let mut sudoku = sudoku.clone();
    for soln_row in solution {
        let (row, col, val) = cell_value_for_matrix_row(&sudoku, *soln_row);
        sudoku.set_cell_value(row, col, val);
    }
    sudoku
}

mod tests {
    use super::Sudoku;
    use super::DLX;
    use super::super::solver; // TODO: is there a better way to state this

    #[test]
    fn test_cell_value_for_matrix_row() {
        let sud = &Sudoku::new(4);

        assert_eq!((0, 0, 1), solver::cell_value_for_matrix_row(sud, 0), "row 0");
        assert_eq!((0, 2, 3), solver::cell_value_for_matrix_row(sud, 10), "row 10");
        assert_eq!((1, 1, 2), solver::cell_value_for_matrix_row(sud, 21), "row 21");
        assert_eq!((2, 0, 3), solver::cell_value_for_matrix_row(sud, 34), "row 34");
        assert_eq!((3, 1, 4), solver::cell_value_for_matrix_row(sud, 55), "row 55");
        assert_eq!((3, 3, 4), solver::cell_value_for_matrix_row(sud, 63), "row 63");
    }

    #[test]
    fn test_matrix_row_for_cell_value() {
        let sud = &Sudoku::new(4);

        assert_eq!(0,  solver::matrix_row_for_cell_value(sud, 0, 0, 1), "row 0");
        assert_eq!(10, solver::matrix_row_for_cell_value(sud, 0, 2, 3), "row 10");
        assert_eq!(21, solver::matrix_row_for_cell_value(sud, 1, 1, 2), "row 21");
        assert_eq!(34, solver::matrix_row_for_cell_value(sud, 2, 0, 3), "row 34");
        assert_eq!(55, solver::matrix_row_for_cell_value(sud, 3, 1, 4), "row 55");
        assert_eq!(63, solver::matrix_row_for_cell_value(sud, 3, 3, 4), "row 63");
    }
}