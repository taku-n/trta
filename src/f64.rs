// f64.rs

use crate::Trta;

impl Trta<f64> for [f64] {
    fn sum(&self) -> f64 {
        self.iter().sum()
    }

    fn ave(&self) -> f64 {
        self.sum() / self.len() as f64
    }

    fn window(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v: Vec<Vec<f64>> = Vec::new();

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under_zero(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v = vec![vec![0.0]; period - 1];

        v.extend(self.window(period));

        v
    }

    fn window_under(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v: Vec<Vec<f64>> = Vec::new();

        for i in 1..period {
            v.push(self.to_vec().into_iter().take(i).collect::<Vec<_>>());
        }
        // create vectors shorter than period

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under_zero_value(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v = vec![0.0; period - 1];

        v.extend(self);

        v.window(period)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!([1.0, 2.0, 3.0].sum(), 6.0);      // array  of f64
        assert_eq!((&[1.0, 2.0, 3.0]).sum(), 6.0);   // slice  of f64
        assert_eq!(vec![1.0, 2.0, 3.0].sum(), 6.0);  // vector of f64
    }

    #[test]
    fn test_ave() {
        assert_eq!([1.0, 2.0, 3.0].ave(), 2.0);
        assert_eq!((&[1.0, 2.0, 3.0]).ave(), 2.0);
        assert_eq!(vec![1.0, 2.0, 3.0].ave(), 2.0);
    }

    #[test]
    fn test_window() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_window_under_zero() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero(3),
                [vec![0.0], vec![0.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window_under_zero(3),
                [vec![0.0], vec![0.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero(3),
                [vec![0.0], vec![0.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_window_under() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window_under(3),
                [vec![1.0], vec![1.0, 2.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window_under(3),
                [vec![1.0], vec![1.0, 2.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window_under(3),
                [vec![1.0], vec![1.0, 2.0],
                vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_window_under_zero_value() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero_value(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window_under_zero_value(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero_value(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }
}
