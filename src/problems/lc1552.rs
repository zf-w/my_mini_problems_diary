//! ## Leetcode 1552. Magnetic Force Between Two Balls
//! https://leetcode.com/problems/magnetic-force-between-two-balls/
//! - `Medium`; `Independently Solved`; `2024-06-19`;
//!
//! We can solve this question using Binary Search. We can search on the minimum distance between two objects and check whether a minimum distance works. I guess, when a question is asking about minimum or maximum value of something that is easy to check, Binary Search would be a good choice.

pub fn max_distance(mut positions: Vec<i32>, m: i32) -> i32 {
    positions.sort_unstable();
    fn is_ok(positions: &Vec<i32>, m: i32, min_dis: i32) -> bool {
        let mut positions_iter = positions.iter().copied();
        let mut prev_position = positions_iter.next().expect("At least 2");

        let mut count = 0;

        for curr_position in positions_iter {
            if curr_position - prev_position >= min_dis {
                count += 1;
                prev_position = curr_position;
                if count >= m {
                    return true;
                }
            }
        }
        false
    }
    let (mut begin_ans, mut end_ans) = (
        1,
        (positions.last().expect("At least 2") - positions[0]) / (m - 1),
    );
    end_ans += 1;
    let mut ans_max: i32 = -1;
    while begin_ans < end_ans {
        let mid_ans = (begin_ans + end_ans) / 2;
        if is_ok(&positions, m, mid_ans) {
            if ans_max < 0 {
                ans_max = mid_ans;
            } else {
                ans_max = ans_max.max(mid_ans);
            }
            begin_ans = mid_ans + 1;
        } else {
            end_ans = mid_ans;
        }
    }
    ans_max
}
