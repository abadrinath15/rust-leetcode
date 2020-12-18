use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

fn main() {
    let nums = vec![3, 3];
    let target = 6;
    println!("{:?}", two_sum(nums, target));
}
