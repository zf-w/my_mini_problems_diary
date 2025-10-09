//! # Leetcode 1733. Minimum Number of People to Teach
//! https://leetcode.com/problems/minimum-number-of-people-to-teach/
//! - `Medium`; `y2025m09d10`; `Independently Solved`; `0ms`; `3.9ms`; `1 attempt`;
//! Topics: count_tracking.

pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    let lang_num = n as usize;
    let friend_num = languages.len();

    let mut lang_need_num_vec: Box<[i32]> = vec![0; lang_num].into_boxed_slice();

    let mut lang_known_vec: Box<[(bool, bool)]> =
        vec![(false, false); friend_num * lang_num].into_boxed_slice();

    let calc_idx = |friend_i: usize, lang_i: usize| -> usize { friend_i * lang_num + lang_i };

    for (friend_i, language_vec) in languages.into_iter().enumerate() {
        for lang_i in language_vec {
            let lang_i = lang_i as usize - 1;

            lang_known_vec[calc_idx(friend_i, lang_i)].0 = true;
        }
    }

    for friendship_vec in friendships {
        let friend_0_i = friendship_vec[0] as usize - 1;
        let friend_1_i = friendship_vec[1] as usize - 1;

        let mut add_flag = true;

        for lang_i in 0..lang_num {
            let idx_0 = calc_idx(friend_0_i, lang_i);
            let idx_1 = calc_idx(friend_1_i, lang_i);

            if lang_known_vec[idx_0].0 == true && lang_known_vec[idx_1].0 == true {
                add_flag = false;
                break;
            }
        }

        if add_flag == false {
            continue;
        }

        for lang_i in 0..lang_num {
            let idx_0 = calc_idx(friend_0_i, lang_i);
            let idx_1 = calc_idx(friend_1_i, lang_i);

            if lang_known_vec[idx_0].0 == false && lang_known_vec[idx_0].1 == false {
                lang_need_num_vec[lang_i] += 1;
                lang_known_vec[idx_0].1 = true;
            }

            if lang_known_vec[idx_1].0 == false && lang_known_vec[idx_1].1 == false {
                lang_need_num_vec[lang_i] += 1;
                lang_known_vec[idx_1].1 = true;
            }
        }
    }

    *lang_need_num_vec.iter().min().expect("at least one")
}
