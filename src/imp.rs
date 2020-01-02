use crate::TrTA;

impl TrTA for [f64] {
    fn sum(&self) -> f64 {
        self.iter().sum()
    }

    fn avg(&self) -> f64 {
        self.sum() / self.len() as f64
    }

    fn window(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v: Vec<Vec<f64>> = Vec::new();

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v: Vec<Vec<f64>> = Vec::new();

        // create vectors shorter than period
        for i in 1..period {
            v.push(self.to_vec().into_iter().take(i).collect::<Vec<_>>());
        }

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


    fn window_under_zero_with(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v = vec![0.0; period - 1];

        v.extend(self);

        v.window(period)
    }

    fn window_under_head_with(&self, period: usize) -> Vec<Vec<f64>> {
        let mut v = vec![self[0]; period - 1];

        v.extend(self);

        v.window(period)
    }

    fn sma(&self, period: usize) -> Vec<f64> {
        self.sma_re(period, &[])
    }

    fn sma_re(&self, period: usize, latest: &[f64]) -> Vec<f64> {
        let mut v = Vec::with_capacity(self.len());

        if latest.len() == 0 {
            v = self.window_under_head_with(period)
                    .into_iter().map(|xs| xs.avg()).collect::<Vec<_>>();
        } else {
            v.extend_from_slice(&latest[0 .. latest.len() - 1]);
            v.extend_from_slice(&(self.window_under_head_with(period))[latest.len() - 1 ..]
                    .into_iter().map(|xs| xs.avg()).collect::<Vec<_>>());
        }

        v
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
    fn test_avg() {
        assert_eq!([1.0, 2.0, 3.0].avg(), 2.0);
        assert_eq!((&[1.0, 2.0, 3.0]).avg(), 2.0);
        assert_eq!(vec![1.0, 2.0, 3.0].avg(), 2.0);
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
    fn test_window_under_zero_with() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero_with(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window_under_zero_with(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window_under_zero_with(3),
                [[0.0, 0.0, 1.0], [0.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_window_under_head_with() {
        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window_under_head_with(3),
                [[1.0, 1.0, 1.0], [1.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window_under_head_with(3),
                [[1.0, 1.0, 1.0], [1.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window_under_head_with(3),
                [[1.0, 1.0, 1.0], [1.0, 1.0, 2.0],
                [1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_sma() {
        assert_eq!([1.0, 4.0, 4.0, 4.0, 7.0].sma(3), [1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_sma_re() {
        assert_eq!([1.0, 4.0, 4.0, 4.0, 7.0].sma_re(3, &[1.0, 2.0, 3.0, 4.0, 5.0]),
                [1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!([1.0, 4.0, 4.0, 4.0, 7.0].sma_re(3, &[2.0, 3.0, 5.0, 7.0, 11.0]),
                [2.0, 3.0, 5.0, 7.0, 5.0]);
        assert_eq!([1.0, 4.0, 4.0, 4.0, 7.0].sma_re(3, &[2.0, 3.0, 5.0]),
                [2.0, 3.0, 3.0, 4.0, 5.0]);
    }
}
