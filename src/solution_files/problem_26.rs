use crate::solution_files::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input_1 = vec![1, 1, 2];
        assert_eq!(2, Solution::remove_duplicates(&mut input_1));
    }
    #[test]
    fn test_2() {
        let mut input_2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, Solution::remove_duplicates(&mut input_2));
    }
}
