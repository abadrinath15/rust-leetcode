use crate::solution_files::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => -1,
            Some(index) => index as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_28_1() {
        assert_eq!(0, Solution::str_str(String::from(""), String::from("")));
    }
    #[test]
    fn test_28_2() {
        assert_eq!(
            2,
            Solution::str_str(String::from("hello"), String::from("ll"))
        );
    }
    #[test]
    fn test_28_3() {
        assert_eq!(
            -1,
            Solution::str_str(String::from("aaaaa"), String::from("bba"))
        );
    }
}
