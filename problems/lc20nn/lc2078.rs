//! # LeetCode 2078. Two Furthest Houses With Different Colors
//! https://leetcode.com/problems/two-furthest-houses-with-different-colors/
//! - y2026m04d20; Independently Solved;

pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut color_iter = colors.into_iter().enumerate();
    let first_color = color_iter.next().expect("len >= 2").1;
    let mut second_color_info: Option<(usize, i32)> = None;

    let mut ans_dis = 0;

    for (i, color) in color_iter {
        if color == first_color {
            if let Some((second_color_idx, _)) = second_color_info.as_ref().cloned() {
                ans_dis = ans_dis.max(i - second_color_idx);
            }
        } else {
            ans_dis = i;
            if second_color_info.is_none() {
                second_color_info = Some((i, color));
            }
        }
        // println!("{}", ans_dis);
    }

    ans_dis as i32
}