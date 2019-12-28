mod i32;

pub trait I32 {
    fn sum(&self) -> i32;
    fn avg(&self) -> i32;

    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))
    fn window(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((1) (1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((0) (0) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((0 0 1) (0 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<i32>>;

    // (1 2 3 4 5) => ((1 1 1) (1 1 2) (1 2 3) (2 3 4) (3 4 5))
    fn window_under_head_with(&self, period: usize) -> Vec<Vec<i32>>;
}
