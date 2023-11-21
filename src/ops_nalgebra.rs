extern crate nalgebra;
extern crate num_traits;

use std::error::Error;
use nalgebra::{DMatrix, Scalar};


pub fn concat<T: Scalar + Clone + num_traits::Zero>(
    matrix_1: DMatrix<T>, matrix_2: DMatrix<T>, axis: u8,
) -> Result<(DMatrix<T>), Box<dyn Error>> {
    let n_rows: usize;
    let n_cols: usize;

    if axis == 0 {
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
    }
    let mut matrix_concat: DMatrix::<T> = DMatrix::from_fn(n_rows, n_cols, |i, j| T::zero());
    for (idx_row, row) in matrix_1.row_iter().enumerate() {
        for (idx_col, elem) in row.row_iter().enumerate() {
            matrix_concat[(idx_row, idx_col)] = elem[(0, 0)].clone();
        }
    }
    return Ok(matrix_concat);
}

