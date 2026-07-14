impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(2);

        let mut temp: HashMap<i32,i32> = HashMap::new();


        for i in 0..nums.len(){
            let x:i32 = target - nums[i];

            if temp.contains_key(&x){
                ans.insert(0,*temp.get(&x).unwrap());
                ans.insert(1,i as i32);

                return ans;
            }else {
                temp.insert(nums[i],i as i32);
            }
        }

        ans
    }
}
