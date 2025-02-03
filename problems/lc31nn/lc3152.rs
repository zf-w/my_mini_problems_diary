//! # Leetcode 3152. Special Array II
//! https://leetcode.com/problems/special-array-ii/
//! - `Medium`; `y2024m12d09`; `Independently Solved`; `15ms`; `10mb`; `4 attempts`;

pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut idxs: Vec<usize> = Vec::with_capacity(nums.len());
    let nums_iter_0 = nums.iter().cloned();
    let mut nums_iter_1 = nums.iter().cloned();
    nums_iter_1.next();
    for (i, (prev_num, curr_num)) in nums_iter_0.zip(nums_iter_1).enumerate() {
        if prev_num & 1 == curr_num & 1 {
            idxs.push(i);
        }
    }
    idxs.sort_unstable();

    let mut ans_bool_vec: Vec<bool> = Vec::with_capacity(queries.len());
    for query in queries {
        let curr_begin_i = query[0] as usize;
        let curr_end_i = query[1] as usize;
        if curr_begin_i == curr_end_i {
            ans_bool_vec.push(true);
            continue;
        }
        ans_bool_vec.push(
            match (
                idxs.binary_search(&curr_begin_i),
                idxs.binary_search(&curr_end_i),
            ) {
                (Ok(i_0), Ok(i_1)) => false,
                (Ok(i_0), Err(i_1)) => false,
                (Err(i_0), Ok(i_1)) => i_0 == i_1,
                (Err(i_0), Err(i_1)) => i_0 == i_1,
            },
        );
    }
    ans_bool_vec
}
