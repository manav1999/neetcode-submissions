use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut temp: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let x = target - nums[i];

            if temp.contains_key(&x) {
                return vec![*temp.get(&x).unwrap(), i as i32];
            } else {
                temp.insert(nums[i], i as i32);
            }
        }

        vec![]
    }
}