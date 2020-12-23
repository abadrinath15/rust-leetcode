use std::collections::HashMap;
use crate::solution_files::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (ind, val) in nums.iter().enumerate() {
            let gap = target - val;
            match map.get(&gap){
                Some(x) => {
                    return vec![*x as i32, ind as i32];
                    }
                None => {
                    map.insert(val, ind);
                }

            };
        }
        vec![]   
    }
}