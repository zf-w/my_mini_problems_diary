//! # Leetcode 3607. Power Grid Maintenance
//! https://leetcode.com/problems/power-grid-maintenance/
//! - `Medium`; `y2025m11d06`; `Independently Solved`; `83ms`; `23.5mb`; `1 attempt`;
//! Topics: union_find,ordered_set.

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let node_num = c as usize;

    let mut union_find_idx_arr: Box<[usize]> = vec![0; node_num].into_boxed_slice();

    for (i, entry_mut_ref) in union_find_idx_arr.iter_mut().enumerate() {
        *entry_mut_ref = i;
    }

    fn union_find_find_unchecked(
        union_find_idx_arr_mut_ref: &mut [usize],
        query_node_i: usize,
    ) -> usize {
        let next_node_i = union_find_idx_arr_mut_ref[query_node_i];

        if next_node_i == query_node_i {
            return query_node_i;
        }

        let ans_node_i = union_find_find_unchecked(union_find_idx_arr_mut_ref, next_node_i);

        union_find_idx_arr_mut_ref[query_node_i] = ans_node_i;

        ans_node_i
    }

    fn union_find_union_unchecked(
        union_find_idx_arr_mut_ref: &mut [usize],
        node_0_i: usize,
        node_1_i: usize,
    ) -> bool {
        let node_0_group_node_i = union_find_find_unchecked(union_find_idx_arr_mut_ref, node_0_i);
        let node_1_group_node_i = union_find_find_unchecked(union_find_idx_arr_mut_ref, node_1_i);

        if node_0_group_node_i == node_1_group_node_i {
            return false;
        }

        if node_0_group_node_i < node_1_group_node_i {
            union_find_idx_arr_mut_ref[node_1_group_node_i] = node_0_group_node_i;
        } else {
            union_find_idx_arr_mut_ref[node_0_group_node_i] = node_1_group_node_i;
        }

        true
    }

    let mut group_num = node_num;

    for connection_vec in connections {
        let node_0_i = connection_vec[0] as usize - 1;
        let node_1_i = connection_vec[1] as usize - 1;

        let union_res_flag =
            union_find_union_unchecked(&mut union_find_idx_arr, node_0_i, node_1_i);

        if union_res_flag == true {
            group_num -= 1;
        }
    }

    use std::collections::BTreeSet;

    let mut group_node_idx_set_box_arr: Box<[BTreeSet<usize>]> =
        vec![BTreeSet::new(); group_num].into_boxed_slice();

    let mut group_node_to_group_i_box_arr = vec![node_num; node_num].into_boxed_slice();

    let mut curr_group_i: usize = 0;

    for node_i in 0..node_num {
        let group_node_i = union_find_find_unchecked(&mut union_find_idx_arr, node_i);

        if group_node_i == node_i || group_node_to_group_i_box_arr[group_node_i] == node_num {
            group_node_to_group_i_box_arr[group_node_i] = curr_group_i;
            curr_group_i += 1;
        }

        let group_i = group_node_to_group_i_box_arr[group_node_i];

        group_node_idx_set_box_arr[group_i].insert(node_i);
    }

    let mut ans_vec = Vec::with_capacity(queries.len());

    for query in queries {
        let type_idx = query[0];
        let query_node_i = query[1] as usize - 1;

        let group_node_i = union_find_find_unchecked(&mut union_find_idx_arr, query_node_i);
        let group_i = group_node_to_group_i_box_arr[group_node_i];
        let group_set_mut_ref = &mut group_node_idx_set_box_arr[group_i];

        if type_idx == 1 {
            let ans_idx = if group_set_mut_ref.contains(&query_node_i) {
                query_node_i as i32 + 1
            } else if let Some(min_node_idx) = group_set_mut_ref.first().cloned() {
                min_node_idx as i32 + 1
            } else {
                -1
            };

            ans_vec.push(ans_idx);
        } else {
            group_set_mut_ref.remove(&query_node_i);
        }
    }
    ans_vec
}