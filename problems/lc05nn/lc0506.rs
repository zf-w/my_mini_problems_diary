//! ## Leetcode 506. Relative Ranks
//! https://leetcode.com/problems/relative-ranks
//! - `Easy`; `Independently Solved`; `2024-05-11`;
//!
//! The problem is about sorting with indices. Interestingly, the score (1e5 maximum) is only using 17 bits, and the index is only going to use 14 bits. This means the score is not going to flip to the negative side for 32-bit signed integers. More interestingly, later, I checked the constraints and found that the score's maximum is 1e6 instead of 1e5. This solution will not work for that, but it's still very interesting to think about it.

pub fn find_relative_ranks(mut score: Vec<i32>) -> Vec<String> {
    let len = score.len();
    let mut ans: Vec<String> = Vec::with_capacity(len);
    for _ in 0..len {
        ans.push(String::new());
    }
    let mask: i32 = (1 << 14) - 1; // Need () here!!!
    for (i, sc) in score.iter_mut().enumerate() {
        *sc <<= 14;
        *sc |= i as i32;
        // println!("{} {:b}", *sc & mask, i);
    }
    score.sort();
    for (i, info) in score.iter().rev().enumerate() {
        let player_i = (info & mask) as usize;
        ans[player_i].clone_from(&match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            i => (i + 1).to_string(),
        })
    }

    ans
}
