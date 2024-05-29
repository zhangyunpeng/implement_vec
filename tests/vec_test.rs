#[cfg(test)]
mod vec_test {
    use implementing_vec::vec::vec::Vec;
    #[test]
    fn basic() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), None);
    }
}