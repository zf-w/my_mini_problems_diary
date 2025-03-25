//! # Leetcode 3169. Count Days Without Meetings
//! https://leetcode.com/problems/count-days-without-meetings/
//! - `Medium`; `y2025m03d23`; `Independently Solved`; `35ms`; `9.5mb`; `2 attempts`;
//! Topics: intervals.

pub fn count_days(mut days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable();

    let mut prev_end = 0;

    for meeting in meetings {
        let begin = meeting[0];
        let end = meeting[1] + 1;
        // println!("{} {} {} {}", begin, end, prev_end, days);
        if end > prev_end {
            // Forgot this
            days -= end - prev_end.max(begin);
        }
        prev_end = prev_end.max(end); // And this
    }

    days
}
