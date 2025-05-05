//! # Leetcode 1128. Number of Equivalent Domino Pairs
//! https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/
//! - `Easy`; `y2025m05d04`; `Independently Solved`; `0ms`; `4.3mb`; `3 attempts`;
//! Topics: uncategorized.

pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut pair_count_arr: [usize; 81] = [0; 81]; // Was 64, 65 before haha.

    let mut ans_count: usize = 0;
    for domino in dominoes {
        let v_0 = domino[0] - 1;
        let v_1 = domino[1] - 1;

        let idx = if v_0 < v_1 {
            v_0 * 9 + v_1 // Was 8 before haha.
        } else {
            v_1 * 9 + v_0
        } as usize;

        // println!("{} {}", v_0, v_1);

        let entry_mut_ref = &mut pair_count_arr[idx];

        ans_count += *entry_mut_ref;
        *entry_mut_ref += 1;
    }

    ans_count as i32
}
