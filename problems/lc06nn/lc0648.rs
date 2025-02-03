//! ## Leetcode 648. Replace Words
//! https://leetcode.com/problems/replace-words/
//! - `Medium`; `Independently Solved`; `2024-06-06`;
//!
//! We can solve this problem by reading the dictionary into some sort of map and checking the prefixes of words in the sentence.
//!
//! But, which data structure should we use? Thinking about how to implement the structure to efficiently store and check is an interesting problem.
//!
//! I choose to use the [PrefixTree]. This structure can provide an early notice of whether a key exists on the map. Scanning through the sample solutions, I saw a bunch of solutions implementing the tree with reference counter smart pointers. I guess using a [vec] and referencing nodes with indices would be safer and somewhat faster?

struct Node {
    pub end: bool,
    pub nexts: [Option<usize>; 26],
}

impl Node {
    pub fn new() -> Self {
        Self {
            end: false,
            nexts: [None; 26],
        }
    }
}

struct PrefixTree {
    pub nodes: Vec<Node>,
}

impl PrefixTree {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node::new());
        Self { nodes }
    }

    pub fn root_entry<'src>(&'src mut self) -> Entry<'src> {
        Entry::new(self)
    }
}

struct Entry<'src> {
    src: &'src mut PrefixTree,
    curr: usize,
}

impl<'src> Entry<'src> {
    pub fn new(src: &'src mut PrefixTree) -> Self {
        Self { src, curr: 0 }
    }

    pub fn end_here(&mut self) {
        self.src.nodes[self.curr].end = true;
    }

    pub fn is_end(&self) -> bool {
        self.src.nodes[self.curr].end
    }

    pub fn has_next(&self, next_dir: usize) -> bool {
        self.src.nodes[self.curr].nexts[next_dir].is_some()
    }

    pub fn go_next(&mut self, next_dir: usize) {
        let curr_len = self.src.nodes.len();
        let curr_index_mut_ref = &mut self.src.nodes[self.curr].nexts[next_dir];
        if let Some(next_i) = *curr_index_mut_ref {
            self.curr = next_i;
        } else {
            *curr_index_mut_ref = Some(curr_len);
            self.src.nodes.push(Node::new());
            self.curr = curr_len;
        }
    }

    pub fn reset(&mut self) {
        self.curr = 0;
    }
}

const BASE_LOWER_A: u8 = 'a' as u8;

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut mp: PrefixTree = PrefixTree::new();
    let mut ans_string: String = String::new();
    fn char_to_i(c: char) -> usize {
        (c as u8 - BASE_LOWER_A) as usize
    }
    for word in dictionary.iter() {
        let mut entry = mp.root_entry();
        for c in word.chars() {
            let next_dir = char_to_i(c);
            entry.go_next(next_dir);
        }
        entry.end_here();
    }
    let mut entry = mp.root_entry();

    let mut curr_state: Option<bool> = None;
    for c in sentence.chars() {
        if c != ' ' {
            if let Some(skip_push) = curr_state {
                if !skip_push {
                    ans_string.push(c);
                }
            } else {
                ans_string.push(c);
            }

            let i = char_to_i(c);

            if entry.has_next(i) {
                entry.go_next(i);
                if entry.is_end() {
                    curr_state = Some(true);
                }
            } else {
                curr_state = Some(false);
            }
        } else {
            entry.reset();
            ans_string.push(' ');
            curr_state = None;
        }
    }
    ans_string
}
