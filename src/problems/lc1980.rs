//! ## Leetcode 1980. Find Unique Binary String
//! https://leetcode.com/problems/find-unique-binary-string
//! 
//! Given an array of strings `nums` containing `n` **unique** binary strings 
//! each of length `n`, return a binary string of length n that does not appear
//! in `nums`. *If there are multiple answers, you may return any of them*
//! 
//! ### Example:
//! ```
//! let nums = vec![String::from("01"), String::from("10")];
//! let ans = learn_cs::problems::lc1980::find_different_binary_string(nums);
//! assert!(ans == "00" || ans == "11");
//! ```
//! ### Thoughts:
//! - `Independently Solved`; `2023-11-15`;
//! 
//! I initially solved this problem independently with sorting. Later, after I
//! checked the solution, I realized that this problem can be easily
//! solved by Cantor's diagonal argument.
//! I guess that's the author's intention setting number of binary numbers 
//! equal to their lengths.
//! It's indeed an interesting problem.
//! 


/// ## Intuition:
/// 
/// The basic idea is to transform the strings into numbers, sort the numbers, 
/// and see if there is a gap between pairs of numbers
pub fn find_different_binary_string(nums: Vec<String>) -> String {
    fn str_to_u16(s: &str) -> u16 {
        let mut res: u16 = 0;
        for c in s.chars() {
            res = res << 1;
            if c == '1' {
                res += 1;
            }
        }
        res
    }
    fn u16_to_str(mut num: u16, len: usize) -> String {
        let mut res: String = String::new();
        while num > 0 {
            if num & 1 == 1 {
                res.push('1');
            } else {
                res.push('0');
            }
            num = num >> 1;
        }
        for _ in (res.len())..len {
            res.push('0');
        }
        let mut reverse: String = String::with_capacity(res.capacity());
        for c in res.chars().rev() {
          reverse.push(c);
        }
        reverse
    }
    let len = nums.len();
    let mut num_nums: Vec<u16> = Vec::with_capacity(len);
    for num_str in nums.iter() {
        num_nums.push(str_to_u16(num_str));
    }
    num_nums.sort();
    if num_nums[0] != 0 {
        return u16_to_str(0, len);
    }
    for i in 0..(len - 1) {
        if num_nums[i] + 1 != num_nums[i + 1] {
            return u16_to_str(num_nums[i] + 1, len);
        }
    }
    u16_to_str(num_nums[len - 1] + 1, len)
}

pub fn find_different_binary_string_1(nums: Vec<String>) -> String {
    let n: usize = nums.len();
    let mut res: String = String::with_capacity(n);
    for i in 0..n {
        if nums[i].get(i..(i+1)).unwrap() == "1" {
            res.push('0');
        } else {
            res.push('1');
        }
    }
    res
}