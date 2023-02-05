use crate::Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut output = vec![vec![0; mat[0].len()]; mat.len()];
        let rows = mat.len();
        let cols = mat[0].len();

        for (row_idx, row) in mat.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                output[row_idx][col_idx] = if value == &0 { 0 } else { (rows * cols) as i32 };
            }
        }

        let rows = output.len();
        let cols = output[0].len();

        traverse(&mut output, rows, cols);

        output
    }
}

fn traverse(output: &mut Vec<Vec<i32>>, rows: usize, cols: usize) {
    // top to bottom - left to right
    for row in 0..rows {
        for col in 0..cols {
            if output[row][col] == 0 {
                continue;
            }

            let neighbors = neighbors(&output, row, col);
            let closest = neighbors.iter().fold(i32::MAX, |acc, val| acc.min(*val));

            if closest + 1 < output[row][col] {
                output[row][col] = closest + 1;
            }
        }
    }

    // bottom to top - right to left
    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            if output[row][col] == 0 {
                continue;
            }

            let neighbors = neighbors(&output, row, col);
            let closest = neighbors.iter().fold(i32::MAX, |acc, val| acc.min(*val));

            if closest + 1 < output[row][col] {
                output[row][col] = closest + 1;
            }
        }
    }
}

fn neighbors(mat: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    let mut neighbors = Vec::new();
    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    for dir in dirs {
        let neighbor = (dir.0 + row as i32, dir.1 + col as i32);
        if in_boundaries(neighbor, mat.len(), mat[0].len()) {
            neighbors.push(mat[neighbor.0 as usize][neighbor.1 as usize]);
        }
    }
    neighbors
}

fn in_boundaries(neighbor: (i32, i32), rows: usize, cols: usize) -> bool {
    neighbor.0 >= 0 && neighbor.0 < rows as i32 && neighbor.1 >= 0 && neighbor.1 < cols as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let ans = Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
        assert_eq!(ans, vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
    }

    #[test]
    fn ex2_test() {
        let ans = Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
        assert_eq!(ans, vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]);
    }

    #[test]
    fn ex3_test() {
        let ans = Solution::update_matrix(vec![
            vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
            vec![1, 1, 1, 1, 0, 1, 0, 0, 1, 1],
        ]);
        assert_eq!(
            ans,
            vec![
                vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
                vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
                vec![1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
                vec![2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
                vec![3, 2, 2, 1, 0, 1, 0, 0, 1, 1],
            ]
        );
    }

    #[test]
    fn ex4_test() {
        let ans = Solution::update_matrix(vec![
            vec![1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
            vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
            vec![0, 1, 0, 1, 1, 0, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 1, 0, 1, 1, 1, 1],
        ]);
        assert_eq!(
            ans,
            vec![
                vec![2, 1, 0, 0, 1, 0, 0, 1, 1, 0],
                vec![1, 0, 0, 1, 0, 1, 1, 2, 2, 1],
                vec![1, 1, 1, 0, 0, 1, 2, 2, 1, 0],
                vec![0, 1, 2, 1, 0, 1, 2, 3, 2, 1],
                vec![0, 0, 1, 2, 1, 2, 1, 2, 1, 0],
                vec![1, 1, 2, 3, 2, 1, 0, 1, 1, 1],
                vec![0, 1, 2, 3, 2, 1, 1, 0, 0, 1],
                vec![1, 2, 1, 2, 1, 0, 0, 1, 1, 2],
                vec![0, 1, 0, 1, 1, 0, 1, 2, 2, 3],
                vec![1, 2, 1, 0, 1, 0, 1, 2, 3, 4]
            ]
        );
    }
}
