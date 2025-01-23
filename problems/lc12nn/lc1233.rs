//! ## Leetcode 1233. Remove Sub-Folders from the Filesystem
//! https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
//! - `Medium`; `Independently Solved`; `y2024m10d25`;

use std::collections::HashMap;

struct FolderTreeNode<'src> {
    seg_str_ref: &'src str,
    next_node_idxs_map: HashMap<&'src str, usize>,
    end_here_flag: bool,
}

struct FolderTree<'src> {
    nodes_vec: Vec<FolderTreeNode<'src>>,
}

impl<'src> FolderTree<'src> {
    fn add_folder(&mut self, folder_str_ref: &'src str) {
        let mut folder_seg_str_refs_iter = folder_str_ref.split('/');
        folder_seg_str_refs_iter.next();
        let mut curr_node_i: usize = 0;
        use std::collections::hash_map::Entry;
        let mut nodes_len = self.nodes_vec.len();
        for folder_seg_str_ref in folder_seg_str_refs_iter {
            let curr_node_mut_ref = &mut self.nodes_vec[curr_node_i];
            let next_node_entry = curr_node_mut_ref
                .next_node_idxs_map
                .entry(folder_seg_str_ref);

            let next_node_i = match next_node_entry {
                Entry::Occupied(occupied_entry) => occupied_entry.get().clone(),
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(nodes_len);
                    nodes_len
                }
            };
            if next_node_i == nodes_len {
                self.nodes_vec.push(FolderTreeNode {
                    seg_str_ref: folder_seg_str_ref,
                    next_node_idxs_map: HashMap::new(),
                    end_here_flag: false,
                });
                nodes_len += 1;
            }
            curr_node_i = next_node_i;
        }
        let curr_node_mut_ref = &mut self.nodes_vec[curr_node_i];
        curr_node_mut_ref.end_here_flag = true;
    }

    pub fn new_with_folders(folder_strings: &'src [String]) -> Self {
        let nodes_vec: Vec<FolderTreeNode> = vec![FolderTreeNode {
            seg_str_ref: "",
            next_node_idxs_map: HashMap::new(),
            end_here_flag: false,
        }];
        let mut tree = Self { nodes_vec };
        for folder_string_ref in folder_strings.iter() {
            tree.add_folder(folder_string_ref);
        }
        tree
    }

    fn remove_subfolders_helper(
        &self,
        subroot_i: usize,
        node_path_mut_ref: &mut Vec<usize>,
        ans_vec_mut_ref: &mut Vec<String>,
    ) {
        let curr_node_ref = &self.nodes_vec[subroot_i];
        if curr_node_ref.end_here_flag == true {
            let mut curr_path_string = String::new();
            for node_idx in node_path_mut_ref.iter().cloned() {
                let node_ref = &self.nodes_vec[node_idx];
                curr_path_string.push('/');
                curr_path_string.push_str(node_ref.seg_str_ref);
            }
            curr_path_string.push('/');
            curr_path_string.push_str(curr_node_ref.seg_str_ref);
            ans_vec_mut_ref.push(curr_path_string);
            return;
        }

        node_path_mut_ref.push(subroot_i);
        for next_node_i in curr_node_ref.next_node_idxs_map.values().cloned() {
            self.remove_subfolders_helper(next_node_i, node_path_mut_ref, ans_vec_mut_ref);
        }
        node_path_mut_ref.pop();
    }

    pub fn remove_subfolders(&self) -> Vec<String> {
        let mut node_path_vec: Vec<usize> = Vec::new();
        let mut ans_strings_vec: Vec<String> = Vec::new();
        let curr_node_ref = &self.nodes_vec[0];
        for next_node_i in curr_node_ref.next_node_idxs_map.values().cloned() {
            self.remove_subfolders_helper(next_node_i, &mut node_path_vec, &mut ans_strings_vec);
        }
        ans_strings_vec
    }
}

impl Solution {
    pub fn remove_subfolders(folders: Vec<String>) -> Vec<String> {
        let tree = FolderTree::new_with_folders(&folders);
        tree.remove_subfolders()
    }
}
