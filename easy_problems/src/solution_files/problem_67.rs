use crate::solution_files::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut b = b;
        let mut a = a;
        if a.len() > b.len() {
            let mut b_t = String::from("0").repeat(a.len() - b.len());
            b_t.push_str(&b);
            b = b_t;
        } else if a.len() < b.len() {
            let mut a_t = String::from("0").repeat(b.len() - a.len());
            a_t.push_str(&a);
            a = a_t;
        }
        let mut res = String::new();
        let mut carry = '0';
        let mut val: char;
        for (left, right) in a.chars().rev().zip(b.chars().rev()) {
            val = match (left, right) {
                ('0', '0') => '0',
                ('0', '1') => '1',
                ('1', '0') => '1',
                ('1', '1') => '2',
                (_, _) => panic!("Value not 0 or 1!"),
            };
            val = match (val, carry) {
                ('0', '0') => '0',
                ('0', '1') => {
                    carry = '0';
                    '1'
                }
                ('1', '0') => '1',
                ('1', '1') => '0',
                ('2', '0') => {
                    carry = '1';
                    '0'
                }
                ('2', '1') => '1',
                (_, _) => panic!("Value not 0 or 1!"),
            };
            res.push(val);
        }
        if carry == '1' {
            res.push(carry);
        }
        res.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67_1() {
        assert_eq!(
            String::from("100"),
            Solution::add_binary(String::from("11"), String::from("1"))
        );
    }
    #[test]
    fn test_67_2() {
        assert_eq!(
            String::from("10101"),
            Solution::add_binary(String::from("1010"), String::from("1011"))
        );
    }
}
