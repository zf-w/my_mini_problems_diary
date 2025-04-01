//! ## Leetcode 962. Maximum Width Ramp
//! https://leetcode.com/problems/maximum-width-ramp/
//! - `Medium`; `Independently Solved (super slow)`; `y2024m10d09`;

use std::collections::BTreeMap;

pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut mp: BTreeMap<i32, usize> = BTreeMap::new();
    let mut ans_max_len: usize = 0;
    for (i, num) in nums.into_iter().enumerate() {
        let prev_i = if let Some((_, last_idx_ref)) = mp.range(..=num).last() {
            // ans_max_len = ans_max_len.max(i - *last_idx_ref);
            last_idx_ref.clone()
        } else {
            i
        };

        ans_max_len = ans_max_len.max(i - prev_i);
        use std::collections::btree_map::Entry;
        match mp.entry(num) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(prev_i);
            }
            _ => (),
        }
    }
    ans_max_len as i32
}
