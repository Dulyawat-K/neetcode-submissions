impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let steps = nums.len();
        let mut result = Vec::new();
        let mut mem = HashMap::<i32, usize>::new();

        for i in 0..steps {
            let current = nums[i];
            let needed = target - current;
            
            if let Some(key) = mem.get(&needed) {
                let num_i = i as i32;
                let num_key = *key as i32;
                result.extend([num_key, num_i]);
                return result;
            }
            else {
                let num_i = i as i32;
                mem.insert(current, i);
            }
        }
        
        result
    }
}
