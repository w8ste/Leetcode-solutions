use std::cmp;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let num_columns = matrix[0].len();
        let mut heights = vec![0; num_columns];
        let mut max_area = 0;

        for row in matrix {
            for j in 0..num_columns {
                if row[j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }
            max_area = cmp::max(max_area, Self::largest_rectangle_area(&heights));
        }

        max_area
    }

    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut result = 0;
        let mut index_stack = Vec::new();
        let mut left = vec![-1; n];
        let mut right = vec![n as i32; n];

        for i in 0..n {
            while !index_stack.is_empty() && heights[*index_stack.last().unwrap()] >= heights[i] {
                right[*index_stack.last().unwrap()] = i as i32;
                index_stack.pop();
            }
            left[i] = if index_stack.is_empty() {
                -1
            } else {
                *index_stack.last().unwrap() as i32
            };
            index_stack.push(i);
        }

        for i in 0..n {
            result = result.max(heights[i] * (right[i] - left[i] - 1));
        }

        result
    }
}
