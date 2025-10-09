//! Leetcode 3508. Implement Router
//! https://leetcode.com/problems/implement-router/
//! - `Medium`; `y2025m09d20`; `Independently Solved`; `78ms`; `80.9mb`; `3 attempts`;
//! Topics: binary_search.

use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

struct Router {
    capacity: usize,
    packet_set: HashSet<(i32, i32, i32)>,
    vec_queue: VecDeque<(i32, i32, i32)>,
    count_hashmap: HashMap<i32, VecDeque<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(capacity: i32) -> Self {
        let capacity: usize = capacity as usize;

        Self {
            capacity,
            packet_set: HashSet::with_capacity(capacity),
            vec_queue: VecDeque::with_capacity(capacity),
            count_hashmap: HashMap::with_capacity(capacity),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = (source, destination, timestamp);
        if self.packet_set.contains(&packet) == true {
            return false;
        }

        if self.vec_queue.len() >= self.capacity {
            self.forward_packet();
        }

        self.packet_set.insert(packet.clone());
        self.vec_queue.push_back(packet);

        use std::collections::hash_map::Entry;

        match self.count_hashmap.entry(destination) {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().push_back(timestamp << 1);
            }
            Entry::Vacant(vacant_entry) => {
                let mut time_stamp_vec_deq = VecDeque::new();
                time_stamp_vec_deq.push_back(timestamp << 1);
                vacant_entry.insert(time_stamp_vec_deq);
            }
        }

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let (src, dest, timestamp) = if let Some(packet) = self.vec_queue.pop_front() {
            packet
        } else {
            return vec![];
        };

        self.packet_set.remove(&(src, dest, timestamp));

        self.count_hashmap
            .get_mut(&dest)
            .expect("Should have this entry.")
            .pop_front();

        vec![src, dest, timestamp]
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let timestamp_vec_deq_ref =
            if let Some(timestamp_vec_deq_ref) = self.count_hashmap.get(&destination) {
                timestamp_vec_deq_ref
            } else {
                return 0;
            };

        let begin_idx = match timestamp_vec_deq_ref.binary_search(&((start_time << 1) - 1)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let end_idx = match timestamp_vec_deq_ref.binary_search(&((end_time << 1) + 1)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        (end_idx - begin_idx) as i32
    }
}
