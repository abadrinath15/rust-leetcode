use crate::solution_files::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        for word in s.rsplit(' ') {
            if word.len() > 0 {
                return word.len() as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58_1() {
        assert_eq!(
            5,
            Solution::length_of_last_word(String::from("Hello World"))
        );
    }

    #[test]
    fn test_58_2() {
        assert_eq!(0, Solution::length_of_last_word(String::from(" ")));
    }

    #[test]
    fn test_58_3() {
        assert_eq!(1, Solution::length_of_last_word(String::from("a ")));
    }
}
