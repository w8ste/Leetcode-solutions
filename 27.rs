impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for n in 0..nums.len() {
            if nums[n] != val {
                nums[i] = nums[n];
                i += 1;
            }
        }
        i as i32
    }
}
