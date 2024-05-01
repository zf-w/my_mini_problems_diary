//! ## Leetcode 1915. Number of Wonderful Substrings
//! https://leetcode.com/problems/number-of-wonderful-substrings
//! - `Medium`; `Learned from Solution`; `2024-04-30`;
//!
//! The core of solving this problem is to use bits and XOR to represent the occurrence of previous letters. But I don't understand yet how the prefix xor works. In my TLE solution, I iterate through all the previous occurence states up to the current char and create the new occurence states. I think the large constant in its time complexity led to time limit being exceeded.

pub fn my_wonderful_substrings(word: String) -> i64 {
    let mut dp: [i64; 2048] = [0; 2048];
    let index = |curr_part: bool, i: usize| -> usize {
        if curr_part {
            1024 + i
        } else {
            i
        }
    };
    fn char_to_mask(c: char) -> usize {
        let a_base = 'a' as u8;
        (1 << c as u8 - a_base) as usize
    }
    fn one_odd(mask: usize) -> bool {
        mask & (mask - 1) == 0
    }
    let mut ans: i64 = 0;
    let mut curr_part = true;
    for c in word.chars() {
        let curr_mask = char_to_mask(c);
        dp[index(curr_part, curr_mask)] = dp[index(!curr_part, 0)] + 1;
        ans += dp[index(curr_part, curr_mask)];
        for dp_mask in 1..1024usize {
            let next_mask = dp_mask ^ curr_mask;
            let curr_dp = dp[index(!curr_part, dp_mask)];
            if next_mask == 0 || one_odd(next_mask) {
                ans += curr_dp;
            }
            dp[index(curr_part, next_mask)] = curr_dp;
        }
        for dp_mask in 0..1024usize {
            dp[index(!curr_part, dp_mask)] = 0;
        }
        curr_part = !curr_part;
    }
    ans
}

pub fn wonderful_substrings(word: String) -> i64 {
    let mut dp: [i64; 1024] = [0; 1024];

    fn char_to_mask(c: char) -> usize {
        let a_base = 'a' as u8;
        (1 << c as u8 - a_base) as usize
    }

    dp[0] = 1;
    let mut ans: i64 = 0;
    let mut prefix_mask = 0;
    for c in word.chars() {
        let curr_mask = char_to_mask(c);
        prefix_mask ^= curr_mask;
        ans += dp[prefix_mask];

        for i in 0..10 {
            let next_mask = prefix_mask ^ (1usize << i);
            ans += dp[next_mask];
        }
        dp[prefix_mask] += 1;
    }
    ans
}
