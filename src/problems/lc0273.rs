//! ## Leetcode 273. Integer to English Words
//! https://leetcode.com/problems/integer-to-english-words/
//! - `Hard`; `Learned from Solution`; `2024-08-06`;
//!
//! Learned from https://leetcode.com/problems/integer-to-english-words/solutions/5599708/hard-one-for-non-native-english-speakers544rrrr

const GROUP_0_STR_REF_ARR: [&'static str; 20] = [
    "",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];

const GROUP_1_STR_REF_ARR: [&'static str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const GROUP_2_STR_REF_ARR: [&'static str; 4] = ["", "Thousand", "Million", "Billion"];

const SPACE_STR_REF: &'static str = " ";

fn transform_below_ten_thousands_num_and_push_to_buf(mut num: usize, buf: &mut String) {
    if num >= 100 {
        let curr_hundreds_i = (num / 100) as usize;
        num = num % 100;
        buf.push_str(GROUP_0_STR_REF_ARR[curr_hundreds_i]);
        buf.push_str(" Hundred ");
    }
    if num >= 20 {
        let curr_tens_i = num / 10;
        num = num % 10;
        buf.push_str(GROUP_1_STR_REF_ARR[curr_tens_i]);
        buf.push_str(SPACE_STR_REF);
        if num > 0 {
            buf.push_str(GROUP_0_STR_REF_ARR[num]);
            buf.push_str(SPACE_STR_REF);
        }
    } else {
        buf.push_str(GROUP_0_STR_REF_ARR[num]);
        buf.push_str(SPACE_STR_REF);
    }
}

pub fn number_to_words(num: i32) -> String {
    if num == 0 {
        return "Zero".to_string();
    }
    let mut num = num as usize;
    let mut ans_string: String = String::new();
    let mut level_num: usize = 1_000_000_000;
    let mut level_i: usize = 3;
    while level_num > 0 {
        let curr_part = num / level_num;
        if curr_part > 0 {
            transform_below_ten_thousands_num_and_push_to_buf(curr_part, &mut ans_string);
            ans_string.push_str(GROUP_2_STR_REF_ARR[level_i]);
            ans_string.push_str(SPACE_STR_REF);
        }
        level_i -= 1;
        num -= curr_part * level_num;
        level_num /= 1000;
    }
    ans_string.pop();
    ans_string
}
