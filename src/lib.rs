mod imp;
pub mod trta;

pub trait TrTA {
    fn sum(&self) -> f64;
    fn avg(&self) -> f64;

    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))
    fn window(&self, period: usize) -> Vec<Vec<f64>>;

    // (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under(&self, period: usize) -> Vec<Vec<f64>>;

    // (1 2 3 4 5) => ((0) (0) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero(&self, period: usize) -> Vec<Vec<f64>>;

    // (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<f64>>;

    // (1 2 3 4 5) => ((1 1 1) (1 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_head_with(&self, period: usize) -> Vec<Vec<f64>>;

    fn sma(&self, period: usize) -> Vec<f64>;

    // "latest" is the latest calculated SMA values to omit recalculation.
    fn sma_re(&self, period: usize, latest: &[f64]) -> Vec<f64>;
}
