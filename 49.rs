use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut sol: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut k: Vec<char> = s.chars().collect();
            k.sort();
            map.entry(k.into_iter().collect())
                .or_insert_with(|| Vec::new())
                .push(s);
        }

        for (_, anagrams) in map {
            sol.push(anagrams);
        }

        return sol;
    }
}
