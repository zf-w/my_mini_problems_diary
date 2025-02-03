//! ## Leetcode 1255. Maximum Score Words Formed by Letters
//! https://leetcode.com/problems/maximum-score-words-formed-by-letters
//! - `Hard`; `Learned from Solution`; `2024-05-24`;
//!
//! I have read this: https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/5199415/c-backtracking-easy-solution
//!
//! At first, I thought a plain recursion might be too slow, but it works. The solution I read didn't check whether a path had already exceeded the limit in the middle, but I guess optimization is helpful.

const BASE_A: u8 = 'a' as u8;

fn char_to_i(c: &char) -> usize {
    (*c as u8 - BASE_A) as usize
}

fn helper(
    i: usize,
    prev_counts: &[usize; 26],
    total_counts: &[usize; 26],
    words: &[String],
    scores: &[i32],
) -> i32 {
    if i >= words.len() {
        return 0;
    }
    let curr_word = &words[i];
    let mut curr_counts: [usize; 26] = prev_counts.clone();
    let mut curr_score = 0;
    let mut possible = true;
    for c in curr_word.chars() {
        let char_i = char_to_i(&c);
        if curr_counts[char_i] >= total_counts[char_i] {
            possible = false;
            break;
        }
        curr_counts[char_i] += 1;
        curr_score += scores[char_i];
    }
    let possible = possible;

    let skip_v = helper(i + 1, &prev_counts, total_counts, words, scores);

    if possible {
        skip_v.max(curr_score + helper(i + 1, &curr_counts, total_counts, words, scores))
    } else {
        skip_v
    }
}

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut total_counts: [usize; 26] = [0; 26];
    let curr_counts: [usize; 26] = [0; 26];

    for letter in letters {
        total_counts[char_to_i(&letter)] += 1;
    }

    helper(0, &curr_counts, &total_counts, &words, &score)
}
