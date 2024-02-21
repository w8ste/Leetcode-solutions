impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut r: i32 = right;

        while left < r {
            r = r & (r - 1);
        }
        return r;
    }
}
