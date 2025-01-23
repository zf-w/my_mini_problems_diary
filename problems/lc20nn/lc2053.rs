//! ## Leetcode 2053. Kth Distinct String in an Array
//! https://leetcode.com/problems/kth-distinct-string-in-an-array/
//! - `Easy`; `Independently Solved`; `2024-08-04`;

use std::collections::HashMap;

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut mp: HashMap<String, bool> = HashMap::new();
    for v_ref in arr.iter() {
        mp.entry(v_ref.clone())
            .and_modify(|v| *v = true)
            .or_insert(false);
    }
    let mut count = 1;
    for v in arr {
        if *mp.get(&v).expect("Getting if distinctive") == true {
            if count == k {
                return v;
            } else {
                count += 1;
            }
        }
    }
    "".to_string()
}
