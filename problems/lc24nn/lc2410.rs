//! # Leetcode 2410. Maximum Matching of Players with Trainers
//! https://leetcode.com/problems/maximum-matching-of-players-with-trainers/
//! - `Medium`; `y2025m07d13`; `Independently Solved`; `9ms`; `4.6mb`; `1 attempt`;
//! Topics: sorting.

pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
    fn reverse_fn(v_ref: &i32) -> i32 {
        -*v_ref
    }
    players.sort_by_key(reverse_fn);
    trainers.sort_by_key(reverse_fn);
    let mut ans_count = 0;

    let mut player_iter = players.into_iter();

    for trainer in trainers {
        let mut found = false;
        for player in player_iter.by_ref() {
            if player <= trainer {
                found = true;
                break;
            }
        }

        if found {
            ans_count += 1;
        } else {
            break;
        }
    }

    ans_count
}