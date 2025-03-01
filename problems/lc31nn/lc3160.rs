//! # Leetcode 3160. Find the Number of Distinct Colors Among the Balls
//! https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls/
//! - `Medium`; `y2025m02d07`; `Independently Solved`; `29ms`; `13.9mb`; `5 attempts`;
//! Topic: `count_tracking`

use std::collections::hash_map::{Entry, HashMap};

pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ball_mark_hashmap: HashMap<usize, usize> = HashMap::new(); // I have misread the boundaries. I thought it was 1e5, but it was actually 1e9. hahaha.

    let mut color_count_hashmap: HashMap<usize, usize> = HashMap::new();

    let mut curr_unique_color_num = 0;

    let mut ans_vec: Vec<i32> = Vec::with_capacity(queries.len());

    for (ball_i, color_i) in queries
        .into_iter()
        .map(|v| -> (usize, usize) { (v[0] as usize, v[1] as usize - 1) })
    {
        let mut ball_color_i_entry = ball_mark_hashmap.entry(ball_i);

        if let Entry::Occupied(ref mut ball_color_i_occ_entry) = &mut ball_color_i_entry {
            let ball_color_i_mut_ref = ball_color_i_occ_entry.get_mut();
            let prev_color_count_entry_mut_ref = color_count_hashmap.entry(*ball_color_i_mut_ref);

            match prev_color_count_entry_mut_ref {
                Entry::Occupied(mut occupied_entry) => {
                    let prev_color_count_mut_ref = occupied_entry.get_mut();
                    *prev_color_count_mut_ref -= 1;
                    if *prev_color_count_mut_ref == 0 {
                        curr_unique_color_num -= 1;
                        // print!("-1 ");
                        occupied_entry.remove_entry();
                    }
                }
                Entry::Vacant(_) => {
                    // println!("{}", *ball_color_i_mut_ref);
                    unreachable!()
                }
            }
        }

        let curr_color_count_entry_mut_ref = color_count_hashmap.entry(color_i);

        match curr_color_count_entry_mut_ref {
            Entry::Occupied(mut occupied_entry) => {
                let curr_color_count_mut_ref = occupied_entry.get_mut();
                *curr_color_count_mut_ref += 1;
            }
            Entry::Vacant(vacant_entry) => {
                curr_unique_color_num += 1;
                vacant_entry.insert(1);
            }
        }
        // println!("");

        match ball_color_i_entry {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() = color_i;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(color_i);
            }
        }

        ans_vec.push(curr_unique_color_num);
    }

    ans_vec
}
