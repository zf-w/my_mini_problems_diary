//! # Leetcode 3477. Fruits Into Baskets II
//! https://leetcode.com/problems/fruits-into-baskets-ii/
//! - `Easy`; `y2025m08d05`; `Independently Solved`; `3ms`; `2.2mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
    let mut ans_unplaced_num = 0;
    for fruit in fruits {
        let mut found_flag = false;
        for basket_mut_ref in baskets.iter_mut() {
            if *basket_mut_ref >= fruit {
                *basket_mut_ref = 0;
                found_flag = true;
                break;
            }
        }

        if found_flag == false {
            ans_unplaced_num += 1;
        }
    }
    ans_unplaced_num
}