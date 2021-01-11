use crate::solution_files::Solution;
use std::char;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = String::from("1");
        for _ in 1..n {
            let mut char_count = 0;
            let mut prev_char = 'a';
            let mut place_holder = String::new();
            for num in res.chars() {
                if prev_char == 'a' {
                    prev_char = num;
                    char_count += 1;
                } else if prev_char == num {
                    char_count += 1;
                } else {
                    place_holder.push(char::from_digit(char_count, 10).unwrap());
                    place_holder.push(prev_char);
                    prev_char = num;
                    char_count = 1;
                }
            }
            place_holder.push(char::from_digit(char_count, 10).unwrap());
            place_holder.push(prev_char);
            res = place_holder;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38_1() {
        assert_eq!(String::from("1"), Solution::count_and_say(1));
    }
    #[test]
    fn test_38_2() {
        assert_eq!(String::from("11"), Solution::count_and_say(2));
    }
    #[test]
    fn test_38_3() {
        assert_eq!(String::from("21"), Solution::count_and_say(3));
    }
    #[test]
    fn test_38_4() {
        assert_eq!(String::from("1211"), Solution::count_and_say(4));
    }
}
