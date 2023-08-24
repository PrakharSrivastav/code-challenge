#[allow(dead_code)]
fn sort_usernames(v: &mut [&str]) {
    v.sort_by_key(|item| item.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usernames_sort() {
        let mut users = vec!["Todd", "amy"];
        sort_usernames(&mut users);
        assert_eq!(users, vec!["amy", "Todd"])
    }
}
