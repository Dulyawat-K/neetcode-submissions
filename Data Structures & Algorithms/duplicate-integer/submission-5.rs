impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut dup = HashSet::new();
        
        for num in nums {
            if dup.contains(&num) {
                return true;
            }

            dup.insert(num);
        }

        false
    }
}
