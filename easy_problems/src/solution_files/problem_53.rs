use crate::solution_files::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut curr_sum = nums[0];
        for num in nums.iter().skip(1) {
            curr_sum = match curr_sum < 0 {
                true => *num,
                false => curr_sum + *num,
            };
            max_sum = curr_sum.max(max_sum);
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53_1() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
    #[test]
    fn test_53_2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }
    #[test]
    fn test_53_3() {
        assert_eq!(0, Solution::max_sub_array(vec![0]));
    }
    #[test]
    fn test_53_4() {
        assert_eq!(-1, Solution::max_sub_array(vec![-1]));
    }
}
