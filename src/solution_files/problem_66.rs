use crate::solution_files::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut carry = 1;
        for digit in digits.iter().rev() {
            let mut val = digit + carry;
            carry = if val == 10 {
                val = 0;
                1
            } else {
                0
            };
            res.push(val);
        }
        if carry == 1 {
            res.push(1);
        }
        res.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_66_1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }
    #[test]
    fn test_66_2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }
    #[test]
    fn test_66_3() {
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
    }
    #[test]
    fn test_66_4() {
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
    }
}
