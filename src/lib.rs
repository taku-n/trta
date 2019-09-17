pub trait Trta<T> {
    fn sum(&self) -> T;
    fn ave(&self) -> T;

    fn window(&self, period: usize) -> Vec<Vec<T>>;
    // (1 2 3 4 5) => ((1 2 3) (2 3 4) (3 4 5))
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
}
