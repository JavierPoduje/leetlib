use std::collections::HashSet;

use crate::Solution;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut hash = HashSet::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, ch) in row.iter().enumerate() {
                if *ch == '1' {
                    hash.insert((row_idx as i32, col_idx as i32));
                }
            }
        }

        let mut islands = 0;
        while let Some(p) = hash.iter().next() {
            let mut stack = vec![*p];

            islands += 1;

            while let Some(curr) = stack.pop() {
                hash.remove(&curr);

                for dir in Self::DIRS {
                    let pos = (curr.0 + dir.0, curr.1 + dir.1);
                    if hash.contains(&pos) {
                        stack.push(pos);
                    }
                }
            }
        }

        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
