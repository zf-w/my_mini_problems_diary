//! # Leetcode 2300. Successful Pairs of Spells and Potions
//! https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
//! - `Medium`; `y2025m10d08`; `Independently Solved`; `24ms`; `2.6mb`; `2 attempts`
//! Topics: binary_search.

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let spell_num = spells.len();
    let potion_num = potions.len();
    let mut spell_idx_vec: Vec<usize> = Vec::with_capacity(spell_num);

    for i in 0..spell_num {
        spell_idx_vec.push(i);
    }

    for entry_mut_ref in potions.iter_mut() {
        *entry_mut_ref <<= 1;
    }

    potions.sort_unstable();
    spell_idx_vec.sort_unstable_by_key(|spell_idx_ref| -> i32 { spells[*spell_idx_ref] });

    let mut ans_ok_num_vec: Vec<i32> = vec![0; spell_num];

    let potion_max = *potions.last().unwrap();

    for spell_idx in spell_idx_vec {
        let spell = spells[spell_idx] as i64;

        let potion_lower_bound =
            (((success / spell) + if success % spell == 0 { 0 } else { 1 }) << 1) - 1;

        // Be careful with underflow and overflow.
        if (potion_max as i64) < potion_lower_bound {
            ans_ok_num_vec[spell_idx] = 0;
            continue;
        }

        let partition_point = potions
            .binary_search(&(potion_lower_bound as i32))
            .expect_err("Shouldn't exist");

        ans_ok_num_vec[spell_idx] = (potion_num - partition_point) as i32;
    }

    ans_ok_num_vec
}
