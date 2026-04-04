//! # Leetcode 1888. Minimum Number of Flips to Make the Binary String Alternating
//! https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/
//! - y2026m03d07; Hinted;

pub fn min_flips(s: String) -> i32 {
        let mut zero_at_even_entry_count: usize = 0;
    let mut zero_at_odd_entry_count: usize = 0;
    let mut one_at_even_entry_count: usize = 0;
    let mut one_at_odd_entry_count: usize = 0;

    s.as_bytes()
        .iter()
        .cloned()
        .fold(true, |is_even_idx_flag, c_u8| -> bool {
            match (c_u8 == b'0', is_even_idx_flag) {
                (true, true) => zero_at_even_entry_count += 1,
                (true, false) => zero_at_odd_entry_count += 1,
                (false, true) => one_at_even_entry_count += 1,
                (false, false) => one_at_odd_entry_count += 1,
            };
            !is_even_idx_flag
        });

    fn calc_action_count(
        zero_at_even_entry_count: usize,
        zero_at_odd_entry_count: usize,
        one_at_even_entry_count: usize,
        one_at_odd_entry_count: usize,
    ) -> i32 {
        ((zero_at_odd_entry_count + one_at_even_entry_count)
            .min(zero_at_even_entry_count + one_at_odd_entry_count)) as i32
    }

    let mut ans_min_action_count = calc_action_count(
        zero_at_even_entry_count,
        zero_at_odd_entry_count,
        one_at_even_entry_count,
        one_at_odd_entry_count,
    );

    if s.len() & 1 == 0 {
        return ans_min_action_count;
    }

    for c_u8 in s.as_bytes().iter().cloned() {
        std::mem::swap(&mut zero_at_even_entry_count, &mut zero_at_odd_entry_count);
        std::mem::swap(&mut one_at_even_entry_count, &mut one_at_odd_entry_count);

        if c_u8 == b'0' {
            zero_at_odd_entry_count -= 1;
            zero_at_even_entry_count += 1;
        } else {
            one_at_odd_entry_count -= 1;
            one_at_even_entry_count += 1;
        }

        ans_min_action_count = ans_min_action_count.min(calc_action_count(
            zero_at_even_entry_count,
            zero_at_odd_entry_count,
            one_at_even_entry_count,
            one_at_odd_entry_count,
        ));
    }

    ans_min_action_count
}