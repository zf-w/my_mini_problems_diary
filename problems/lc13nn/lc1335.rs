//! ## Leetcode 1335. Minimum Difficulty of a Job Schedule
//! https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule
//! - `Hard`; `Learned from previous`; `2023-12-28`;
//! 
//! I'm kind of understanding the drift here. The "i" represents the start of a day's job, and "j" represents the end of a day's job. I guess the key might be getting more jobs done can only make the day more costly. But I don't fully understand how the "day" loop works.

pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
  let job_len = job_difficulty.len();
  if job_len < d as usize {
      return -1;
  }
  let mut dp: Vec<i32> = vec![20000; job_len + 1];
  dp[job_len] = 0;
  for day in 0..d as usize {
      for i in 0..job_len - day {
          let mut curr_max = 0;
          dp[i] = 20000;
          for j in i..job_len - day {
              curr_max = curr_max.max(job_difficulty[j]);
              dp[i] = dp[i].min(curr_max + dp[j + 1]);
          }
      }
  }
  *dp.first().unwrap()
}
