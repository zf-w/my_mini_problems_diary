//! ## Leetcode 2482. Difference Between Ones and Zeros in Row and Column
//! https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column
//! - `Medium`; `Independently Solved`; `2023-12-13`;
//! 
//! I checked the solution after I solved this. I guess the time complexity is at least `O(m * n)`, since that's the size of the resulted array. I wonder how the Java solution can be as fast as 3ms.
//! 
//! Another interesting point would be ways of initializing vectors. Using the default constructor with a default value and a length is an option. An alternative would be manually pushing elements into a vector constructed with capacity. Although these two options are generally the same under the surface, copying the default value might be costy considering coping vectors or similar stuff.

pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut v: Vec<i32> = vec![0; n];
    
    for j in 0..n {
        let count: &mut i32 = &mut v[j];
        for i in 0..m {
            if grid[i][j] == 1 {
                *count += 1;
            } else {
                *count -= 1;
            }
        }
    }
    let mut ans: Vec<Vec<i32>> = Vec::with_capacity(m);
    for i in 0..m {
        let mut count: i32 = 0;
        for j in 0..n {
            if grid[i][j] == 1 {
                count += 1;
            } else {
                count -= 1;
            }
        }
        let mut diff_row: Vec<i32> = Vec::with_capacity(n);
        for j in 0..n {
            diff_row.push(v[j] + count);
        }
        ans.push(diff_row);
    }
    
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::test;
    #[test]
    fn ans_correct() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![0, 0, 1]
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 0, 4],
            vec![0, 0, 4],
            vec![-2, -2, 2]
        ];
        let res = ones_minus_zeros(grid);
        
        assert_eq!(res, expected);
    }

    #[test]
    fn capacity_correct() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![0, 0, 1]
        ];
        let res = ones_minus_zeros(grid);

        let mut flag = test::vec_capa_matches_len(&res);
        for row in res.iter() {
            flag &= test::vec_capa_matches_len(row);
        }
        assert!(flag);
    }
}