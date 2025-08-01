//! # Leetcode 1865. Finding Pairs With a Certain Sum
//! https://leetcode.com/problems/finding-pairs-with-a-certain-sum/
//! - `Medium`; `y2025m07d06`; `Hinted`; `35ms`; `8.8mb`; `4 attempts`;
//! Topics: count_tracking.

use std::collections::{HashMap, hash_map::Entry};
struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    num2_count_arr: HashMap<i32, usize>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        nums1.sort_unstable();
        let nums2_len = nums2.len();
        let mut ans = Self {
            nums1,
            nums2,
            num2_count_arr: HashMap::with_capacity(1000),
        };

        for i in 0..nums2_len {
            //ans.num2_count_arr[ans.nums2[num] as usize - 1] += 1;
            match ans.num2_count_arr.entry(ans.nums2[i]) {
                Entry::Occupied(mut occupied_entry) => {
                    *occupied_entry.get_mut() += 1;
                },
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                },
            }
        }

        ans
    }
    
    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let mut num2_idx = self.nums2[index];
        match self.num2_count_arr.entry(num2_idx) {
            Entry::Occupied(mut occupied_entry) => {
                let entry_mut_ref = occupied_entry.get_mut();
                *entry_mut_ref -= 1;
                if *entry_mut_ref == 0 {
                    occupied_entry.remove_entry();
                }
            },
            Entry::Vacant(vacant_entry) => unreachable!(),
        };
        self.nums2[index as usize] += val;
        num2_idx += val;
        match self.num2_count_arr.entry(num2_idx) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            },
        }
    }
    
    fn count(&self, tot: i32) -> i32 {
        let nums1_len = self.nums1.len();
        let mut ans = 0;
        for i in 0..nums1_len {
            let num1 = self.nums1[i];
            if num1 >= tot {
                break;
            }
            let target = tot - num1;
 
            ans += match self.num2_count_arr.get(&target).cloned() {
                Some(v) => v,
                None => 0,
            };
        }
        ans as i32
    }
}