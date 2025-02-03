//! ## Leetcode 2461. Maximum Sum of Distinct Subarrays With Length K
//! https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
//! - `Medium`; `y2024m11d18`; `Independently Solved`; `15ms`; `6.2mb`; `1 attempt`;

pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k_usize = k as usize;
    let mut elem_map: std::collections::HashMap<i32, usize> =
        std::collections::HashMap::with_capacity(k_usize);
    let mut end_iter = nums.iter().cloned();
    let mut begin_iter = nums.iter().cloned();
    let mut sum_i64 = 0;
    let mut duplicate_usize: usize = 0;

    #[inline]
    fn add_elem(
        curr_elem: i32,
        sum_i64_mut_ref: &mut i64,
        duplicate_usize_mut_ref: &mut usize,
        map_mut_ref: &mut std::collections::HashMap<i32, usize>,
    ) {
        let entry = map_mut_ref.entry(curr_elem);
        match entry {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();
                if *count_mut_ref == 1 {
                    *duplicate_usize_mut_ref += 1;
                }
                *count_mut_ref += 1;
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
        *sum_i64_mut_ref += curr_elem as i64;
    }

    for _ in 0..k_usize {
        let curr_elem = end_iter.next().expect("k < len");
        add_elem(curr_elem, &mut sum_i64, &mut duplicate_usize, &mut elem_map);
    }
    let mut ans_sum = 0;
    if duplicate_usize == 0 {
        ans_sum = sum_i64;
    }
    for (to_pop_elem, curr_elem) in begin_iter.zip(end_iter) {
        sum_i64 -= to_pop_elem as i64;
        let entry = elem_map.entry(to_pop_elem);
        match entry {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                let count_mut_ref = occupied_entry.get_mut();
                if *count_mut_ref == 2 {
                    duplicate_usize -= 1;
                }
                *count_mut_ref -= 1;
                if *count_mut_ref == 0 {
                    occupied_entry.remove_entry();
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                unreachable!()
            }
        }

        add_elem(curr_elem, &mut sum_i64, &mut duplicate_usize, &mut elem_map);
        if duplicate_usize == 0 {
            ans_sum = ans_sum.max(sum_i64);
        }
    }
    ans_sum
}
