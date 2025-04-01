//! ## Leetcode 1106. Parsing A Boolean Expression
//! https://leetcode.com/problems/parsing-a-boolean-expression/
//! - `Hard`; `Independently Solved`; `y2024m10d20`;

fn parse_bool_expr_helper(expr_str_ref: &str) -> Result<bool, &'static str> {
    if expr_str_ref == "t" {
        return Ok(true);
    } else if expr_str_ref == "f" {
        return Ok(false);
    }

    let mut idx_and_char_iter = expr_str_ref.char_indices();
    let cmd_char = idx_and_char_iter.next().ok_or("Not having a cmd char")?.1;
    let mut sub_begin_i = idx_and_char_iter.next().ok_or("Missing '('")?.0 + 1;
    let mut bracket_count: usize = 0;
    match cmd_char {
        '!' => {
            let mut end_i_opt: Option<usize> = None;

            while let Some((i, c)) = idx_and_char_iter.next() {
                if c == '(' {
                    bracket_count += 1;
                } else if bracket_count == 0 && c == ')' {
                    end_i_opt = Some(i);
                    break;
                } else if c == ')' {
                    bracket_count -= 1;
                }
            }
            let end_i = end_i_opt.ok_or("Expecting ')'")?;
            Ok(!parse_bool_expr_helper(&expr_str_ref[sub_begin_i..end_i])?)
        }
        '|' => {
            while let Some((i, c)) = idx_and_char_iter.next() {
                if c == '(' {
                    bracket_count += 1;
                }
                if bracket_count == 0 && (c == ',' || c == ')') {
                    let sub_ans = parse_bool_expr_helper(&expr_str_ref[sub_begin_i..i])?;
                    if sub_ans == true {
                        return Ok(true);
                    }
                    sub_begin_i = i + 1;

                    if c == ')' {
                        break;
                    }
                }
                if c == ')' {
                    bracket_count -= 1;
                }
            }
            Ok(false)
        }
        '&' => {
            while let Some((i, c)) = idx_and_char_iter.next() {
                if c == '(' {
                    bracket_count += 1;
                } else if bracket_count == 0 && (c == ',' || c == ')') {
                    let sub_ans = parse_bool_expr_helper(&expr_str_ref[sub_begin_i..i])?;
                    if sub_ans == false {
                        return Ok(false);
                    }
                    sub_begin_i = i + 1;

                    if c == ')' {
                        break;
                    }
                } else if c == ')' {
                    bracket_count -= 1;
                }
            }
            Ok(true)
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn parse_bool_expr(expression: String) -> bool {
    match parse_bool_expr_helper(&expression) {
        Ok(res) => res,
        Err(err_msg) => {
            panic!("{}", err_msg)
        }
    }
}
