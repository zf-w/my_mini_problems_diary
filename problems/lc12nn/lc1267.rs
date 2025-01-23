//! # Leetcode 1267. Count Servers that Communicate
//! https://leetcode.com/problems/count-servers-that-communicate/
//! - `Medium`; `y2025m01d22`; `Independently Solved`; `0ms`; `2.7mb`; `2 attempts`;

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let row_num: usize = grid.len();
    let col_num: usize = grid[0].len();

    let mut total_server_count: usize = 0;

    #[derive(Clone)]
    enum Info {
        None,
        One(usize),
        More,
    }

    impl Info {
        pub fn update(&mut self, idx: usize) {
            *self = match self {
                Info::None => Self::One(idx),
                Info::One(_) => Self::More,
                Info::More => Self::More,
            };
        }
    }

    let mut info_vec: Vec<Info> = vec![Info::None; row_num + col_num];

    let (row_info_arr_mut_ref, col_info_arr_mut_ref) = info_vec.split_at_mut(row_num);

    for (row_i, row_ref) in grid.iter().enumerate() {
        for (col_i, cell_ref) in row_ref.iter().enumerate() {
            if *cell_ref > 0 {
                row_info_arr_mut_ref
                    .get_mut(row_i)
                    .expect("0, length checked.")
                    .update(col_i);

                col_info_arr_mut_ref
                    .get_mut(col_i)
                    .expect("1, length checked.")
                    .update(row_i);

                total_server_count += 1;
            }
        }
    }

    for (row_i, row_info) in row_info_arr_mut_ref.iter().enumerate() {
        if let Info::One(col_i_ref) = row_info {
            if let Info::One(row_i_ref) = col_info_arr_mut_ref
                .get(*col_i_ref)
                .expect("code 2, expecting length checked")
            {
                assert_eq!(*row_i_ref, row_i); // First attempt bug: asserting the "row_i_ref" instead of the "row_i."
                total_server_count -= 1;
            }
        }
    }
    total_server_count as i32
}
