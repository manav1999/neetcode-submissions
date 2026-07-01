impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut setOfNums:HashSet<i32> = HashSet::new(); 
    
    for i in 0..nums.len(){

        if setOfNums.contains(&nums[i]) == true {

            return true;
        }
                    setOfNums.insert(nums[i]);

    }
    false
     }
}
