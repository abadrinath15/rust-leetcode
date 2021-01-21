use crate::solution_files::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f32).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69_1() {
        assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn test_69_2() {
        assert_eq!(2, Solution::my_sqrt(8));
    }
    #[test]
    fn test_69_3() {
        assert_eq!(46340, Solution::my_sqrt(2147483647));
    }
}
