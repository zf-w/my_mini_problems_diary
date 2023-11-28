//! ## Leetcode 1727. Largest Submatrix With Rearrangements
//! - `Medium`
//! 
//! You are given a binary matrix `matrix` of size `m * n`, and you are allowed to rearrange the **columns** of the `matrix` in any order.
//! 
//! Return *the area of the largest submatrix within `matrix` where **every** element of the submatrix is `1` after reordering the columns optimally.
//! 
//! ### Thoughts:
//! - `Learned`; `2023-11-25`;
//! 
//! This is an interesting question. At first, I got confused about why sorting the rows and comparing the value of the column times the length of the right of it could work.

pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
    let m: usize = matrix.len();
    let n: usize = matrix[0].len();
    let mut row_copy: Vec<i32> = vec![0;n];
    fn calculate_area(row: &Vec<i32>, row_copy: &mut Vec<i32>) -> i32 {
        let n: usize = row.len();
        row_copy.copy_from_slice(row);
        row_copy.sort();
        let mut ans: i32 = 0;
        for (j, cell) in row_copy.iter().enumerate() {
            ans = ans.max(cell * (n - j) as i32);
        }
        ans
    }

    let mut ans: i32 = calculate_area(&matrix[0], &mut row_copy);
    for i in 1..m {
        row_copy.copy_from_slice(&matrix[i - 1]);
        let curr = &mut matrix[i];
        for (j, cell) in curr.iter_mut().enumerate() {
            if *cell > 0 {
                *cell = row_copy[j] + 1;
            }
        }
        
        ans = ans.max(calculate_area(curr, &mut row_copy));
    }
    ans
}