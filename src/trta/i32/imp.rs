use crate::trta::I32;

impl I32 for [i32] {
    fn sum(&self) -> i32 {
        self.iter().sum()
    }

    fn ave(&self) -> i32 {
        self.sum() / self.len() as i32
    }

    fn window(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();

        // create vectors shorter than period
        for i in 1..period {
            v.push(self.to_vec().into_iter().take(i).collect::<Vec<_>>());
        }

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under_zero(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v = vec![vec![0]; period - 1];

        v.extend(self.window(period));

        v
    }

    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v = vec![0; period - 1];  // std::vec::Vec<{integer}>

        v.extend(self);

        v.window(period)
    }

    fn window_under_head_with(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v = vec![self[0]; period - 1];

        v.extend(self);

        v.window(period)
    }

    fn sma(&self, period: usize) -> Vec<i32> {
        vec![0]
    }

    fn sma_calced(&self, period: usize, begin: usize, latest: &[i32]) -> Vec<i32> {
        vec![0]
    }
}
