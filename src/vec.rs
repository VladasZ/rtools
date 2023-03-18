pub trait MapVec<T, U> {
    fn map<F: FnMut(T) -> U>(self, f: F) -> Vec<U>;
}

impl<T, U> MapVec<T, U> for Vec<T> {
    fn map<F: FnMut(T) -> U>(self, f: F) -> Vec<U> {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::MapVec;

    #[test]
    fn map_vec() {
        let strings = vec!["1", "2", "3", "4", "5"];
        let ints = strings.map(|a| a.parse::<u32>().unwrap());
        assert_eq!(ints, vec![1, 2, 3, 4, 5]);
    }
}
