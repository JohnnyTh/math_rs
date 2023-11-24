use std::fmt;
use std::error::Error;
use nalgebra::{DMatrix, linalg::try_invert_to};

use crate::ops_nalgebra::concat;

#[derive(Debug, Clone)]
pub enum AlgorithmError {
    DimensionMismatch,
    MatrixInversionError,
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmError::DimensionMismatch =>
                write!(f, "First dimensions of data matrix and labels matrix do not match."),
            AlgorithmError::MatrixInversionError =>
                write!(f, "Could not ")
        }
    }
}

impl Error for AlgorithmError {}

impl From<Box<dyn Error>> for AlgorithmError {
    fn from(_: Box<dyn Error>) -> Self {
        AlgorithmError::DimensionMismatch
    }
}

pub struct LinearRegression {
    n_threads: u8,
    slope: Option<DMatrix<f64>>,
    intercept: Option<DMatrix<f64>>,
}

impl LinearRegression {
    pub fn new(n_threads: u8) -> LinearRegression {
        LinearRegression {
            n_threads,
            slope: None,
            intercept: None,
        }
    }

    pub fn fit(&mut self, data: &DMatrix<f64>, labels: &DMatrix<f64>) ->
    Result<(), AlgorithmError> {
        if data.nrows() != labels.nrows() {
            return Err(AlgorithmError::DimensionMismatch);
        }

        let data_w_bias = concat(&DMatrix::<f64>::zeros(data.nrows(), 1), data, &1)?;
        let mut xtxi = DMatrix::<f64>::zeros(data_w_bias.nrows(), data_w_bias.ncols());

        let xtx = data_w_bias.transpose() * &data_w_bias;
        let xy = data_w_bias.transpose() * labels;

        let inv_succ = try_invert_to(xtx, &mut xtxi);
        if !inv_succ {
            return Err(AlgorithmError::MatrixInversionError);
        }
        let params = xtxi * xy;

        self.intercept = Some(params.view((0, 0), (labels.nrows(), 1)).clone_owned());
        self.slope = Some(params.view((0, 1), (params.nrows(), params.ncols())).clone_owned());

        Ok(())
    }
}
