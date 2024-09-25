//! Leetcode 1688. Count of Matches in Tournament
//! https://leetcode.com/problems/count-of-matches-in-tournament
//! - `Easy`; `Independently Solved`; `2023-12-04`;
//! 
//! I guess 'removing' a team needs one match. So, to pick one winner, you need to remove all the other teams. 

pub fn number_of_matches(n: i32) -> i32 {
    n - 1
}