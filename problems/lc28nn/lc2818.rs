//! # Leetcode 2818. Apply Operations to Maximize Score
//! https://leetcode.com/problems/apply-operations-to-maximize-score/
//! - `Hard`; `y2025m03d31`; `Learned from Solution`; `319ms`; `7.6mb`; `1 attempt`;
//! Topics: uncategorized.
//!
//! Learned from https://leetcode.com/problems/apply-operations-to-maximize-score/solutions/6512908/apply-operations-to-maximize-score

pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::{BinaryHeap, VecDeque};
    const MODULOS_NUM: i64 = 1000000007;

    let mut k: i64 = k as i64;

    fn quick_pow(mut base_num: i64, mut exponent_num: i64) -> i64 {
        let mut ans_num = 1;
        while exponent_num > 0 {
            if exponent_num & 1 == 1 {
                ans_num = (ans_num * base_num) % MODULOS_NUM;
            }
            base_num = (base_num * base_num) % MODULOS_NUM;
            exponent_num >>= 1;
        }

        ans_num
    }

    let len = nums.len();

    let mut prime_score_vec: Vec<i32> = vec![0; len];

    for (i, mut num) in nums.iter().cloned().enumerate() {
        let prime_score_entry_mut_ref = &mut prime_score_vec[i];
        for factor in 2..=((num as f32).sqrt()) as i32 {
            if num % factor != 0 {
                continue;
            }

            *prime_score_entry_mut_ref += 1;

            while num % factor == 0 {
                num /= factor;
            }
        }

        if num >= 2 {
            *prime_score_entry_mut_ref += 1;
        }
    }

    let mut next_dominant_vec: Vec<usize> = vec![len; len];
    let mut prev_dominant_vec: Vec<usize> = vec![len; len];

    let mut decreasing_prime_score_idx_stack: VecDeque<usize> = VecDeque::with_capacity(len);

    for (i, curr_prime_score) in prime_score_vec.iter().cloned().enumerate() {
        while decreasing_prime_score_idx_stack.is_empty() == false
            && prime_score_vec[*decreasing_prime_score_idx_stack
                .back()
                .expect("len checked")]
                < curr_prime_score
        {
            let top_idx = decreasing_prime_score_idx_stack
                .pop_back()
                .expect("len checked");
            next_dominant_vec[top_idx] = i;
        }

        if let Some(top_idx) = decreasing_prime_score_idx_stack.back().cloned() {
            prev_dominant_vec[i] = top_idx;
        }

        decreasing_prime_score_idx_stack.push_back(i);
    }

    // println!("{:?}", prime_score_vec);
    // println!("prev: {:?}", prev_dominant_vec);
    // println!("next: {:?}", next_dominant_vec);

    let mut subarray_num_vec: Vec<usize> = Vec::with_capacity(len);

    for i in 0..len {
        let prev_dominant_idx = prev_dominant_vec[i];
        subarray_num_vec.push(
            (next_dominant_vec[i] - i)
                * (if prev_dominant_idx == len {
                    i + 1
                } else {
                    i - prev_dominant_idx
                }),
        );
    }

    let mut processing_pq: BinaryHeap<(i32, usize)> = BinaryHeap::with_capacity(len);

    for (i, num) in nums.iter().cloned().enumerate() {
        processing_pq.push((num, i));
    }

    let mut ans_score: i64 = 1;

    while k > 0 {
        let (num, i) = processing_pq.pop().expect("should work");

        let oper_num: i64 = k.min(subarray_num_vec[i] as i64);

        ans_score = (ans_score * quick_pow(num as i64, oper_num)) % MODULOS_NUM;

        k -= oper_num;
    }

    ans_score as i32
}
