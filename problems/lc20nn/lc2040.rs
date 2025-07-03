//! # Leetcode 2040. Kth Smallest Product of Two Sorted Arrays
//! https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays/
//! - `Hard`; `y2025m06d25`; `Hinted`; `299ms`; `2.9mb`; `2 attempts`;
//! Topics: binary_search.

pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
    let k = k as usize;
    let mut begin = i64::MIN as i128;
    let mut end = i64::MAX as i128 + 1;

    let get_posi_row_less_than_count = |num: i32, less_than_num: i128| -> usize {
        let mut begin_i = 0usize;
        let mut end_i = nums2.len();

        while begin_i < end_i {
            let mid_i = (begin_i + end_i) >> 1;
            let mid_num = nums2[mid_i];

            let mid_prod = mid_num as i128 * num as i128;

            if mid_prod >= less_than_num {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }

        begin_i
    };

    let get_nega_row_less_than_count = |num: i32, less_than_num: i128| -> usize {
        let mut begin_i = 0usize;
        let mut end_i = nums2.len();

        while begin_i < end_i {
            let mid_i = (begin_i + end_i) >> 1;
            let mid_num = nums2[mid_i];

            let mid_prod = mid_num as i128 * num as i128;

            if mid_prod < less_than_num {
                end_i = mid_i;
            } else {
                begin_i = mid_i + 1;
            }
        }

        begin_i
    };

    let get_product_less_than_count = |mid: i128| -> usize {
        let mut ans_count: usize = 0;
        for num_1 in nums1.iter().cloned() {
            ans_count += if num_1 > 0 {
                get_posi_row_less_than_count(num_1, mid)
            } else if num_1 < 0 {
                get_nega_row_less_than_count(num_1, mid)
            } else if mid <= 0 {
                0
            } else {
                nums2.len()
            };
        }
        ans_count
    };

    while begin < end {
        let mid = (begin + end) / 2;

        let less_than_mid_count = get_product_less_than_count(mid);

        if less_than_mid_count < k {
            begin = mid + 1;
        } else if less_than_mid_count > k {
            end = mid;
        } else {
            return mid as i64;
        }
    }
    unreachable!()
}
