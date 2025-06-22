//! # Leetcode 2138. Divide a String Into Groups of Size k
//! https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
//! - `Easy`; `y2025m06d22`; `Independently Solved`; `0ms`; `2.1mb`; `1 attempt`;
//! Topics: uncategorized.

pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let k = k as usize;
    let len = s.len();

    let fill_num = k - (len % k);

    let (div_str_num, full_num) = if len % k == 0 {
        let div = len / k;
        (div, div)
    } else {
        let div = len / k;
        (div + 1, div)
    };

    let mut ans_vec: Vec<String> = Vec::with_capacity(div_str_num);

    let mut c_iter = s.chars();

    for _ in 0..full_num {
        let mut curr_string = String::with_capacity(k);
        for _ in 0..k {
            curr_string.push(c_iter.next().expect("len checked."));
        }
        ans_vec.push(curr_string);
    }

    if fill_num < k {
        let mut curr_string = String::with_capacity(k);
        for c in c_iter {
            curr_string.push(c);
        }
        for _ in 0..fill_num {
            curr_string.push(fill);
        }
        ans_vec.push(curr_string);
    }

    ans_vec
}
