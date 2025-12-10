//! # Leetcode 3578. Count Partitions With Max-Min Difference at Most K
//! https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/

pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BTreeSet;

    const MODULUS: i64 = 1_000_000_007;

    let i32_add_with_modulus_closure =
        |a: i32, b: i32| -> i32 { (((a as i64) + (b as i64)) % MODULUS) as i32 };

    let i32_sub_with_modulus_closure =
        |a: i32, b: i32| -> i32 { i32_add_with_modulus_closure(a, (MODULUS as i32) - b) };

    let len = nums.len();
    let mut dp_arr_box = vec![0; len + 1];

    dp_arr_box[0] = 1;

    let mut num_begin_iter = nums.iter().cloned();
    let num_end_iter = nums.iter().cloned().enumerate();

    let mut range_tracking_btree_set: BTreeSet<i32> = BTreeSet::new();

    fn get_range(s_ref: &BTreeSet<i32>) -> i32 {
        if s_ref.is_empty() == true {
            return 0;
        }

        s_ref.last().unwrap() - s_ref.first().unwrap()
    }

    let mut begin_i: usize = 0;

    for (end_i, receiving_num) in num_end_iter {
        range_tracking_btree_set.insert(receiving_num);

        while begin_i < end_i && get_range(&range_tracking_btree_set) > k {
            let parting_num = num_begin_iter.next().expect("Should be having a num.");
            range_tracking_btree_set.remove(&parting_num);
            begin_i += 1;
        }

        let dp_entry_i = end_i + 1;

        dp_arr_box[dp_entry_i] = i32_add_with_modulus_closure(
            i32_add_with_modulus_closure(
                dp_arr_box[dp_entry_i - 1],
                i32_sub_with_modulus_closure(dp_arr_box[end_i], dp_arr_box[begin_i]),
            ),
            dp_arr_box[dp_entry_i],
        )
    }

    i32_sub_with_modulus_closure(dp_arr_box[len], dp_arr_box[len - 1])
}
