//! ## Leetcode 2416. Sum of Prefix Scores of Strings
//! https://leetcode.com/problems/sum-of-prefix-scores-of-strings/
//! - `Hard`; `Independently Solved, but Slow`; `2024-09-24`;

struct Node {
    next_node_idxs: Box<[Option<usize>; 26]>, // I guess this would make "moving" faster, but it seems not using the "Box" version is actually quicker.
    end_here_count: usize,
}

impl Node {
    pub fn new() -> Self {
        Node {
            next_node_idxs: Box::new([None; 26]),
            end_here_count: 0,
        }
    }
}

const LOWER_A_U8: u8 = 'a' as u8;

struct PrefixTree {
    nodes: Vec<Node>,
}

fn char_to_idx(c: char) -> usize {
    (c as u8 - LOWER_A_U8) as usize
}

impl PrefixTree {
    pub fn new() -> Self {
        let mut nodes: Vec<Node> = Vec::with_capacity(10);
        nodes.push(Node::new());
        Self { nodes }
    }

    pub fn insert(&mut self, char_iter: impl Iterator<Item = char>) {
        let mut curr_node_i: usize = 0;
        let mut next_node_i: usize = self.nodes.len();
        // print!("Insert ");
        for next_child_i in char_iter.map(char_to_idx) {
            // print!("->{}", next_child_i);
            let curr_node_mut_ref = unsafe { self.nodes.get_unchecked_mut(curr_node_i) };
            curr_node_mut_ref.end_here_count += 1;
            let curr_node_next_i_mut_ref = unsafe {
                curr_node_mut_ref
                    .next_node_idxs
                    .get_unchecked_mut(next_child_i)
            };
            if let Some(next_node_i) = curr_node_next_i_mut_ref.clone() {
                curr_node_i = next_node_i;
            } else {
                *curr_node_next_i_mut_ref = Some(next_node_i);
                self.nodes.push(Node::new());
                curr_node_i = next_node_i;
                next_node_i += 1;
            }
        }
        // print!("->{}", curr_node_i);
        let curr_node_mut_ref = unsafe { self.nodes.get_unchecked_mut(curr_node_i) };
        curr_node_mut_ref.end_here_count += 1;
        // println!("");
    }

    pub fn calc_score(&mut self, char_iter: impl Iterator<Item = char>) -> i32 {
        let mut curr_node_i: usize = 0;
        // print!("Calc ");
        let mut ans_score: usize = 0;
        for next_child_i in char_iter.map(char_to_idx) {
            // print!("->{}", next_child_i);
            let curr_node_ref = unsafe { self.nodes.get_unchecked(curr_node_i) };
            ans_score += curr_node_ref.end_here_count;
            let curr_node_next_i_ref =
                unsafe { curr_node_ref.next_node_idxs.get_unchecked(next_child_i) };
            if let Some(next_node_i) = curr_node_next_i_ref.clone() {
                curr_node_i = next_node_i;
            } else {
                unreachable!("Defined by the problem.");
            }
        }
        let curr_node_ref = unsafe { self.nodes.get_unchecked(curr_node_i) };
        ans_score += curr_node_ref.end_here_count;
        // println!("");
        ans_score as i32
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut words_len = words.len();
    let mut prefix_tree = PrefixTree::new();
    for word_ref in words.iter() {
        prefix_tree.insert(word_ref.chars());
    }
    let mut ans_scores_vec: Vec<i32> = Vec::with_capacity(words_len);
    for word in words.into_iter() {
        ans_scores_vec.push(prefix_tree.calc_score(word.chars()) - words_len as i32);
    }
    ans_scores_vec
}
