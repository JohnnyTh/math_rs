extern crate nalgebra;

use std::fmt;
use nalgebra::{DMatrix};

#[derive(Debug)]
pub enum ValidationError {
    DimensionMismatch,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::DimensionMismatch =>
                write!(f, "First dimensions of data matrix and labels matrix do not match.")
        }
    }
}

pub struct LinearRegression {
    n_threads: u8,
    slope: DMatrix<f64>,
    intercept: DMatrix<f64>,
}

impl LinearRegression {
    pub fn fit(&mut self, data: DMatrix<f64>, labels: DMatrix<f64>) ->
    Result<(), ValidationError> {
        if data.nrows() != labels.nrows() {
            return Err(ValidationError::DimensionMismatch);
        }
        //let data_ = na::DMatrix::<f64>::from_element(data.nrows(), 1, 1.0);

        //  ndarray::concatenate!(
        //     Axis(1),
        //     Array::<f64, Ix2>::ones((data.shape()[0], 1)),
        //     data
        // );
        // let xtx = data_.t().dot(&data_);
        // let xy = data_.t().dot(&labels);

        Ok(())
    }
}
