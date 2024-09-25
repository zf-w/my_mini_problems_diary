//! ## Count Triplets That Can Form Two Arrays of Equal XOR
//! https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
//! - `Medium`; `Independently Solved`; `2024-05-30`
//!
//! With XOR's properties, we can use the prefix sum trick to quickly calculate the XOR sum of a given slice of elements.

pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut pre_sum: Vec<i32> = Vec::with_capacity(len + 1);
    pre_sum.push(0);
    let mut prev = 0;
    for num in arr.iter() {
        prev ^= *num;
        pre_sum.push(prev);
    }

    fn is_i_j_k_ok(pre_sum: &mut Vec<i32>, i: usize, j: usize, k: usize) -> bool {
        pre_sum[i] ^ pre_sum[j] == pre_sum[j + 1] ^ pre_sum[k]
    }

    let mut ans_count: i32 = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            for k in j..=len {
                if is_i_j_k_ok(&mut pre_sum, i, j, k) {
                    ans_count += 1
                }
            }
        }
    }
    ans_count
}
