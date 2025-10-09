//! # Leetcode 3484. Design Spreadsheet
//! https://leetcode.com/problems/design-spreadsheet/
//! - `Medium`; `y2025m09d19`; `Independently Solved`; `24ms`; `7.5mb`; `2 attempts`;
//! Topics: uncategorized.

const UPPER_LETTER_NUM: usize = 26;

struct Spreadsheet {
    row_num: usize,
    data: Box<[i32]>,
}

fn calc_char_idx(c_u8: u8) -> usize {
    (c_u8 - b'A') as usize
}

fn num_char_to_digit(c_u8: u8) -> usize {
    (c_u8 - b'0') as usize
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn calc_idx(c_u8_iter_mut_ref: &mut impl Iterator<Item = u8>, row_num: usize) -> (bool, usize) {
        let first_c_u8 = c_u8_iter_mut_ref.next().unwrap();
        let mut ans_idx = 0;
        if (b'0'..=b'9').contains(&first_c_u8) {
            ans_idx += num_char_to_digit(first_c_u8);

            while let Some(c_u8) = c_u8_iter_mut_ref.next() {
                if c_u8 == b'+' {
                    break;
                }
                ans_idx = ans_idx * 10 + num_char_to_digit(c_u8);
            }

            (false, ans_idx)
        } else {
            ans_idx += calc_char_idx(first_c_u8) * row_num;

            let mut row_idx = 0;

            while let Some(c_u8) = c_u8_iter_mut_ref.next() {
                if c_u8 == b'+' {
                    break;
                }
                row_idx = row_idx * 10 + num_char_to_digit(c_u8);
            }

            (true, ans_idx + row_idx - 1) // Bug 0: A1 is actually "A0".
        }
    }

    fn new(rows: i32) -> Self {
        let row_num = rows as usize;

        Self {
            row_num,
            data: vec![0; row_num * UPPER_LETTER_NUM].into_boxed_slice(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let mut iter = cell.as_bytes().iter().cloned();

        let (_, idx) = Self::calc_idx(&mut iter, self.row_num);
        self.data[idx] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0);
    }

    fn get_value(&self, formula: String) -> i32 {
        let mut c_u8_iter = formula.as_bytes().iter().cloned();
        c_u8_iter.next();

        let (is_first_idx_flag, idx_0) = Self::calc_idx(&mut c_u8_iter, self.row_num);

        let mut ans = 0;

        ans += if is_first_idx_flag == true {
            self.data[idx_0]
        } else {
            idx_0 as i32
        };

        let (is_second_idx_flag, idx_1) = Self::calc_idx(&mut c_u8_iter, self.row_num);

        ans += if is_second_idx_flag == true {
            self.data[idx_1]
        } else {
            idx_1 as i32
        };

        ans
    }
}
