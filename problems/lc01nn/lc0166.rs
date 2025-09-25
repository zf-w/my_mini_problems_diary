//! # Leetcode 166. Fraction to Recurring Decimal
//! https://leetcode.com/problems/fraction-to-recurring-decimal/
//! - `Medium`; `y2025m09d24`; `Learned from Solution`; `0ms`; `2.4mb`; `2 attempts`;
//! Topics: dynamic_programming.
//! https://leetcode.com/problems/fraction-to-recurring-decimal/solutions/7218653/fraction-to-recurring-decimal-conversion-long-division-beginner-friendly

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return String::from("0");
    }

    let mut ans_string = String::new();
    let mut numerator: i64 = numerator as i64;
    let mut denominator: i64 = denominator as i64;

    if (numerator > 0) != (denominator > 0) {
        ans_string.push('-');
    }

    numerator = numerator.max(-numerator);
    denominator = denominator.max(-denominator);

    ans_string.push_str(&(numerator / denominator).to_string());

    let mut remainder = numerator % denominator;

    if remainder == 0 {
        return ans_string;
    }

    ans_string.push('.');

    use std::collections::HashMap;

    let mut remainder_map: HashMap<i64, usize> = HashMap::new();
    let mut idx = ans_string.len();

    while remainder > 0 {
        if let Some(idx) = remainder_map.get(&remainder).cloned() {
            ans_string.insert(idx, '(');
            ans_string.push(')');
            break;
        }

        remainder_map.insert(remainder, idx);

        remainder *= 10;

        ans_string.push((b'0' + (remainder / denominator) as u8) as char);

        remainder %= denominator;

        idx += 1;
    }
    ans_string
}
