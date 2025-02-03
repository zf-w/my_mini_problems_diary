//! # Leetcode 2429. Minimize XOR
//! https://leetcode.com/problems/minimize-xor/
//! - `Medium`; `y2024m01d15`; `Independently Solved`; `0ms`; `2.3mb`; `2 attempts`;

pub fn minimize_xor(num1: i32, mut num2: i32) -> i32 {
    let mut one_bit_count: usize = 0;
    while num2 > 0 {
        num2 = (num2 - 1) & num2;
        one_bit_count += 1;
    }
    let mut ans_num = 0;
    for i in 0..32usize {
        let off = 32 - i - 1;
        let curr_bit = (num1 >> off) & 1;
        if off < one_bit_count {
            ans_num |= 1 << off;
        } else if curr_bit == 1 {
            one_bit_count -= 1;
            ans_num |= 1 << off;
            if one_bit_count == 0 {
                break;
            }
        }
        // println!("{}, {}", curr_bit, one_bit_count);
    }
    ans_num
}
