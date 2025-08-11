//! # Leetcode 2438. Range Product Queries of Powers
//! https://leetcode.com/problems/range-product-queries-of-powers/
//! - `Medium`; `y2025m08d10`; `Independently Solved`; `26ms`; `8.8mb`; `3 attempts`;
//! Topics: uncategorized.

pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pow_exp_vec: Vec<i32> = Vec::with_capacity(32);

    let mut i = 0;
    while n > 0 {
        if n & 1 == 1 {
            pow_exp_vec.push(i);
        }

        n >>= 1;
        i += 1;
    }

    let mut ans_vec: Vec<i32> = Vec::with_capacity(queries.len());

    for query in queries {
        let begin_i = query[0];
        let end_i = query[1] + 1;

        let mut ans: i64 = 1;

        for pow_i in begin_i..end_i {
            let pow_exp = pow_exp_vec[pow_i as usize];
            ans = (ans * (1 << pow_exp) as i64) % 1000000007;
        }

        ans_vec.push(ans as i32);
    }

    ans_vec
}