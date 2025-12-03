//! # Leetcode 2435. Paths in Matrix Whose SUm Is Divisible by K
//! https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/
//! - `Hard`; `y2025m11d25`; `Independently Solved`; `15ms`; `19.3mb`; `2 attempts`;
//! Topics: dynamic_programming.

pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const PATH_MODULOS: usize = 1_000_000_007;
    let k_len = k as usize;
    let row_num = grid.len();
    let col_num = grid[0].len();

    let mut grid_dp_arr_box: Box<[usize]> = vec![0; 2 * col_num * k_len].into_boxed_slice();

    let index_closure = |row_i: usize, col_i: usize, modulos_i: usize| -> usize {
        row_i * (col_num * k_len) + col_i * k_len + modulos_i
    };

    let mut curr_modulos = 0;

    let mut prev_row_i = 0;
    let mut curr_row_i = 1;

    for col_i in 0..col_num { // The first initialization is about the first row... not the first column, even though the indices of iterating through the first is on rows.
        curr_modulos = (curr_modulos + (grid[0][col_i] as usize) % k_len) % k_len;
        grid_dp_arr_box[index_closure(prev_row_i, col_i, curr_modulos)] = 1;
    }

    curr_modulos = (grid[0][0] as usize) % k_len;

    for row_i in 1..row_num {
        let curr_cell_value = (grid[row_i][0] as usize) % k_len;
        curr_modulos = (curr_modulos + curr_cell_value) % k_len;
        
        let curr_cell_dp_idx = index_closure(curr_row_i, 0, 0);
        
        for modulos_i in 0..k_len { // Don't forget to initialize the entries before reusing them...
            grid_dp_arr_box[curr_cell_dp_idx + modulos_i] = 0;
        }

        grid_dp_arr_box[index_closure(curr_row_i, 0, curr_modulos)] = 1;

        for col_i in 1..col_num {
            let curr_cell_value = (grid[row_i][col_i] as usize) % k_len;
            let curr_cell_dp_idx = index_closure(curr_row_i, col_i, 0);
            let prev_row_cell_dp_idx = index_closure(prev_row_i, col_i, 0);
            let prev_col_cell_dp_idx = index_closure(curr_row_i, col_i - 1, 0);

            for modulos_i in 0..k_len {
                let prev_modulos_i = (modulos_i + k_len - curr_cell_value) % k_len; // First major bug here, it was "curr_cell_value + k_len - modulos_i" haha.

                grid_dp_arr_box[curr_cell_dp_idx + modulos_i] =
                    ((grid_dp_arr_box[prev_row_cell_dp_idx + prev_modulos_i]
                        + grid_dp_arr_box[prev_col_cell_dp_idx + prev_modulos_i])
                        % PATH_MODULOS);
            }
        }

        std::mem::swap(&mut prev_row_i, &mut curr_row_i);
    }

    grid_dp_arr_box[index_closure(prev_row_i, col_num - 1, 0)] as i32
}