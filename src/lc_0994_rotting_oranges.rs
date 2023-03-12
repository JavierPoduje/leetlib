use std::collections::VecDeque;

use crate::Solution;

impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut fresh_oranges = 0;
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, orange) in row.iter().enumerate() {
                if *orange == 2 {
                    queue.push_back((row_idx, col_idx));
                }
                if *orange == 1 {
                    fresh_oranges += 1;
                }
            }
        }

        let mut minutes = 0;

        while !queue.is_empty() {
            let mut next_queue = VecDeque::new();

            while let Some(p) = queue.pop_front() {
                if grid[p.0 as usize][p.1 as usize] == 1 {
                    grid[p.0 as usize][p.1 as usize] = 2;
                    fresh_oranges -= 1;
                }

                for d in Self::DIRECTIONS {
                    let row = d.0 + p.0 as i32;
                    let col = d.1 + p.1 as i32;
                    if in_boundaries(&grid, row, col) && grid[row as usize][col as usize] == 1 {
                        if grid[row as usize][col as usize] == 1 {
                            grid[row as usize][col as usize] = 2;
                            fresh_oranges -= 1;
                        }
                        next_queue.push_back((row as usize, col as usize));
                    }
                }
            }
            queue = next_queue;

            if !queue.is_empty() {
                minutes += 1;
            }
        }

        if fresh_oranges == 0 {
            minutes
        } else {
            -1
        }
    }
}

fn in_boundaries(grid: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    0 <= row && row < grid.len() as i32 && 0 <= col && col < grid[0].len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn ex2_test() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn ex3_test() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }
}
