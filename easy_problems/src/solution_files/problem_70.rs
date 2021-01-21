use crate::solution_files::Solution;

fn combination(n: i32, c: i32, r: i32) -> i32 {
    let mut res = 1;
    let mut n = n as i64;
    let mut c = c as i64;
    let mut r = r as i64;
    while n > 0 {
        res = res * n;
        if c > 0 && res % c == 0 {
            res = res / c;
            c -= 1;
        }
        if r > 0 && res % r == 0 {
            res = res / r;
            r -= 1;
        }
        n -= 1;
    }
    res as i32
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Use a bit of math & combinatorics to get to the solution. We know 1*a + 2*b = n;
        // so b = (n - a) / 2
        let mut count;
        if n % 2 == 0 {
            count = 2;
        } else {
            count = 1;
        }
        for a in 1..n {
            if (n - a) % 2 == 0 {
                let b = (n - a) / 2;
                count += combination(a + b, a, b)
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70_1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test_70_2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
    #[test]
    fn test_70_4() {
        assert_eq!(5, Solution::climb_stairs(4));
    }
    #[test]
    fn test_70_5() {
        assert_eq!(14930352, Solution::climb_stairs(35));
    }
}
