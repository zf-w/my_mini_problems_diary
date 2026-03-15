//! # Leetcode 1622. Fancy Sequence
//! https://leetcode.com/problems/fancy-sequence/
//! - y2026m03d15; Learned from Solution;

const MODULUS: i64 = 1_000_000_007;
const MODULUS_INV_POW: i64 = MODULUS - 2;

fn add_with_modulus(a: i64, b: i64) -> i64 {
    (a + b) % MODULUS
}

fn mul_with_modulus(a: i64, b: i64) -> i64 {
    (a * b) % MODULUS
}

fn sub_with_modulus(a: i64, b: i64) -> i64 {
    (a + MODULUS - b) % MODULUS
}

fn quick_pow(mut num: i64, mut exponent: i64) -> i64 {
    let mut ans = 1;

    while exponent > 0 {
        if (exponent & 1) > 0 {
            ans = mul_with_modulus(ans, num);
        }

        num = mul_with_modulus(num, num);

        exponent >>= 1;
    }

    ans
}

fn div_with_modulus(a: i64, b: i64) -> i64 {
    let b_inv = quick_pow(b, MODULUS_INV_POW);

    mul_with_modulus(a, b_inv)
}

struct Fancy {
    value_vec: Vec<i32>,
    mul_value: i64,
    add_value: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            value_vec: Vec::new(),
            mul_value: 1,
            add_value: 0,
        }
    }

    fn append(&mut self, val: i32) {
        self.value_vec.push(
            (div_with_modulus(sub_with_modulus(val as i64, self.add_value), self.mul_value)) as i32,
        );
    }

    fn add_all(&mut self, inc: i32) {
        self.add_value = add_with_modulus(self.add_value, inc as i64);
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as i64;
        self.mul_value = mul_with_modulus(self.mul_value, m);
        self.add_value = mul_with_modulus(self.add_value, m);
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;

        if let Some(v) = self.value_vec.get(idx).cloned() {
            let v = v as i64;
            add_with_modulus(mul_with_modulus(v, self.mul_value), self.add_value) as i32
        } else {
            -1
        }
    }
}
