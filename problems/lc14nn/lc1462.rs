//! # Leetcode 1462. Course Schedule IV
//! https://leetcode.com/problems/course-schedule-iv/
//! - `Medium`; `y2025m01d27`; `Independently Solved`; `0ms`; `3.19mb`; `2 attempts`;

pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    use std::collections::VecDeque;
    let course_num = num_courses as usize;
    let calc_idx =
        |course_0_i: usize, course_1_i: usize| -> usize { course_0_i * course_num + course_1_i };
    let mut adj_table = vec![false; course_num * course_num].into_boxed_slice();
    let mut degree_vec: Box<[usize]> = vec![0; course_num].into_boxed_slice();
    let mut unlock_adj_vec_box_arr: Box<[Vec<usize>]> =
        vec![Vec::new(); course_num].into_boxed_slice();
    for prerequiste in prerequisites {
        let course_i = prerequiste[0] as usize;
        let unlock_course_i = prerequiste[1] as usize;
        degree_vec[unlock_course_i] += 1;
        unlock_adj_vec_box_arr[course_i].push(unlock_course_i);
    }

    let mut course_queue: VecDeque<usize> = VecDeque::with_capacity(course_num);

    for (course_i, need_num) in degree_vec.iter().cloned().enumerate() {
        if need_num == 0 {
            course_queue.push_back(course_i);
        }
    }

    let copy_row_to_another_row_plus_self =
        |adj_table_mut_ref: &mut [bool], course_i: usize, unlock_course_i: usize| {
            for i in 0..course_num {
                let from_idx = calc_idx(course_i, i);
                let to_idx = calc_idx(unlock_course_i, i);
                adj_table_mut_ref[to_idx] = adj_table_mut_ref[from_idx];
            }
            adj_table_mut_ref[calc_idx(unlock_course_i, course_i)] = true;
        };

    while course_queue.is_empty() == false {
        let curr_course_i = course_queue.pop_front().expect("checked len.");
        let unlock_course_idx_arr_ref = unlock_adj_vec_box_arr
            .get(curr_course_i)
            .expect("getting nexts.")
            .as_slice();
        for unlock_course_i in unlock_course_idx_arr_ref.iter().cloned() {
            copy_row_to_another_row_plus_self(&mut adj_table, curr_course_i, unlock_course_i);
            let unlock_degree_entry_mut_ref = degree_vec
                .get_mut(unlock_course_i)
                .expect("Getting degree.");
            *unlock_degree_entry_mut_ref -= 1;
            if *unlock_degree_entry_mut_ref == 0 {
                course_queue.push_back(unlock_course_i);
            }
        }
    }
    let mut ans_vec: Vec<bool> = Vec::with_capacity(queries.len());
    for query in queries {
        let course_i = query[1] as usize;
        let need_course_i = query[0] as usize;
        ans_vec.push(adj_table[calc_idx(course_i, need_course_i)]);
    }
    ans_vec
}
