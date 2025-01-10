//! # Leetcode 1408. String Matching in an Array
//! https://leetcode.com/problems/string-matching-in-an-array/
//! - `Easy`; `y2025m01d07`; `Independently Solved`; `0ms`; `2.4mb`; `1 attempt`;

pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
    let mut i: usize = 0;
    let mut len = words.len();
    let mut popped_string_vec: Vec<String> = Vec::with_capacity(len);
    while i < len {
        let mut is_substr_flag = false;
        let word_ref = words.get(i).expect("Expecting [Some] because len checked.");
        for (i_1, word_1_ref) in words.iter().chain(popped_string_vec.iter()).enumerate() {
            if i_1 == i {
                continue;
            }
            if word_1_ref.contains(word_ref) {
                is_substr_flag = true;
            }
        }
        if is_substr_flag == false {
            words.swap(i, len - 1);
            let popped_string = words.pop().expect("Expecting [Some] because len checked.");
            popped_string_vec.push(popped_string);
            len -= 1;
        } else {
            i += 1;
        }
    }
    words
}
