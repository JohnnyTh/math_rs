extern crate nalgebra;

use nalgebra::{DMatrix};

#[path = "../src/ops_nalgebra.rs"]
mod ops_nalgebra;

pub fn test_concat() {
    let axis = 1;
    let mat_1 = DMatrix::<f64>::from_row_slice(3, 1, &[5., 5., 5.]);
    let mat_2 = DMatrix::<f64>::from_row_slice(3, 2, &[10., 10., 10., 11., 11., 11.]);

    let mat_concat = ops_nalgebra::concat(mat_1, mat_2, axis);
    match mat_concat {
        Ok(mat_concat) => println!("{}", mat_concat),
        Err(e) => println!("An error occurred: {}", e),
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::test_concat;

    #[test]
    fn run_test_concat() {
        test_concat();
    }
}