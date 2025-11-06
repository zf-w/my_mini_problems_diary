//! # Leetcode 3147. Taking Maximum Energy From the Mystic Dungeon
//! https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/
//! - `Medium`; `y2025m10d09`; `Independently Solved`; `27ms`; `3mb`; `3 attempts`;
//! Topics: uncategorized.

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;

    let energy_len = energy.len();

    let mut ans_max = energy[energy_len - 1];

    for mut rev_begin_idx in (if k >= energy_len { 0 } else { energy_len - k })..energy_len {
        let mut curr_begin_max = energy[rev_begin_idx];
        ans_max = ans_max.max(curr_begin_max); // Don't forget the first potential update.

        // print!("{} ", curr_begin_max);
        while rev_begin_idx >= k {
            // Don't forget the equal sign.
            rev_begin_idx -= k;
            curr_begin_max += energy[rev_begin_idx];

            ans_max = ans_max.max(curr_begin_max);
            // print!("{} ", curr_begin_max);
        }
        // println!("");
    }
    ans_max
}
