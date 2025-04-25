//! # Leetcode 2799. Count Complete Subarrays in an Array
//! https://leetcode.com/problems/count-complete-subarrays-in-an-array/
//! - `Medium`; `y2025m04d24`; `Independently Solved`; `1ms`; `2.3mb`; `1 attempt`;
//! Topics: sliding_window.

pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    use std::collections::{hash_map::Entry, HashMap};
    let len = nums.len();
    let mut count_map: HashMap<i32, usize> = HashMap::with_capacity(len);

    for num in nums.iter().cloned() {
        count_map.insert(num, 0);
    }

    let unique_num = count_map.len();

    let mut begin_iter = nums.iter().cloned();
    let end_iter = nums.iter().cloned().enumerate();

    let mut begin_i: usize = 0;
    // let mut end_i: usize = 0;

    fn push(count_map_mut_ref: &mut HashMap<i32, usize>, unique_num_mut_ref: &mut usize, num: i32) {
        match count_map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();

                if *count_mut_ref == 0 {
                    *unique_num_mut_ref += 1;
                }

                *count_mut_ref += 1;
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }

    fn pop(count_map_mut_ref: &mut HashMap<i32, usize>, unique_num_mut_ref: &mut usize, num: i32) {
        match count_map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();

                *count_mut_ref -= 1;

                if *count_mut_ref == 0 {
                    *unique_num_mut_ref -= 1;
                }
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }

    let mut curr_unique_num: usize = 0;

    let mut ans_count: usize = 0;

    for (end_i, num) in end_iter {
        push(&mut count_map, &mut curr_unique_num, num);

        let satisfy_flag = curr_unique_num == unique_num;

        // println!("{}", end_i);
        while curr_unique_num == unique_num && begin_i <= end_i {
            let begin_num = begin_iter.next().expect("begin < end");
            begin_i += 1;
            ans_count += len - end_i;
            pop(&mut count_map, &mut curr_unique_num, begin_num);
        }

        // println!("{} {:?}", begin_i, satisfy_flag);

        // if satisfy_flag == true {
        //     // ans_count += begin_i;
        // }
    }

    ans_count as i32
}
