// lib.rs

mod i32;
mod f64;

pub trait Trta<T> {
    fn sum(&self) -> T;
    fn ave(&self) -> T;

    fn window(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))

    fn window_under_zero(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((0) (0) (1 2 3) (2 3 4) (3 4 5))

    fn window_under(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))

    fn window_under_zero_value(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))
}
