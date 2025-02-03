/// ## Leetcode 2563. Count the Number of Fair Pairs
/// https://leetcode.com/problems/count-the-number-of-fair-pairs/
/// - `Medium`; `Independently Solved`; `y2024m11d14`;
///
/// I guess this might be quicker if we consider the duplication of pairs. Maybe the begin and end indices can be further "pruned".
pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    // [0,1,4,4,5,7]
    // [1,2,5,5,6,8]
    // [4,5,8,8,9,11]
    // [4,5,8,8,9,11]
    // [5,6,9,9,10,11]
    // [7,8,11,11,12,14]
    nums.sort_unstable();
    let len = nums.len();
    let mut ans_count: usize = 0;
    let mut prev_lower_i = len;
    let mut prev_upper_i = len;
    for num_ref in nums.iter() {
        let curr_lower = lower - num_ref;
        let curr_upper = upper - num_ref;
        let mut begin_i = 0;
        let mut end_i = prev_lower_i;
        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            if nums[mid_i] >= curr_lower {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }
        let lower_i = begin_i;
        prev_lower_i = lower_i;
        let mut begin_i = prev_lower_i;
        let mut end_i = prev_upper_i;

        while begin_i < end_i {
            let mid_i = (begin_i + end_i) / 2;
            if nums[mid_i] > curr_upper {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }
        let upper_i = begin_i;
        prev_upper_i = upper_i;
        ans_count += upper_i - lower_i;
        let num_square = num_ref + num_ref;
        if num_square >= lower && num_square <= upper {
            ans_count -= 1;
        }
    }
    ans_count as i64 / 2
}
