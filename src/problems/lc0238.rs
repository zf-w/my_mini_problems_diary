//! Leetcode 238. Product of Array Except Self
//! https://leetcode.com/problems/product-of-array-except-self
//! - `Medium`; `Independently Solved`; `2024-03-15`;
//!
//! An interesting question. Hinted by the range of numbers in the array.

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ans: Vec<i32> = Vec::with_capacity(len);
    let mut counts: [usize; 61] = [0; 61];
    let mut multi0: [i32; 61] = [0; 61];
    let mut multi1: [i32; 61] = [0; 61];

    let index = |num: &i32| -> usize { (num + 30) as usize };

    for num in nums.iter() {
        counts[index(num)] += 1;
    }

    for (i, count) in counts.iter().enumerate() {
        let m = count.clone() as u32;
        multi0[i] = ((i - 30) as i32).pow(m);
        multi1[i] = ((i - 30) as i32).pow(m - 1);
    }

    let mut memo: [Option<i32>; 61] = [None; 61];

    let mut calculate = |num: &i32| -> i32 {
        let curr_i = index(num);
        if let Some(v) = memo[curr_i] {
            return v;
        }
        let mut prod = 1;
        for i in 0..61 {
            if curr_i != i {
                prod *= multi0[i];
            } else {
                prod *= multi1[i];
            }
        }
        memo[curr_i] = Some(prod);
        prod
    };

    for num in nums.iter() {
        ans.push(calculate(num));
    }
    ans
}
