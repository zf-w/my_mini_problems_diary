//! ## Leetcode 1052. Grumpy Bookstore Owner
//! https://leetcode.com/problems/grumpy-bookstore-owner/
//! - `Medium`; `Independently Solved`; `2024-06-21`;
//!
//! We can iterate through the times and see which groups of minutes can boost the number of satisfied customers the most.

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut default_satisfied = 0;
    let is_grumpy_map = |v: &i32| -> bool { *v > 0 };
    let mut curr_iter = customers
        .iter()
        .copied()
        .zip(grumpy.iter().map(is_grumpy_map));
    let prev_iter = customers
        .iter()
        .copied()
        .zip(grumpy.iter().map(is_grumpy_map));
    let mut curr_more_satisfied = 0;
    for _ in 0..minutes {
        let (curr_customer_num, is_grumpy) = curr_iter
            .next()
            .expect("Minutes less than customers'length");
        if is_grumpy {
            curr_more_satisfied += curr_customer_num;
        } else {
            default_satisfied += curr_customer_num;
        }
    }
    let mut max_more_satisfied = curr_more_satisfied;
    for ((prev_customer_num, prev_is_grumpy), (curr_customer_num, curr_is_grumpy)) in
        prev_iter.zip(curr_iter)
    {
        if prev_is_grumpy {
            curr_more_satisfied -= prev_customer_num;
        }
        if curr_is_grumpy {
            curr_more_satisfied += curr_customer_num;
        } else {
            default_satisfied += curr_customer_num;
        }
        max_more_satisfied = max_more_satisfied.max(curr_more_satisfied);
    }
    default_satisfied + max_more_satisfied
}
