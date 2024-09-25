//! ## Leetcode 376. Wiggle Subsequence
//! https://leetcode.com/problems/wiggle-subsequence/
//! - `Medium`; `Independently Solved`; `2024-06-19`;
//!
//! When I saw "subsequence," I thought about Dynamic Programming and Induction. I guess we can solve this using a more intuitive approach of counting special peaks.
//!

pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut dp: Vec<(usize, usize)> = Vec::with_capacity(len);
    let mut nums_iter = nums.iter().copied().enumerate();
    if nums_iter.next().is_none() {
        unreachable!();
    };
    dp.push((1, 1));

    let mut ans_len = 1;
    for (curr_i, curr_num) in nums_iter {
        let mut curr_dp = (1, 1);
        for prev_i in 0..curr_i {
            let prev_num = nums[prev_i];
            if prev_num < curr_num {
                curr_dp.0 = curr_dp.0.max(dp[prev_i].1 + 1);
                ans_len = ans_len.max(curr_dp.0);
            } else if prev_num > curr_num {
                curr_dp.1 = curr_dp.1.max(dp[prev_i].0 + 1);
                ans_len = ans_len.max(curr_dp.1);
            }
        }
        dp.push(curr_dp);
    }
    ans_len as i32
}

pub fn wiggle_max_length_v1(nums: Vec<i32>) -> i32 {
    let mut nums_iter = nums.iter().copied();
    let mut prev_pivot = if let Some(num) = nums_iter.next() {
        num
    } else {
        unreachable!();
    };

    let mut ans_len = 1;
    let mut prev_num: i32 = if let Some(prev_num) = nums_iter.next() {
        if prev_num != prev_pivot {
            ans_len += 1;
        }
        prev_num
    } else {
        return ans_len;
    };
    for curr_num in nums_iter {
        if (curr_num < prev_num && prev_num >= prev_pivot)
            || (curr_num > prev_num && prev_num <= prev_pivot)
        {
            prev_pivot = prev_num;
            ans_len += 1;
        }
        prev_num = curr_num;
    }
    ans_len as i32
}

#[test]
fn check_3_3_3_2_5() {
    assert_eq!(wiggle_max_length_v1(vec![3, 3, 3, 2, 5]), 3)
}

#[test]
fn check_26() {
    assert_eq!(
        wiggle_max_length_v1(vec![
            51, 226, 208, 165, 202, 286, 190, 212, 219, 271, 36, 245, 20, 238, 238, 89, 105, 66,
            73, 9, 254, 206, 221, 237, 203, 33, 249, 253, 150, 102, 57, 249, 203, 10, 123, 178, 85,
            203, 35, 276, 129, 116, 37, 163, 99, 142, 187, 249, 134, 77, 217, 298, 29, 127, 174,
            115, 122, 178, 12, 80, 122, 76, 16, 41, 115, 84, 104, 121, 127, 40, 287, 129, 9, 172,
            112, 51, 40, 135, 205, 53, 259, 196, 248, 5, 123, 184, 209, 130, 271, 42, 18, 143, 24,
            101, 10, 273, 252, 50, 173, 80, 119, 129, 45, 257, 299, 78, 278, 78, 190, 215, 284,
            129, 200, 232, 103, 97, 167, 120, 49, 298, 141, 146, 154, 233, 214, 196, 244, 50, 110,
            48, 152, 82, 226, 26, 254, 276, 292, 286, 215, 56, 128, 122, 82, 241, 222, 12, 272,
            192, 224, 136, 116, 70, 39, 207, 295, 49, 194, 90, 210, 123, 271, 18, 276, 87, 177,
            240, 276, 33, 155, 200, 51, 6, 212, 36, 149, 202, 48, 114, 58, 91, 83, 221, 175, 148,
            278, 300, 284, 86, 191, 95, 77, 215, 113, 257, 153, 135, 217, 76, 85, 269, 126, 194,
            199, 195, 20, 204, 194, 50, 220, 228, 90, 221, 256, 87, 157, 246, 74, 156, 9, 196, 16,
            259, 234, 79, 31, 206, 148, 12, 223, 151, 96, 229, 165, 9, 144, 26, 255, 201, 33, 89,
            145, 155, 1, 204, 37, 107, 80, 212, 88, 186, 254, 9, 158, 180, 24, 45, 158, 100, 52,
            131, 71, 174, 229, 236, 296, 299, 184, 168, 41, 45, 76, 68, 122, 85, 292, 238, 293,
            179, 143, 128, 47, 87, 267, 53, 187, 76, 292, 0, 160, 70, 172, 292, 9, 64, 156, 153,
            26, 145, 196, 222
        ]),
        202
    )
}
