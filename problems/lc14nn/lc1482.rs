//! ## Leetcode 1482. Minimum Number of Days to Make m Bouquets
//! https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
//! - `Medium`; `Independently Solved`; `2024-06-19`;
//!
//! We can solve this question using Binary Search. We can search on the minimum days needed and check whether we can gather enough flowers. I guess, when a question is asking about minimum or maximum value of something that is easy to check, Binary Search would be a good choice.

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    fn is_ok(bloom_day: &Vec<i32>, day: i32, m: i32, k: i32) -> bool {
        let mut count = 0;
        let mut continous_count = 0;
        for is_bloom in bloom_day.iter().map(|v| -> bool { v <= &day }) {
            if is_bloom {
                continous_count += 1;
                if continous_count >= k {
                    count += 1;
                    continous_count = 0;
                }
            } else {
                continous_count = 0;
            }
            if count >= m {
                return true;
            }
        }
        false
    }
    let (mut begin_day, mut end_day) = bloom_day
        .iter()
        .fold((i32::MAX, 0), |(curr_min, curr_max), v| {
            (curr_min.min(*v), curr_max.max(*v))
        });
    end_day += 1;
    let mut ans_min_days: i32 = -1;
    while begin_day < end_day {
        let mid_day = (begin_day + end_day) / 2;
        if is_ok(&bloom_day, mid_day, m, k) {
            if ans_min_days < 0 {
                ans_min_days = mid_day;
            } else {
                ans_min_days = ans_min_days.min(mid_day);
            }
            end_day = mid_day;
        } else {
            begin_day = mid_day + 1;
        }
    }
    ans_min_days
}
