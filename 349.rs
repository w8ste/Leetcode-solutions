use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // get the intersection of num1 and 2
        let mut inter: Vec<i32> = nums1
            .iter()
            .filter(|&x| nums2.contains(x))
            .cloned()
            .collect();

        // create vector with solely unique values
        let unique_elements: Vec<i32> = {
            let mut set = HashSet::new();
            inter.retain(|&x| set.insert(x));
            inter
        };
        unique_elements
    }
}
