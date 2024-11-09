//! ## Leetcode 1590. Make Sum Divisible by P
//! https://leetcode.com/problems/make-sum-divisible-by-p/
//! - `Medium`; `Hinted`; `y2024m10d03`;

pub fn min_subarray(mut nums: Vec<i32>, p: i32) -> i32 {
    let mut prev_sum: i32 = 0;
    for num_mut_ref in nums.iter_mut() {
        prev_sum = (prev_sum + *num_mut_ref) % p;
        *num_mut_ref = prev_sum;
    }
    let mut prev_mod_i_map: HashMap<i32, usize> = HashMap::with_capacity(p as usize);
    let need = prev_sum % p;
    let len = nums.len();
    let mut ans_min_usize = len;
    use std::collections::hash_map::Entry;
    for (i, curr_mod_num) in nums.into_iter().enumerate() {
        let target_mod_num = (curr_mod_num + p - need) % p;
        if let Some(target_i) = prev_mod_i_map.get(&target_mod_num) {
            ans_min_usize = ans_min_usize.min(i - target_i);
        }
        match prev_mod_i_map.entry(curr_mod_num) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.insert(i);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(i);
            }
        }
    }
    if ans_min_usize == len {
        -1
    } else {
        ans_min_usize as i32
    }
}
