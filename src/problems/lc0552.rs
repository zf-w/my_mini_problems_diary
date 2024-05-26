//! ## Leetcode 552. Student Attendance Record II
//! https://leetcode.com/problems/student-attendance-record-ii
//! - `Hard`; `Independently Solved`; `2024-05-26`;
//!
//! We can solve this problem using Dynamic Programming. Since there can be at most one `A` in a valid record, we can fairly easily calculate them with recursive relation like "some valid `P` and `L` records + `A` + some valid `P` and `L` records." We can ignore it first and only think about valid records with only `L` and `P`. When appending a new letter to a set of valid `PL` records, if the new letter is `P`, the new record will always be valid. If the new letter is `L`, only the previous records with two ending `L`s will become invalid. With that relationship, we can solve the problem with Dynamic Programming.

const MOD: u64 = 1000000007;

pub fn check_record(n: i32) -> i32 {
    let len = n as usize;
    let mut dp: Vec<u64> = vec![0; len + 1];
    dp[0] = 1;
    dp[1] = 2;
    dp[2] = 4;
    dp[3] = 7;
    for i in 4..=len {
        dp[i] = (dp[i - 1] * 2) % MOD;
        dp[i] = (dp[i] - dp[i - 4]) % MOD;
    }
    let mut ans: u64 = dp[len];
    for i in 0..len {
        ans = (ans + (dp[i] * dp[len - i - 1] % MOD)) % MOD;
    }
    ans as i32
}
