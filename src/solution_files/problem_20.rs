use crate::solution_files::Solution;

impl Solution {
    fn closing_pair(c: char) -> char {
        match c {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            _ => '\0',
        }
    }

    pub fn is_valid(s: String) -> bool {
        let mut open_stack: Vec<char> = Vec::new();
        for bracket in s.chars() {
            match bracket {
                '(' | '{' | '[' => open_stack.push(bracket),
                ')' | '}' | ']' => {
                    if let Some(open_bracket) = open_stack.pop() {
                        if bracket != Solution::closing_pair(open_bracket) {
                            return false
                        }
                    }
                    else {
                        return false
                    }
                },
                _ => return false,
            }
        }
        if open_stack.len() == 0 {
            return true
        }
        else {
            return false
        }
    }
}
