extern crate nalgebra;
extern crate log;

use log::{error, info};
use nalgebra::{DMatrix};

#[path = "../src/ops_nalgebra.rs"]
mod ops_nalgebra;

pub fn test_concat_col() {
    let mat_1 = DMatrix::<f64>::from_column_slice(4, 1, &[5., 5., 5., 5.]);
    let mat_2 = DMatrix::<f64>::from_column_slice(4, 2, &[10., 66., 66., 10., 11., 11., 11., 11.]);

    let mat_expected: DMatrix::<f64> = DMatrix::from_column_slice(
        4, 3, &[5., 5., 5., 5., 10., 66., 66., 10., 11., 11., 11., 11.],
    );
    let mat_concat = ops_nalgebra::concat(&mat_1, &mat_2, &1);

    match mat_concat {
        Ok(mat_concat) => {
            info!("Matrix concatenation along COL(1) axis: {}", mat_concat);
            assert_eq!(mat_expected, mat_concat);
        }
        Err(e) => error!("An error occurred: {}", e),
    }
}

pub fn test_concat_row() {
    let mat_1 = DMatrix::<f64>::from_column_slice(
        4, 2, &[5., 5., 5., 5., 6., 6., 6., 6.],
    );
    let mat_2 = DMatrix::<f64>::from_column_slice(
        4, 2, &[10., 66., 66., 10., 11., 11., 11., 11.],
    );
    let mat_expected: DMatrix::<f64> = DMatrix::from_column_slice(
        8, 2, &[
            5., 5., 5., 5., 10., 66., 66., 10.,
            6., 6., 6., 6., 11., 11., 11., 11.
        ],
    );
    let mat_concat = ops_nalgebra::concat(&mat_1, &mat_2, &0);

    match mat_concat {
        Ok(mat_concat) => {
            info!("Matrix concatenation along ROW(0) axis: {}", mat_concat);
            assert_eq!(mat_expected, mat_concat);
        }
        Err(e) => error!("An error occurred: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use test_concat_row;
    use test_concat_col;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            std::env::set_var("RUST_LOG", "info");
            env_logger::init();
        });
    }

    #[test]
    fn run_test_concat_col() {
        setup();
        test_concat_col();
    }

    #[test]
    fn run_test_concat_row() {
        setup();
        test_concat_row();
    }
}