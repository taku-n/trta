pub trait Trta {
    fn hello(&self) {
        println!("hello, world");
    }
}

impl Trta for Vec<f64> {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
