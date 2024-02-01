impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        if(num2 == 0) {
            return num1;
        }
        return Self::sum(num1^num2, (num1&num2)<<1);
    }
    pub fn alt_sum(num1: i32, num2: i32) -> i32 {
        return num1 + num2; //less efficient
    }
}

