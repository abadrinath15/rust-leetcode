use crate::solution_files::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for index in 0..nums.len() {
            if nums[index] >= target {
                return index as i32;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_35_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_35_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_35_4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn test_35_5() {
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }
}
