impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut inventory_s = HashMap::new();
        let mut inventory_t = HashMap::new();

        for char in s.chars() {
            inventory_s.entry(char).and_modify(|count| *count += 1).or_insert(1);
        }

        for char in t.chars() {
            inventory_t.entry(char).and_modify(|count| *count += 1).or_insert(1);
        }

        inventory_s == inventory_t
    }
}
