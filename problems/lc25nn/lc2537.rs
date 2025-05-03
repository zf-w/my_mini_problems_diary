//! # Leetcode 2537. COunt the Number of Good Subarrays
//! https://leetcode.com/problems/count-the-number-of-good-subarrays/
//! - `Medium`; `y2025m04d16`; `Independently Solved`; `13ms`; `5.5mb`; `2 attempts`;
//! Topics: sliding_window.

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    use std::collections::{hash_map::Entry, HashMap};
    let k = k as usize;
    let len = nums.len();
    let mut good_count: usize = 0;
    let mut count_map: HashMap<i32, usize> = HashMap::with_capacity(len);

    let mut begin_i: usize = 0;
    let mut end_i: usize = 0;

    fn push_num(map_mut_ref: &mut HashMap<i32, usize>, count_mut_ref: &mut usize, num: i32) {
        match map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let entry_count_mut_ref = occupied_entry.get_mut();

                *count_mut_ref += *entry_count_mut_ref;
                *entry_count_mut_ref += 1;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    fn pop_num(map_mut_ref: &mut HashMap<i32, usize>, count_mut_ref: &mut usize, num: i32) {
        match map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let entry_count_mut_ref = occupied_entry.get_mut();

                *entry_count_mut_ref -= 1;
                *count_mut_ref -= *entry_count_mut_ref;
            }
            Entry::Vacant(_) => {
                panic!("Remove non-existent elem...")
            }
        }
    }

    let mut ans_count: i64 = 0;
    while end_i < len {
        let num = nums[end_i];

        push_num(&mut count_map, &mut good_count, num);

        // if good_count >= k {
        //     println!("[{} {})", begin_i, end_i);
        //     let left_len = 1;
        //     let right_len = len - end_i;
        //     ans_count += (left_len * right_len) as i64;
        // }

        while good_count >= k && begin_i <= end_i {
            let left_len = 1;
            let right_len = len - end_i;
            ans_count += (left_len * right_len) as i64;
            let to_pop_num = nums[begin_i];
            begin_i += 1;
            pop_num(&mut count_map, &mut good_count, to_pop_num);
        }

        end_i += 1;
    }

    ans_count
}
