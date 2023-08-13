#[allow(dead_code)]
fn unique(mut list: Vec<i32>) -> Vec<i32> {
    list.sort();
    list.dedup();
    list
}
#[allow(dead_code)]
fn unique_generic<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();
    list
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unique_all() {
        let input: Vec<i32> = vec![];
        assert_eq!(unique(input), []);

        let input: Vec<i32> = vec![1, 2, 2, 4];
        assert_eq!(unique(input), [1, 2, 4]);

        let input: Vec<i32> = vec![3, 3, 3, 3, 1, 2];
        assert_eq!(unique(input), [1, 2, 3]);

        let input: Vec<i32> = vec![3, 1, 2, 2];
        assert_eq!(unique(input), [1, 2, 3]);
    }

    #[test]
    fn test_unique_generic() {
        let input: Vec<i64> = vec![];
        assert_eq!(unique_generic(input), []);

        let input: Vec<i32> = vec![1, 2, 2, 4];
        assert_eq!(unique_generic(input), [1, 2, 4]);

        let input: Vec<i32> = vec![3, 3, 3, 3, 1, 2];
        assert_eq!(unique_generic(input), [1, 2, 3]);

        let input: Vec<i64> = vec![3, 1, 2, 2];
        assert_eq!(unique_generic(input), [1, 2, 3]);
    }
}
