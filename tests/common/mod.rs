use core::convert::TryFrom;

use rand::Rng;

use twofloat::{TwoFloat, TwoFloatError};

const TEST_ITERS: usize = 100000;

pub fn random_f32() -> f32 {
    let mut engine = rand::thread_rng();
    let mantissa_dist = rand::distributions::Uniform::new(0, 1u32 << 23);
    let exponent_dist = rand::distributions::Uniform::new(0, (1u32 << 8) - 1);
    let x = f32::from_bits(engine.sample(mantissa_dist) | (engine.sample(exponent_dist) << 23));
    if engine.gen() {
        x
    } else {
        -x
    }
}

pub fn repeated_test(mut test: impl FnMut()) {
    for _ in 0..TEST_ITERS {
        test();
    }
}

pub fn repeated_test_enumerate(mut test: impl FnMut(usize)) {
    for i in 0..TEST_ITERS {
        test(i);
    }
}

pub fn get_valid_f64<F>(pred: F) -> f32
where
    F: Fn(f32) -> bool,
{
    loop {
        let a = random_f32();
        if pred(a) {
            return a;
        }
    }
}

pub fn get_valid_f32_gen<G, F>(mut gen: G, pred: F) -> f32
where
    G: FnMut() -> f32,
    F: Fn(f32) -> bool,
{
    loop {
        let a = gen();
        if pred(a) {
            return a;
        }
    }
}

pub fn get_twofloat() -> TwoFloat {
    loop {
        if let Ok(result) = TwoFloat::try_from((random_f32(), random_f32())) {
            return result;
        }
    }
}

pub fn try_get_twofloat_with_hi(hi: f32) -> Result<TwoFloat, TwoFloatError> {
    if hi == 0.0 {
        return Ok(TwoFloat::from(0.0));
    }

    for _ in 0..10 {
        let result = TwoFloat::try_from((hi, random_f32() % hi));
        if result.is_ok() {
            return result;
        }
    }

    Err(TwoFloatError::ConversionError {})
}

pub fn try_get_twofloat_with_lo(lo: f32) -> Result<TwoFloat, TwoFloatError> {
    for _ in 0..10 {
        let result = TwoFloat::try_from((random_f32(), lo));
        if result.is_ok() {
            return result;
        }
    }

    Err(TwoFloatError::ConversionError {})
}

pub fn get_valid_twofloat<F>(pred: F) -> TwoFloat
where
    F: Fn(f32, f32) -> bool,
{
    loop {
        let a = random_f32();
        let b = random_f32();
        if !pred(a, b) {
            continue;
        }

        if let Ok(result) = TwoFloat::try_from((a, b)) {
            return result;
        }
    }
}

pub fn get_valid_pair<F>(pred: F) -> (f32, f32)
where
    F: Fn(f32, f32) -> bool,
{
    loop {
        let a = random_f32();
        let b = random_f32();
        if pred(a, b) {
            return (a, b);
        }
    }
}

#[allow(unused_macros)]
macro_rules! assert_eq_ulp {
    ($left:expr, $right:expr, $ulp:expr) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1u31 << 31) == 0 {
                x
            } else {
                x ^ ((1u32 << 31) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i32)
            .saturating_sub(fix_sign(b_bits) as i32)
            .abs();
        if !(diff <= *ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`"#, ulp_val, left_val, right_val, diff)
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt,)+) => ({
        let left_val = $left;
        let right_val = $right;
        let ulp_val = $ulp;

        let a_bits = left_val.to_bits();
        let b_bits = right_val.to_bits();
        let fix_sign = |x| {
            if x & (1u32 << 31) == 0 {
                x
            } else {
                x ^ ((1u32 << 31) - 1)
            }
        };
        let diff = (fix_sign(a_bits) as i32)
            .saturating_sub(fix_sign(b_bits) as i32)
            .abs();
        if !(diff <= ulp_val) {
            panic!(r#"assertion failed: `(left == right) ({:?} ulp)`
  left: `{:?}`,
 right: `{:?}`,
  diff: `{}`: {}"#, ulp_val, left_val, right_val, diff, format_args!($($args,)+))
        }
    });
    ($left:expr, $right:expr, $ulp:expr, $($args:tt),+) => {
        assert_eq_ulp!($left, $right, $ulp, $($args,)+)
    };
}
