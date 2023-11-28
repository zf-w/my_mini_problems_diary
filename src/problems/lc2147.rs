//! ## Leetcode 2147. Number of Ways to Divide a Long Corridor
//! - `Hard`
//! 
//! Along a Along a long library corridor, there is a line of seats and decorative plants. You are given a **0-indexed** string corridor of length `n` consisting of letters `'S'` and `'P'` where each `'S'` represents a seat and each `'P'` represents a plant.
//! One room divider has **already** been installed to the left of index 0, and **another** to the right of index `n - 1`. Additional room dividers can be installed. For each position between indices i - 1 and i (1 <= i <= n - 1), at most one divider can be installed.
//! Divide the corridor into non-overlapping sections, where each section has **exactly two seats** with any number of plants. There may be multiple ways to perform the division. Two ways are **different** if there is a position with a room divider installed in the first way but not in the second way.
//! Return the number of ways to divide the corridor. Since the answer may be very large, return it **modulo** `10^9 + 7`. If there is no way, return `0`.
//! 
//! ![Corridor Example](https://assets.leetcode.com/uploads/2021/12/04/1.png)
//! 
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-27`;
//! 
//! The only way you can make decisions about where to put the room dividers is the space between the second seat of the previous room and the first seat of the current room. I used this intuition to solve the problem.
//! 

pub fn number_of_ways(corridor: String) -> i32 {
    let mut prev_seat: usize = 0;
    let mut seat_count: usize = 0;
    let mut ans: i32 = 1;
    let m: usize = 1000000007;
    for (i,c) in corridor.char_indices() {
        match c {
        'S' => {
            seat_count += 1;
            if seat_count > 2 && seat_count & 1 == 1 {
                ans = ((ans as usize * (i - prev_seat)) % m) as i32;
            }
            
            prev_seat = i;
        },
        _ => ()
        }
    }
    
    if seat_count % 2 == 1 || seat_count == 0 {
        0
    } else {
        ans
    }
}