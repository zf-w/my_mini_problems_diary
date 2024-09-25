//! ## Leetcode 884. Uncommon Words from Two Sentences
//! https://leetcode.com/problems/uncommon-words-from-two-sentences/
//! - `Easy`; `Independently Solved`; `2024-09-17`;

use std::collections::HashMap;

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let splited_strs_1 = s1.split(' ');
    let splited_strs_2 = s2.split(' ');
    let mut word_set: HashMap<&str, bool> = HashMap::new();
    for part_str_ref in splited_strs_1 {
        let entry_mut_ref = word_set.entry(part_str_ref);
        entry_mut_ref
            .and_modify(|val_mut_ref| *val_mut_ref = false)
            .or_insert(true);
    }

    for part_str_ref in splited_strs_2 {
        let entry_mut_ref = word_set.entry(part_str_ref);
        entry_mut_ref
            .and_modify(|val_mut_ref| *val_mut_ref = false)
            .or_insert(true);
    }

    let mut ans_vec: Vec<String> = Vec::new();
    for (part_str_ref, is_uncommon_flag) in word_set {
        if is_uncommon_flag {
            ans_vec.push(part_str_ref.to_string());
        }
    }
    ans_vec
}
