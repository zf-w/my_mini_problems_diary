//! # Leetcode 1792. Maximum Average Pass Ratio
//! https://leetcode.com/problems/maximum-average-pass-ratio/
//! - `Medium`; `y2024m12d15`; `Hinted`; `167ms`; `10mb`; `1 attempt`;

#[inline]
fn calc_percent(passing_num: i32, total_num: i32) -> f64 {
    passing_num as f64 / total_num as f64
}

#[derive(PartialEq, PartialOrd)]
struct ClassInfo {
    increase_percent: f64,
    passing_num: i32,
    total_num: i32,
}

impl ClassInfo {
    pub fn new_with_info(passing_num: i32, total_num: i32) -> Self {
        let increase_percent =
            calc_percent(passing_num + 1, total_num + 1) - calc_percent(passing_num, total_num);
        Self {
            increase_percent,
            passing_num,
            total_num,
        }
    }

    pub fn push_extra_student(&mut self) {
        let passing_num = self.passing_num + 1;
        let total_num = self.total_num + 1;
        let increase_percent =
            calc_percent(passing_num + 1, total_num + 1) - calc_percent(passing_num, total_num);

        self.increase_percent = increase_percent;
        self.passing_num = passing_num;
        self.total_num = total_num;
    }

    pub fn get_avg(&self) -> f64 {
        self.passing_num as f64 / self.total_num as f64
    }
}

impl Ord for ClassInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.increase_percent
            .partial_cmp(&other.increase_percent)
            .expect("Should work for the problem.")
    }
}

impl Eq for ClassInfo {}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;
        let class_vec_len = classes.len();
        let mut heap: BinaryHeap<ClassInfo> = BinaryHeap::with_capacity(classes.len());
        for class_vec in classes {
            let passing_num = class_vec[0];
            let total_num = class_vec[1];
            heap.push(ClassInfo::new_with_info(passing_num, total_num))
        }

        for _ in 0..extra_students {
            let mut popped_class_info = heap.pop().expect("Should have");

            popped_class_info.push_extra_student();
            heap.push(popped_class_info);
        }

        let mut ans_avg: f64 = 0.0;
        for class_info in heap {
            ans_avg += class_info.get_avg();
        }

        ans_avg / class_vec_len as f64
    }
}
