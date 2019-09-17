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

impl Trta<i32> for [i32] {
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

    fn window_under_zero(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v = vec![vec![0]; period - 1];

        v.extend(self.window(period));

        v
    }

    fn window_under(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();

        for i in 1..period {
            v.push(self.to_vec().into_iter().take(i).collect::<Vec<_>>());
        }
        // create vectors shorter than period

        for i in 0..=(self.len() - period) {
            v.push(self.to_vec().into_iter().skip(i).take(period).collect::<Vec<_>>());
        }

        v
    }

    fn window_under_zero_value(&self, period: usize) -> Vec<Vec<i32>> {
        let mut v = vec![0; period - 1];

        v.extend(self);

        v.window(period)
    }
}

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
        assert_eq!([1, 2, 3].sum(), 6);       // array  of i32
        assert_eq!((&[1, 2, 3]).sum(), 6);    // slice  of i32
        assert_eq!(vec![1, 2, 3].sum() , 6);  // vector of i32

        assert_eq!([1.0, 2.0, 3.0].sum(), 6.0);      // array  of f64
        assert_eq!((&[1.0, 2.0, 3.0]).sum(), 6.0);   // slice  of f64
        assert_eq!(vec![1.0, 2.0, 3.0].sum(), 6.0);  // vector of f64
    }

    #[test]
    fn test_ave() {
        assert_eq!([1, 2, 3].ave(), 2);
        assert_eq!((&[1, 2, 3]).ave(), 2);
        assert_eq!(vec![1, 2, 3].ave() , 2);

        assert_eq!([1.0, 2.0, 3.0].ave(), 2.0);
        assert_eq!((&[1.0, 2.0, 3.0]).ave(), 2.0);
        assert_eq!(vec![1.0, 2.0, 3.0].ave(), 2.0);
    }

    #[test]
    fn test_window() {
        assert_eq!([1, 2, 3, 4, 5].window(3), [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window(3), [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window(3), [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);

        assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0].window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!((&[1.0, 2.0, 3.0, 4.0, 5.0]).window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
        assert_eq!(vec![1.0, 2.0, 3.0, 4.0, 5.0].window(3),
                [[1.0, 2.0, 3.0], [2.0, 3.0, 4.0], [3.0, 4.0, 5.0]]);
    }

    #[test]
    fn test_window_under_zero() {
        assert_eq!([1, 2, 3, 4, 5].window_under_zero(3),
                [vec![0], vec![0], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under_zero(3),
                [vec![0], vec![0], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under_zero(3),
                [vec![0], vec![0], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);

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
        assert_eq!([1, 2, 3, 4, 5].window_under(3),
                [vec![1], vec![1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under(3),
                [vec![1], vec![1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under(3),
                [vec![1], vec![1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);

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
        assert_eq!([1, 2, 3, 4, 5].window_under_zero_value(3),
                [[0, 0, 1], [0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under_zero_value(3),
                [[0, 0, 1], [0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under_zero_value(3),
                [[0, 0, 1], [0, 1, 2], [1, 2, 3], [2, 3, 4], [3, 4, 5]]);

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
