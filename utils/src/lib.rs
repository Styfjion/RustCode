pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(feature = "test-util")]
pub fn feature_func() {
    println!("This is feature func");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
