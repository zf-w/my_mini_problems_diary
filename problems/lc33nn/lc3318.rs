//! # Leetcode 3318. Find X-Sum of All K-Long Subarrays I
//! https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/
//! - `Easy`; `y2025m11d04`; `Independently Solved`; `0ms`; `2.2mb`; `2 attempts`;
//! Topics: ordered_set.

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    use std::{cmp::Reverse,collections::{hash_map::Entry, BTreeSet, HashMap}};
    type CountMap = HashMap<i32, usize>;
    type SortSet = BTreeSet<Reverse<(usize, i32)>>;

    let k = k as usize;
    let len = nums.len();

    let mut count_map: CountMap = HashMap::with_capacity(k);
    let mut sort_set: SortSet = SortSet::new();

    fn push_num(count_map_mut_ref: &mut CountMap, sort_set_mut_ref: &mut SortSet, num: i32) {
        match count_map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let val_mut_ref = occupied_entry.get_mut();

                sort_set_mut_ref.remove(&Reverse((*val_mut_ref, num)));

                *val_mut_ref += 1;

                sort_set_mut_ref.insert(Reverse((*val_mut_ref, num)));
            }
            Entry::Vacant(vacant_entry) => {
                sort_set_mut_ref.insert(Reverse((1, num)));
                vacant_entry.insert(1);
            }
        } //.or_insert(0)) += 1;
    }

    fn pop_num(count_map_mut_ref: &mut CountMap, sort_set_mut_ref: &mut SortSet, num: i32) {
        match count_map_mut_ref.entry(num) {
            Entry::Occupied(mut occupied_entry) => {
                let val_mut_ref = occupied_entry.get_mut();

                sort_set_mut_ref.remove(&Reverse((*val_mut_ref, num)));

                *val_mut_ref -= 1;

                if *val_mut_ref == 0 {
                    occupied_entry.remove();
                } else {
                    sort_set_mut_ref.insert(Reverse((*val_mut_ref, num)));
                }
            }
            Entry::Vacant(_) => {
                unreachable!()
            }
        } //.or_insert(0)) += 1;
    }

    fn get_sum(sort_set_ref: &SortSet, x: i32) -> i64 {
        sort_set_ref.into_iter().take(x as usize).fold(
            0,
            |ans_count, Reverse((num_count_ref, num_ref))| -> i64 {
                // println!("{} {}", *num_ref, *num_count_ref);
                ans_count + ((*num_ref) as i64 * ((*num_count_ref) as i64))
            },
        )
    }

    let mut num_iter = nums.iter().cloned();
    let mut i: usize = 0;

    if i < k - 1 {
        while let Some(num) = num_iter.next() {
            push_num(&mut count_map, &mut sort_set, num);
            i += 1;
            if i >= k - 1 {
                break;
            }
        }
    }

    let mut ans_vec = Vec::with_capacity(len - k + 1);

    let pop_num_iter = nums.iter().cloned();

    for (num, to_pop_num) in num_iter.zip(pop_num_iter) {
        push_num(&mut count_map, &mut sort_set, num);
        ans_vec.push(get_sum(&sort_set, x));
        pop_num(&mut count_map, &mut sort_set, to_pop_num);
    }

    ans_vec
}