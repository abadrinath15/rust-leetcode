use crate::solution_files::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_27_1() {
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    }
    #[test]
    fn test_27_2() {
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
