extern crate criterion;
extern crate nalgebra;
extern crate rand;
extern crate log;

use log::{info};
use nalgebra::{DMatrix};
use criterion::{Criterion, criterion_group, criterion_main};
use rand::{Rng};
use std::time::Instant;

#[path = "../src/ops_nalgebra.rs"]
mod ops_nalgebra;

pub fn test_performance_nalgebra_concat(c: &mut Criterion) {
    c.bench_function("concat large matrices", |b| b.iter(|| {
        let n_elements = 1000;
        let mut rng = rand::thread_rng();
        let data1: Vec<f64> = (0..n_elements * n_elements).map(|_| rng.gen()).collect();
        let data2: Vec<f64> = (0..n_elements * n_elements).map(|_| rng.gen()).collect();

        let mat1 = DMatrix::<f64>::from_vec(n_elements, n_elements, data1);
        let mat2 = DMatrix::<f64>::from_vec(n_elements, n_elements, data2);

        let start = Instant::now();
        let _ = ops_nalgebra::concat(&mat1, &mat2, &0);
        info!(
            "Concatenated mat_1 [{}x{}] and mat_2 [{}x{}] Time elapsed: {}ms",
            mat1.nrows(),
            mat2.ncols(),
            mat2.nrows(),
            mat2.ncols(),
            start.elapsed().as_millis()
        );
        // add memory profiling here
    }));
}

criterion_group!(benches, test_performance_nalgebra_concat);
criterion_main!(benches);