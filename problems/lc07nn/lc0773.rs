//! ## Leetcode 773. Sliding Puzzle
//! https://leetcode.com/problems/sliding-puzzle/
//! - `Hard`; `y2024m11d25`; `Learned from Solution`; `0ms`; `2.2mb`; `1 attempt`;
//!
//! I should have tried to solve this by myself.

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    type State = [u8; 6];
    fn make_nexts(state_ref: &State) -> &'static [(usize, usize)] {
        if state_ref[0] == 0 {
            &[(0, 1), (0, 3)]
        } else if state_ref[1] == 0 {
            &[(0, 1), (1, 2), (1, 4)]
        } else if state_ref[2] == 0 {
            &[(1, 2), (2, 5)]
        } else if state_ref[3] == 0 {
            &[(0, 3), (3, 4)]
        } else if state_ref[4] == 0 {
            &[(3, 4), (1, 4), (4, 5)]
        } else {
            &[(4, 5), (2, 5)]
        }
    }
    fn make_next_state(state_ref: &State, i_0: usize, i_1: usize) -> [u8; 6] {
        let mut ans_state = state_ref.clone();
        ans_state.swap(i_0, i_1);
        ans_state
    }

    const TARGET_STATE: [u8; 6] = [1, 2, 3, 4, 5, 0];

    let start_state: [u8; 6] = [
        board[0][0] as u8,
        board[0][1] as u8,
        board[0][2] as u8,
        board[1][0] as u8,
        board[1][1] as u8,
        board[1][2] as u8,
    ];

    if TARGET_STATE == start_state {
        return 0;
    }

    let mut state_q: std::collections::VecDeque<State> = std::collections::VecDeque::new();

    let mut visited_state_set: std::collections::HashSet<State> = std::collections::HashSet::new();

    visited_state_set.insert(start_state);

    let mut ans_len: usize = 1;
    while state_q.is_empty() == false {
        let level_len = state_q.len();
        for _ in 0..level_len {
            let curr_state = state_q.pop_front().expect("checked len");

            let next_moves_ref = make_nexts(&curr_state);
            for (i_0, i_1) in next_moves_ref.iter().cloned() {
                let next_state = make_next_state(&curr_state, i_0, i_1);
                if visited_state_set.contains(&next_state) {
                    continue;
                }
                if next_state == TARGET_STATE {
                    return ans_len as i32;
                }

                visited_state_set.insert(next_state);
                state_q.push_back(next_state);
            }
        }
        ans_len += 1;
    }
    -1
}
