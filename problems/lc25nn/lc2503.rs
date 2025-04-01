//! # Leetcode 2503. Maximum Number of Points From Grid Queries
//! https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/
//! - `Hard`; `y2025m03d28`; `Independently Solved`; `34ms`; `4mb`; `1 attempt`;
//! Topics: breadth_first_search/priority_queue.

pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::BinaryHeap;
    let row_num = grid.len();
    let col_num = grid[0].len();

    let cord_to_index_closure = |row_i: usize, col_i: usize| -> usize { row_i * col_num + col_i };

    let index_to_cord_closure = |idx: usize| -> (usize, usize) { (idx / col_num, idx % col_num) };

    let mut curr_group_max = grid[0][0];

    let mut index_pq: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(row_num * col_num);

    index_pq.push((-curr_group_max, 0));

    grid[0][0] = -grid[0][0];

    let mut curr_group_num: usize = 0;

    let mut info_vec: Vec<(i32, usize)> = Vec::with_capacity(row_num * col_num);

    while let Some((curr_val_rev, curr_i)) = index_pq.pop() {
        let (curr_row_i, curr_col_i) = index_to_cord_closure(curr_i);
        let curr_val = -curr_val_rev;

        if curr_val > curr_group_max {
            info_vec.push((curr_group_max, curr_group_num));

            curr_group_max = curr_val;
        }
        curr_group_num += 1;

        if curr_row_i > 0 {
            let next_row_i = curr_row_i - 1;
            let entry_mut_ref = &mut grid[next_row_i][curr_col_i];

            if *entry_mut_ref > 0 {
                index_pq.push((
                    -(*entry_mut_ref),
                    cord_to_index_closure(next_row_i, curr_col_i),
                ));
                *entry_mut_ref = -(*entry_mut_ref);
            }
        }
        if curr_row_i + 1 < row_num {
            let next_row_i = curr_row_i + 1;
            let entry_mut_ref = &mut grid[next_row_i][curr_col_i];

            if *entry_mut_ref > 0 {
                index_pq.push((
                    -(*entry_mut_ref),
                    cord_to_index_closure(next_row_i, curr_col_i),
                ));
                *entry_mut_ref = -(*entry_mut_ref);
            }
        }
        if curr_col_i > 0 {
            let next_col_i = curr_col_i - 1;
            let entry_mut_ref = &mut grid[curr_row_i][next_col_i];

            if *entry_mut_ref > 0 {
                index_pq.push((
                    -(*entry_mut_ref),
                    cord_to_index_closure(curr_row_i, next_col_i),
                ));
                *entry_mut_ref = -(*entry_mut_ref);
            }
        }

        if curr_col_i + 1 < col_num {
            let next_col_i = curr_col_i + 1;
            let entry_mut_ref = &mut grid[curr_row_i][next_col_i];

            if *entry_mut_ref > 0 {
                index_pq.push((
                    -(*entry_mut_ref),
                    cord_to_index_closure(curr_row_i, next_col_i),
                ));
                *entry_mut_ref = -(*entry_mut_ref);
            }
        }
    }

    info_vec.push((curr_group_max, curr_group_num));

    // println!("{:?}", info_vec);

    let query_num = queries.len();
    let mut ans_vec = Vec::with_capacity(query_num);

    let info_vec = info_vec;

    for mut query in queries {
        query -= 1;

        let mut begin_i = 0;
        let mut end_i = info_vec.len();

        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            let mid_val = info_vec[mid_i];
            // Be careful with this `>` haha. It was `>=` before.
            if mid_val.0 > query {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }

        ans_vec.push(if begin_i == 0 {
            0
        } else {
            info_vec[begin_i - 1].1 as i32
        });
    }

    ans_vec
}
