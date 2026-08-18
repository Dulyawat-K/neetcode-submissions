impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let steps = nums.len();
        let mut result = Vec::new();

        for i in 0..steps {
            for j in 0..steps {
                if i == j {
                    continue
                }
                if target == nums[i] + nums[j] {
                    let num_i = i as i32;
                    let num_j = j as i32;
                    result.extend([num_i, num_j]);

                    return result
                }
            }
        }

        result
    }
}
