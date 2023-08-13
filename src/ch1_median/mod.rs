#[allow(dead_code)]
fn median(list: &mut Vec<f32>) -> Option<f32> {
    if list.is_empty() {
        return None;
    }
    // sort the items
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // if odd then its the middle value
    if list.len() % 2 == 1 {
        let item_index = list.len() / 2;
        Some(list[item_index])
    } else {
        // else take the avg of two middle items
        let item_index = list.len() / 2;
        let item2 = list[item_index];
        let item1 = list[item_index - 1];

        Some((item1 + item2) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_median_all() {
        let mut input: Vec<f32> = vec![];
        assert_eq!(median(&mut input), None);

        let mut input: Vec<f32> = vec![1.0, 2.0];
        assert_eq!(median(&mut input), Some(1.5));

        let mut input: Vec<f32> = vec![3.0, 1.0, 2.0];
        assert_eq!(median(&mut input), Some(2.0));

        let mut input: Vec<f32> = vec![3.0, 1.0, 2.0, 2.2];
        assert_eq!(median(&mut input), Some(2.1));
    }
}
