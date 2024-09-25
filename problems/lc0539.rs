//! ## Leetcode 539. Minimum Time Difference
//! https://leetcode.com/problems/minimum-time-difference/
//! - `Medium`; `Learned from Solution`; `2024-09`;
//!
//! Learned from: https://leetcode.com/problems/minimum-time-difference/solutions/5792274/1440-mins-in-1-day-explained-with-video-c-java-python-js-count-total-minutes-explained

pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut times_vec: Vec<u16> = Vec::with_capacity(time_points.len());
    #[inline]
    fn time_point_to_time_num(time_point: String) -> u16 {
        let mut time_string_chars_iter = time_point.chars();
        let mut ans_minutes_num: u16 = 0;
        ans_minutes_num += time_string_chars_iter
            .next()
            .expect("Should have a ten hour digit") as u16
            * 10
            * 60;
        ans_minutes_num += time_string_chars_iter
            .next()
            .expect("Should have a hour digit") as u16
            * 60;
        time_string_chars_iter.next();
        ans_minutes_num += time_string_chars_iter
            .next()
            .expect("Should have a ten minutes digit") as u16
            * 10;
        ans_minutes_num += time_string_chars_iter
            .next()
            .expect("Should have a minute digit") as u16;
        ans_minutes_num
    }
    for time_point in time_points {
        times_vec.push(time_point_to_time_num(time_point));
    }
    times_vec.sort_unstable();
    let mut next_elem_iter = times_vec.iter();
    next_elem_iter.next();
    let pair_iter = times_vec.iter().zip(next_elem_iter);
    pair_iter.fold(
        1440 - times_vec.last().expect("Have >= 2 elem")
            + times_vec.first().expect("First, have >= 2 elem"),
        |min_dis: u16, (curr_time_ref, next_time_ref)| -> u16 {
            min_dis.min(next_time_ref - curr_time_ref)
        },
    ) as i32
}
