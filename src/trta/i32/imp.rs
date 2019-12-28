use crate::trta::I32;

impl I32 for [i32] {
    fn sum(&self) -> i32 {
        self.iter().sum()
    }

    fn avg(&self) -> i32 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!([1, 2, 3].sum(), 6);      // array  of f64
        assert_eq!((&[1, 2, 3]).sum(), 6);   // slice  of f64
        assert_eq!(vec![1, 2, 3].sum(), 6);  // vector of f64
    }

    #[test]
    fn test_avg() {
        assert_eq!([1, 2, 3].avg(), 2);
        assert_eq!((&[1, 2, 3]).avg(), 2);
        assert_eq!(vec![1, 2, 3].avg(), 2);
    }

    #[test]
    fn test_window() {
        assert_eq!([1, 2, 3, 4, 5].window(3),
                [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window(3),
                [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window(3),
                [[1, 2, 3], [2, 3, 4], [3, 4, 5]]);
    }

    #[test]
    fn test_window_under() {
        assert_eq!([1, 2, 3, 4, 5].window_under(3),
                [vec![1], vec![1, 2],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under(3),
                [vec![1], vec![1, 2],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under(3),
                [vec![1], vec![1, 2],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    }

    #[test]
    fn test_window_under_zero() {
        assert_eq!([1, 2, 3, 4, 5].window_under_zero(3),
                [vec![0], vec![0],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under_zero(3),
                [vec![0], vec![0],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under_zero(3),
                [vec![0], vec![0],
                vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
    }

    #[test]
    fn test_window_under_zero_with() {
        assert_eq!([1, 2, 3, 4, 5].window_under_zero_with(3),
                [[0, 0, 1], [0, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under_zero_with(3),
                [[0, 0, 1], [0, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under_zero_with(3),
                [[0, 0, 1], [0, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
    }

    #[test]
    fn test_window_under_head_with() {
        assert_eq!([1, 2, 3, 4, 5].window_under_head_with(3),
                [[1, 1, 1], [1, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!((&[1, 2, 3, 4, 5]).window_under_head_with(3),
                [[1, 1, 1], [1, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
        assert_eq!(vec![1, 2, 3, 4, 5].window_under_head_with(3),
                [[1, 1, 1], [1, 1, 2],
                [1, 2, 3], [2, 3, 4], [3, 4, 5]]);
    }
}
