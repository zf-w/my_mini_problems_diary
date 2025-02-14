//! ## Leetcode 1395. Count Number of Teams
//! https://leetcode.com/problems/count-number-of-teams/
//! - `Medium`; `Independently Solved`; `2024-07-29`;

pub fn num_teams(rating: Vec<i32>) -> i32 {
    let len = rating.len();
    let mut dp: Vec<(usize, usize)> = vec![(0, 0); len];
    let mut ans_total = 0;
    for (curr_i, curr_rating) in rating.iter().cloned().enumerate() {
        let (mut curr_g, mut curr_l) = (0, 0);
        for (prev_rating, (prev_g, prev_l)) in
            rating[..curr_i].iter().cloned().zip(dp.iter().cloned())
        {
            if prev_rating < curr_rating {
                curr_l += 1;
                ans_total += prev_l;
            } else if prev_rating > curr_rating {
                curr_g += 1;
                ans_total += prev_g;
            } else {
                unreachable!()
            }
        }
        dp[curr_i] = (curr_g, curr_l);
    }
    ans_total as i32
}
