mod i32;

pub trait I32 {
    fn sum(&self) -> i32;
    fn ave(&self) -> i32;

    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))
    fn window(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((0) (0) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((1 1 1) (1 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_head_with(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<i32>>;

    fn sma(&self, period: usize) -> Vec<i32>;

    // "latest" is the latest calculated SMA values to omit recalculations.
    fn sma_calced(&self, period: usize, begin: usize, latest: &[i32]) -> Vec<i32>;
}
