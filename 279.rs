impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut sol: i32 = n;
        let mut num: i32 = 2;
        while num * num <= n {
            sol = Self::min(
                sol,
                n / (num * num) as i32 + Self::num_squares(n % (num * num)),
            );
            num += 1;
        }
        return sol;
    }
    fn min(x: i32, y: i32) -> i32 {
        if x < y {
            x
        } else {
            y
        }
    }
}
