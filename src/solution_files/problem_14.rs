use crate::solution_files::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs_c = strs.clone();
        let mut shortest: String;
        let mut shortest_t;
        match strs_c.pop() {
            None => shortest = String::new(),
            Some(x) => shortest = x,
        };
        for word in strs_c {
            if shortest.len() == 0 {
                break;
            }
            else if word.len() < shortest.len() {
                shortest = shortest[0..word.len()].to_string();
            }
            shortest_t = String::new();
            for x_y in shortest.chars().zip(word.chars()) {
                let (x, y) = x_y;
                if x == y {
                    shortest_t.push(x);
                }
                else {
                    break;
                }
           }
           shortest = shortest_t;
        };
        shortest
    }
}
