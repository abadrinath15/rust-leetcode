use crate::solution_files::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {false}
        else {
            Solution::reverse_local(x) == x
        }
    }

    pub fn reverse_local(x: i32) -> i32 {
        let mut y = x;
        let mut res: i32 = 0;
        while y.abs() > 0 {
            match res.checked_mul(10) {
                Some(mul_res) => match mul_res.checked_add(y % 10) {
                    Some(add_res) => res = add_res,
                    None => {
                        res = 0;
                        break;
                    }
                },
                None => {
                    res = 0;
                    break;
                }
            }
            y = y / 10;
        }
    res       
    }
}