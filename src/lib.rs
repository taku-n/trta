mod imp;
pub mod trta;

pub trait TrTA<T> {
    fn sum(&self) -> T;
    fn ave(&self) -> T;

    fn window(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))

    fn window_under_zero(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((0) (0) (1 2 3) (2 3 4) (3 4 5))

    fn window_under(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))

    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))

    fn window_under_head_with(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1 1 1) (1 1 2) (1 2 3) (2 3 4) (3 4 5))

    fn sma(&self, period: usize) -> Vec<T>;
    fn sma_calced(&self, period: usize, begin: usize, latest: &[T]) -> Vec<T>;
    // "latest" is the latest calculated SMA values to omit recalculations.
}
