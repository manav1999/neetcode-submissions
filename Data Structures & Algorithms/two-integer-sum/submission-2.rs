use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut temp: HashMap<i32, i32> = HashMap::new();

for (i, &n) in nums.iter().enumerate() {
    let x = target - n;
    if let Some(&j) = temp.get(&x) {
        return vec![j, i as i32];
    }
    temp.insert(n, i as i32);
}

        vec![]
    }
}