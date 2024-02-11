impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dp_current = vec![vec![-1; cols]; cols];
        let mut dp_next = vec![vec![-1; cols]; cols];
        dp_current[0][cols - 1] = grid[0][0] + grid[0][cols - 1];
        for i in 1..rows {
            for j1 in 0..cols {
                for j2 in 0..cols {
                    let cherries = grid[i][j1] + if j1 == j2 { 0 } else { grid[i][j2] };
                    for prev_j1 in (j1 as isize - 1)..=(j1 as isize + 1) {
                        for prev_j2 in (j2 as isize - 1)..=(j2 as isize + 1) {
                            if prev_j1 >= 0
                                && prev_j1 < cols as isize
                                && prev_j2 >= 0
                                && prev_j2 < cols as isize
                                && dp_current[prev_j1 as usize][prev_j2 as usize] != -1
                            {
                                dp_next[j1][j2] = dp_next[j1][j2]
                                    .max(dp_current[prev_j1 as usize][prev_j2 as usize] + cherries);
                            }
                        }
                    }
                }
            }
            std::mem::swap(&mut dp_current, &mut dp_next);
            for row in dp_next.iter_mut() {
                row.fill(-1);
            }
        }
        let mut max_cherries = 0;
        for j1 in 0..cols {
            for j2 in 0..cols {
                max_cherries = max_cherries.max(dp_current[j1][j2]);
            }
        }
        max_cherries
    }
}
