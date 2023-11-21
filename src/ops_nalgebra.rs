extern crate nalgebra;
extern crate num_traits;

use std::error::Error;
use nalgebra::{DMatrix, Scalar};


pub fn concat<T: Scalar + Clone + num_traits::Zero>(
    matrix_1: &DMatrix<T>, matrix_2: &DMatrix<T>, axis: &u8,
) -> Result<DMatrix<T>, Box<dyn Error>> {
    let n_rows: usize;
    let n_cols: usize;
    let idx_insert_row: usize;
    let idx_insert_col: usize;

    if *axis == 0 {
        // row axis
        if matrix_1.ncols() != matrix_2.ncols() {
            let msg = format!(
                "Cannot concatenate two matrices along {} \
                with != number of columns", axis.to_string()
            );
            return Err(msg.into());
        }
        n_rows = matrix_1.nrows() + matrix_2.nrows();
        n_cols = matrix_1.ncols();

        idx_insert_row = matrix_1.nrows();
        idx_insert_col = 0;
    } else {
        // col axis
        if matrix_1.nrows() != matrix_2.nrows() {
            let msg = format!(
                "Cannot concatenate two matrices along {} \
                with != number of rows", axis.to_string()
            );
            return Err(msg.into());
        }
        n_rows = matrix_1.nrows();
        n_cols = matrix_1.ncols() + matrix_2.ncols();

        idx_insert_row = 0;
        idx_insert_col = matrix_1.ncols();
    }
    let mut matrix_concat: DMatrix::<T> = DMatrix::from_fn(n_rows, n_cols, |_i, _j| T::zero());

    matrix_concat.view_mut((0, 0), (matrix_1.nrows(), matrix_1.ncols())).copy_from(matrix_1);
    matrix_concat.view_mut(
        (idx_insert_row, idx_insert_col),
        (matrix_2.nrows(), matrix_2.ncols()),
    ).copy_from(matrix_2);

    return Ok(matrix_concat);
}

