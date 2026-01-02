//! # Leetcode 3606. Coupon Code Validator
//! https://leetcode.com/problems/coupon-code-validator/
//! - `Easy`; `y2025m12d12`; `Independently Solved`; `0ms`; `2.5mb`; `1 attempt`


pub fn validate_coupons(
    mut code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
) -> Vec<String> {
    let total_coupon_num = code.len();

    fn check_code_char(c: char) -> bool {
        let c_u8 = c as u8;

        c_u8.is_ascii_digit() || c_u8.is_ascii_lowercase() || c_u8.is_ascii_uppercase() || c == '_'
    }

    fn check_code(code: &str) -> bool {
        if code.is_empty() == true {
            return false;
        }

        for c in code.chars() {
            if check_code_char(c) == false {
                return false;
            }
        }

        true
    }

    fn check_category_str(category_str_ref: &str) -> bool {
        if category_str_ref.is_empty() {
            return false;
        }

        let first_char_u8 = category_str_ref.as_bytes()[0];

        match first_char_u8 {
            b'e' => category_str_ref == "electronics",
            b'g' => category_str_ref == "grocery",
            b'p' => category_str_ref == "pharmacy",
            b'r' => category_str_ref == "restaurant",
            _ => false,
        }
    }

    fn check_coupon_info(
        (_, (code_ref, (business_category_string_ref, is_active_flag))): (
            usize,
            (&String, (&String, bool)),
        ),
    ) -> bool {
        (is_active_flag == true)
            && check_code(code_ref)
            && check_category_str(business_category_string_ref)
    }

    let mut coupon_idx_vec: Vec<usize> = Vec::with_capacity(total_coupon_num);

    for info in code
        .iter()
        .zip(business_line.iter().zip(is_active.iter().cloned()))
        .enumerate()
    {
        if check_coupon_info(info) == false {
            continue;
        }

        coupon_idx_vec.push(info.0);
    }

    coupon_idx_vec.sort_by(|idx_0_ref, idx_1_ref| -> std::cmp::Ordering {
        let idx_0 = *idx_0_ref;
        let idx_1 = *idx_1_ref;

        (&business_line[idx_0], &code[idx_0]).cmp(&(&business_line[idx_1], &code[idx_1]))
    });

    let mut ans_string_vec: Vec<String> = Vec::with_capacity(coupon_idx_vec.len());

    for coupon_idx in coupon_idx_vec {
        let mut temp_swap_string = String::new();

        std::mem::swap(&mut code[coupon_idx], &mut temp_swap_string);

        ans_string_vec.push(temp_swap_string);
    }

    ans_string_vec
}