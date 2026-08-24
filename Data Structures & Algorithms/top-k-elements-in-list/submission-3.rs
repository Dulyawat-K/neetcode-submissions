impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        let mut buckets: Vec<Vec<i32>>= Vec::new();
        let buckets_count = nums.len() + 1;
        let mut result = Vec::new();

        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        for _ in 0..buckets_count {
            buckets.push(Vec::new());
        }

        for (number,count) in freq {
            buckets[count].push(number);
        }

        for i in (0..buckets.len()).rev() {
            for bucket in &buckets[i] {
                let k_usize = k as usize;
                if result.len() == k_usize {
                    break
                }
                else {
                    result.push(*bucket);
                }
            }
        }
    
        result
    }
}
