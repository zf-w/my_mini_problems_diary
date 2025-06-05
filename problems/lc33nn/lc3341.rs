//! # Leetcode 3341. Find Minimum Time to Reach Last Room I
//! https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/
//! - `Medium`; `y2025m05d07`; `Independently Solved`; `0ms`; `2.4mb`; `3 attempts`;
//! Topics: breath_first_search/priority_queue.

pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;

    let row_num = move_time.len();
    let col_num = move_time[0].len();

    let target_row_i = row_num - 1;
    let target_col_i = col_num - 1;

    let mut pq: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(row_num * col_num);

    let mut visited_map_vec: Vec<bool> = vec![false; row_num * col_num];

    let calc_index_fn = |row_i: usize, col_i: usize| -> usize { row_i * col_num + col_i };

    let calc_pos_fn = |idx: usize| -> (usize, usize) { (idx / col_num, idx % col_num) };

    visited_map_vec[0] = true;

    pq.push((0, 0));

    fn push_to_pq(
        row_i: usize,
        col_i: usize,
        calc_index_fn: impl Fn(usize, usize) -> usize,
        mut time: i32,
        visited_map_vec_mut_ref: &mut [bool],
        pq_mut_ref: &mut BinaryHeap<(i32, usize)>,
        move_time_ref: &[Vec<i32>],
    ) {
        let idx = calc_index_fn(row_i, col_i);
        if visited_map_vec_mut_ref[idx] == true {
            return;
        }

        visited_map_vec_mut_ref[idx] = true;

        time = time.min(-move_time_ref[row_i][col_i]) - 1;

        pq_mut_ref.push((time, idx));
    }

    while pq.is_empty() == false {
        let (time, idx) = pq.pop().expect("len > 0");
        let (row_i, col_i) = calc_pos_fn(idx);

        if row_i > 0 {
            push_to_pq(
                row_i - 1,
                col_i,
                calc_index_fn,
                time,
                &mut visited_map_vec,
                &mut pq,
                &move_time,
            );
        }

        if col_i > 0 {
            push_to_pq(
                row_i,
                col_i - 1,
                calc_index_fn,
                time,
                &mut visited_map_vec,
                &mut pq,
                &move_time,
            );
        }

        if row_i + 1 < row_num {
            if row_i + 1 == target_row_i && col_i == target_col_i {
                return (-time).max(move_time[row_i + 1][col_i]) + 1;
            }
            push_to_pq(
                row_i + 1,
                col_i,
                calc_index_fn,
                time,
                &mut visited_map_vec,
                &mut pq,
                &move_time,
            );
        }

        if col_i + 1 < col_num {
            if row_i == target_row_i && col_i + 1 == target_col_i {
                return (-time).max(move_time[row_i][col_i + 1]) + 1;
            }
            push_to_pq(
                row_i,
                col_i + 1,
                calc_index_fn,
                time,
                &mut visited_map_vec,
                &mut pq,
                &move_time,
            );
        }
    }
    unreachable!()
}
