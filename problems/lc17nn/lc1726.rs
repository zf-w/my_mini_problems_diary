//! # Leetcode 1726. Tuple with Same Product
//! https://leetcode.com/problems/tuple-with-same-product/
//! - `Medium`; `y2025m02d06`; `Independently Solved`; `43ms`; `12.4mb`; `1 attempt`;

pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let nums_len = nums.len();
    use std::collections::HashMap;
    let mut hash_map: HashMap<i32, i32> = HashMap::with_capacity(nums_len * nums_len / 2);

    let mut num_iter_0 = nums.iter().cloned();
    while let Some(num_0) = num_iter_0.next() {
        let mut num_iter_1 = num_iter_0.clone();
        // let _ = num_iter_1.next();

        for num_1 in num_iter_1 {
            // println!("{} {}", num_0, num_1);
            match hash_map.entry(num_0 * num_1) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    *occupied_entry.get_mut() += 1
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }
    }

    let mut ans_count = 0;

    for (_, count) in hash_map {
        ans_count += count * (count - 1) * 4;
    }
    ans_count
}
