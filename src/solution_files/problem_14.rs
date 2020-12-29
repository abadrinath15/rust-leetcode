use crate::solution_files::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut shortest: String;
        let mut shortest_t: String;
        if let Some(first) = strs.first() {
            shortest = first.to_string();
            for word in &strs[1..] {
                if shortest.len() == 0 {
                    break;
                } else if word.len() < shortest.len() {
                    shortest = shortest[0..word.len()].to_string();
                }
                shortest_t = String::new();
                for x_y in shortest.chars().zip(word.chars()) {
                    let (x, y) = x_y;
                    if x == y {
                        shortest_t.push(x);
                    } else {
                        break;
                    }
                }
                shortest = shortest_t;
            }
            shortest
        } else {
            String::from("")
        }
    }
}
