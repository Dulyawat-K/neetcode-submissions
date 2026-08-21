impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        let mut result = Vec::<(i32, usize)>::new();
        let mut slice = Vec::<i32>::new();
        for num in nums {
            *seen.entry(num).or_insert(0) += 1;
        }

        for (number, freq) in seen {
            result.push((number, freq));
        }
        result.sort_by(|a, b| b.1.cmp(&a.1));

        let k_usize: usize = k as usize;
        for i in 0..k_usize {
            slice.push(result[i].0);
        }

        slice
    }
}
