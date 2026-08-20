impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<Vec<char>,Vec<String>>::new();

        for element in strs {
            let mut chars = element.chars().collect::<Vec<char>>();
            chars.sort();
            // print!("{:#?}", chars);

            let group = map.entry(chars).or_insert_with(Vec::new);
            group.push(element);
            // print!("{:#?}", map);

        }

        let val = map.into_values().collect();
        val
    }
}
