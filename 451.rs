use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut vec: Vec<char> = map.keys().cloned().collect();

        vec.sort_by(|&a, &b| map[&b].cmp(&map[&a]));

        let mut sol: String = String::new();

        for c in vec {
            sol += &c.to_string().repeat(map[&c]);
        }

        return sol;
    }
}
