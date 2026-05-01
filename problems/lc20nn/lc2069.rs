//! # LeetCode 2069. Walking Robot Simulation II
//! https://leetcode.com/problems/walking-robot-simulation-ii/
//! - y2026m04d07; Independently Solved;

struct Robot {
    row_i: usize,
    col_i: usize,
    row_len: usize,
    col_len: usize,
    direction_idx: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            row_i: 0,
            col_i: 0,
            row_len: height as usize,
            col_len: width as usize,
            direction_idx: 0,
        }
    }

    fn step(&mut self, num: i32) {
        let num = (num as usize) % (((self.row_len + self.col_len) << 1) - 4);
        if self.row_i == 0 && self.col_i == 0 && self.direction_idx == 1 && num == 0 {
            self.direction_idx = 2;
        }
        for _ in 0..num {
            match self.direction_idx {
                0 => {
                    if self.row_i + 1 == self.row_len {
                        if self.col_i == 0 {
                            self.row_i -= 1;
                            self.direction_idx = 2;
                        } else {
                            self.col_i -= 1;
                            self.direction_idx = 3;
                        }
                    } else {
                        self.row_i += 1;
                    }
                }
                1 => {
                    if self.col_i + 1 == self.col_len {
                        if self.row_i + 1 == self.row_len {
                            self.col_i -= 1;
                            self.direction_idx = 3;
                        } else {
                            self.row_i += 1;
                            self.direction_idx = 0;
                        }
                    } else {
                        self.col_i += 1;
                    }
                }
                2 => {
                    if self.row_i == 0 {
                        if self.col_i + 1 == self.col_len {
                            self.row_i += 1;
                            self.direction_idx = 0;
                        } else {
                            self.col_i += 1;
                            self.direction_idx = 1;
                        }
                    } else {
                        self.row_i -= 1;
                    }
                }
                _ => {
                    if self.col_i == 0 {
                        if self.row_i == 0 {
                            self.col_i += 1;
                            self.direction_idx = 1;
                        } else {
                            self.row_i -= 1;
                            self.direction_idx = 2;
                        }
                    } else {
                        self.col_i -= 1;
                    }
                }
            }
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.row_i as i32, self.col_i as i32]
    }

    fn get_dir(&self) -> String {
        match self.direction_idx {
            0 => String::from("North"),
            1 => String::from("East"),
            2 => String::from("South"),
            _ => String::from("West"),
        }
    }
}
