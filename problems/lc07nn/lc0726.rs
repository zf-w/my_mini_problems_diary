//! ## Leetcode 726. Number of Atoms
//! https://leetcode.com/problems/number-of-atoms/
//! - `Hard`; `Independently Solved`; `2024-07-14`;

use std::collections::BTreeMap;

enum EventEnum<'src> {
    Count(usize),
    Atom(&'src [u8]),
    L,
    R,
}

const ASCII_0_U8: u8 = '0' as u8;

fn parse_usize_from_ascii_u8s(char_u8s_ref: &[u8]) -> usize {
    let mut ans_num: usize = 0;
    for c in char_u8s_ref.iter().cloned() {
        ans_num = ans_num * 10 + (c - ASCII_0_U8) as usize;
    }
    ans_num
}

fn parse_formula_bytes<'src>(formula_bytes: &'src [u8]) -> Vec<EventEnum<'src>> {
    let len = formula_bytes.len();
    let char_iter = formula_bytes.iter().enumerate();
    let mut prev_info: Option<(usize, bool)> = None;
    let mut events: Vec<EventEnum> = Vec::new();

    for (curr_i, c_u8_ref) in char_iter {
        let c = *c_u8_ref as char;

        if c.is_alphabetic() {
            if c.is_uppercase() {
                if let Some((prev_i, atom_flag)) = prev_info {
                    let prev_char_u8s_ref = &formula_bytes[prev_i..curr_i];
                    if atom_flag {
                        events.push(EventEnum::Atom(prev_char_u8s_ref));
                        events.push(EventEnum::Count(1));
                        // print!("{},1,", unsafe {
                        //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
                        // });
                    } else {
                        events.push(EventEnum::Count(parse_usize_from_ascii_u8s(
                            prev_char_u8s_ref,
                        )));
                        // print!("{},", unsafe {
                        //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
                        // });
                    }
                }
                prev_info = Some((curr_i, true));
            }
        } else if c.is_numeric() {
            if let Some((prev_i, atom_flag)) = prev_info {
                let prev_char_u8s_ref = &formula_bytes[prev_i..curr_i];
                if atom_flag {
                    events.push(EventEnum::Atom(prev_char_u8s_ref));
                    // print!("{},", unsafe {
                    //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
                    // });
                    prev_info = Some((curr_i, false));
                }
            } else {
                prev_info = Some((curr_i, false));
            }
        } else {
            if let Some((prev_i, atom_flag)) = prev_info {
                let prev_char_u8s_ref = &formula_bytes[prev_i..curr_i];
                if atom_flag {
                    events.push(EventEnum::Atom(prev_char_u8s_ref));
                    events.push(EventEnum::Count(1));
                    // print!("{},1,", unsafe {
                    //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
                    // });
                } else {
                    events.push(EventEnum::Count(parse_usize_from_ascii_u8s(
                        prev_char_u8s_ref,
                    )));
                    // print!("{},", unsafe {
                    //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
                    // });
                }
            }
            prev_info = None;
            if c == '(' {
                events.push(EventEnum::L);
                // print!("(,");
            } else if c == ')' {
                events.push(EventEnum::R);
                // print!("),")
            } else {
                unreachable!()
            }
        }
    }
    if let Some((prev_i, atom_flag)) = prev_info {
        let prev_char_u8s_ref = &formula_bytes[prev_i..len];
        if atom_flag {
            events.push(EventEnum::Atom(prev_char_u8s_ref));
            events.push(EventEnum::Count(1));
            // print!("{},1,", unsafe {
            //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
            // });
        } else {
            events.push(EventEnum::Count(parse_usize_from_ascii_u8s(
                prev_char_u8s_ref,
            )));
            // print!("{},", unsafe {
            //     String::from_utf8_unchecked(prev_char_u8s_ref.to_vec())
            // });
        }
    }
    return events;
}

pub fn count_of_atoms(formula: String) -> String {
    let formula_bytes = formula.as_bytes();
    let events = parse_formula_bytes(formula_bytes);
    let mut multipliers_stk: Vec<usize> = Vec::new();
    let mut multiplier: usize = 1;
    let mut map: BTreeMap<&[u8], usize> = BTreeMap::new();
    let mut prev_num_flag = false;

    for event in events.iter().rev() {
        match event {
            EventEnum::Count(count) => {
                multiplier *= count;
                multipliers_stk.push(count.clone());
                prev_num_flag = true;
            }
            EventEnum::Atom(char_u8s_ref) => {
                map.entry(&char_u8s_ref)
                    .and_modify(|count| {
                        *count += multiplier;
                    })
                    .or_insert(multiplier);
                if prev_num_flag {
                    if let Some(prev_multiplier) = multipliers_stk.pop() {
                        multiplier /= prev_multiplier;
                    }
                    prev_num_flag = false;
                }
            }
            EventEnum::L => {
                if let Some(prev_multiplier) = multipliers_stk.pop() {
                    multiplier /= prev_multiplier;
                }
            }
            EventEnum::R => (),
        }
    }

    let mut ans_u8s: Vec<u8> = Vec::new();

    for (atom_u8s_ref_ref, count_ref) in map.iter() {
        for c in atom_u8s_ref_ref.iter() {
            ans_u8s.push(c.clone());
        }
        if *count_ref > 1 {
            let count_string = count_ref.to_string();
            for c in count_string.as_bytes() {
                ans_u8s.push(c.clone());
            }
        }
    }
    unsafe { String::from_utf8_unchecked(ans_u8s) }
}
