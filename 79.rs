impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if Self::dfs(&board, &word, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(board: &Vec<Vec<char>>, word: &String, i: usize, j: usize, s: usize) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        if i >= rows || j >= cols {
            return false;
        }

        let target_char = word.chars().nth(s).unwrap();
        if board[i][j] != target_char {
            return false;
        }
        if s == word.len() - 1 {
            return true;
        }

        let mut new_board = board.clone();
        let cache = new_board[i][j];
        new_board[i][j] = '*';

        let is_exist = Self::dfs(&new_board, word, i + 1, j, s + 1)
            || Self::dfs(&new_board, word, i.wrapping_sub(1), j, s + 1)
            || Self::dfs(&new_board, word, i, j + 1, s + 1)
            || Self::dfs(&new_board, word, i, j.wrapping_sub(1), s + 1);

        new_board[i][j] = cache;

        is_exist
    }
}
