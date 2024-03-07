use core::{cmp::Ordering, num::FpCategory};

use hexf::hexf32;

use crate::{math_util::mathfn, TwoFloat};

const DEG_PER_RAD: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.ca5dc2p5"),
    lo: hexf32!("-0x1.1e7ab4p-49"),
};

const RAD_PER_DEG: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.1df46ap-6"),
    lo: hexf32!("0x1.5c1d8cp-62"),
};

const EXPONENT_MASK: u32 = (1 <<  8) - 1;
const MANTISSA_MASK: u32 = (1 << 23) - 1;

/// Checks if two `f32` values do not overlap, with the first value being the
/// more significant. This matches definition 1.4 in Joldes et al. (2017).
///
/// # Examples
///
/// ```
/// # use twofloat::no_overlap;
/// let a = no_overlap(1.0, -1e-200);
/// let b = no_overlap(1e-200, 1.0);
/// let c = no_overlap(1.0, 0.25);
///
/// assert!(a);
/// assert!(!b);
/// assert!(!c);
/// ```
pub fn no_overlap(a: f32, b: f32) -> bool {
    match a.classify() {
        FpCategory::Normal => {
            if b == 0.0 {
                return true;
            }
            let bits = a.to_bits();
            let biased_exponent = ((bits >> 23) & EXPONENT_MASK) as i16;
            let offset = if (bits & MANTISSA_MASK) == 0 && mathfn::signumf(a) != mathfn::signumf(b) {
                (1<<7) + 23 + 1
            } else {
                (1<<7) + 23
            };
            let limit = mathfn::exp2f((biased_exponent - offset) as f32);
            match mathfn::fabsf(b).partial_cmp(&limit) {
                Some(Ordering::Less) => true,
                Some(Ordering::Equal) => (bits & 1) == 0,
                _ => false,
            }
        }
        FpCategory::Subnormal | FpCategory::Zero => b == 0.0,
        _ => false,
    }
}



impl TwoFloat {
    /// Smallest finite `TwoFloat` value.
    pub const MIN: Self = Self {
        hi: f32::MIN,
        lo: hexf32!("-0x1.fffffp+127"),
    };

    /// Smallest positive normal `TwoFloat` value.
    pub const MIN_POSITIVE: Self = Self {
        hi: f32::MIN_POSITIVE,
        lo: 0.0,
    };

    /// Largest finite `TwoFloat` value.
    pub const MAX: Self = Self {
        hi: f32::MAX,
        lo: hexf32!("0x1.fffffep+127"),
    };

    /// Represents an error value equivalent to `f32::NAN`.
    pub const NAN: Self = Self {
        hi: f32::NAN,
        lo: f32::NAN,
    };

    /// Represents the difference between 1.0 and the next representable normal value.
    pub const EPSILON: Self = Self {
        hi: f32::MIN_POSITIVE,
        lo: 0.0,
    };

    /// A positive infinite value
    pub const INFINITY: Self = Self {
        hi: f32::INFINITY,
        lo: f32::INFINITY,
    };

    /// A negative infinite value
    pub const NEG_INFINITY: Self = Self {
        hi: f32::NEG_INFINITY,
        lo: f32::NEG_INFINITY,
    };

    /// Creates a new TwoFloat from a constant `f32` value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// const value: TwoFloat = TwoFloat::from_f32(1.0);
    /// assert_eq!(value.hi(), 1.0);
    /// ```
    pub const fn from_f64(value: f64) -> Self {
        TwoFloat { hi: value as f32, lo: 0.0 }
    }

    /// Returns the high word of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, -1.0e-200);
    /// assert_eq!(value.hi(), 1.0);
    /// ```
    pub fn hi(&self) -> f32 {
        self.hi
    }

    /// Returns the low word of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let value = TwoFloat::new_add(1.0, -1.0e-200);
    /// assert_eq!(value.lo(), -1.0e-200);
    /// ```
    pub fn lo(&self) -> f32 {
        self.lo
    }

    /// Returns `true` if `self` is a valid value, where both components are
    /// finite (not infinity or `NAN`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(1.0, 1.0e-300).is_valid();
    /// let b = TwoFloat::new_mul(1.0e300, 1.0e300).is_valid();
    ///
    /// assert!(a);
    /// assert!(!b);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.hi.is_finite() && self.lo.is_finite() && no_overlap(self.hi, self.lo)
    }

    /// Returns the minimum of two numbers. If one of the arguments is `NAN`,
    /// the other is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(35.2, 1e-84);
    /// let b = TwoFloat::new_add(35.2, -1e-93);
    ///
    /// assert_eq!(a.min(b), b);
    /// ```
    pub fn min(self, other: Self) -> Self {
        if !self.is_valid() {
            other
        } else if !other.is_valid() || self <= other {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two numbers. If one of the arguments is `NAN`,
    /// the other is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(35.2, 1e-84);
    /// let b = TwoFloat::new_add(35.2, -1e-93);
    ///
    /// assert_eq!(a.max(b), a);
    /// ```
    pub fn max(self, other: Self) -> Self {
        if !self.is_valid() {
            other
        } else if !other.is_valid() || self >= other {
            self
        } else {
            other
        }
    }

    /// Converts degrees to radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(90.0);
    /// let b = a.to_radians();
    ///
    /// assert!((b - twofloat::consts::FRAC_PI_2).abs() < 1e-16);
    /// ```
    pub fn to_radians(self) -> Self {
        self * RAD_PER_DEG
    }

    /// Converts radians to degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = twofloat::consts::PI;
    /// let b = a.to_degrees();
    ///
    /// assert!((b - 180.0).abs() < 1e-16);
    /// ```
    pub fn to_degrees(self) -> Self {
        self * DEG_PER_RAD
    }

    /// Takes the reciprocal (inverse) of the number, `1/x`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::new_add(67.2, 5.7e-53);
    /// let b = a.recip();
    /// let difference = b.recip() - a;
    ///
    /// assert!(difference.abs() < 1e-16);
    /// ```
    pub fn recip(self) -> Self {
        1.0 / self
    }

    /// Raises the number to an integer power. Returns a NAN value for 0^0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0).powi(3);
    /// let b = TwoFloat::from(0.0).powi(0);
    ///
    /// assert!(a - TwoFloat::from(8.0) <= 1e-16);
    /// assert!(!b.is_valid());
    /// ```
    pub fn powi(self, n: i32) -> Self {
        match n {
            0 => {
                if self.hi == 0.0 && self.lo == 0.0 {
                    Self::NAN
                } else {
                    Self::from(1.0)
                }
            }
            1 => self,
            -1 => self.recip(),
            _ => {
                let mut result = Self::from(1.0);
                let mut n_pos = n.abs();
                let mut value = self;
                while n_pos > 0 {
                    if (n_pos & 1) != 0 {
                        result *= &value;
                    }
                    value *= value;
                    n_pos >>= 1;
                }
                if n > 0 {
                    result
                } else {
                    result.recip()
                }
            }
        }
    }
}

impl PartialEq<f32> for TwoFloat {
    fn eq(&self, other: &f32) -> bool {
        self.hi.eq(other) && self.lo == 0.0
    }
}

impl PartialEq<TwoFloat> for f32 {
    fn eq(&self, other: &TwoFloat) -> bool {
        self.eq(&other.hi) && other.lo == 0.0
    }
}

impl PartialEq<TwoFloat> for TwoFloat {
    fn eq(&self, other: &TwoFloat) -> bool {
        if self.is_valid() != other.is_valid()
            || self.hi.is_nan()
            || self.lo.is_nan()
            || other.hi.is_nan()
            || self.lo.is_nan()
        {
            false
        } else if self.is_valid() {
            self.hi == other.hi && self.lo == other.lo
        } else {
            // all infinities compare equal
            true
        }
    }
}

impl PartialOrd<f32> for TwoFloat {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        let hi_cmp = self.hi.partial_cmp(other);
        if hi_cmp == Some(Ordering::Equal) {
            self.lo.partial_cmp(&0.0)
        } else {
            hi_cmp
        }
    }
}

impl PartialOrd<TwoFloat> for f32 {
    fn partial_cmp(&self, other: &TwoFloat) -> Option<Ordering> {
        let hi_cmp = self.partial_cmp(&other.hi);
        if hi_cmp == Some(Ordering::Equal) {
            0.0.partial_cmp(&other.lo)
        } else {
            hi_cmp
        }
    }
}

impl PartialOrd<TwoFloat> for TwoFloat {
    fn partial_cmp(&self, other: &TwoFloat) -> Option<Ordering> {
        if self.hi.is_nan() || self.lo.is_nan() || other.hi.is_nan() || other.lo.is_nan() {
            return None;
        }

        match (self.is_valid(), other.is_valid()) {
            (true, true) => {
                let hi_cmp = self.hi.partial_cmp(&other.hi);
                if matches!(hi_cmp, Some(Ordering::Equal)) {
                    self.lo.partial_cmp(&other.lo)
                } else {
                    hi_cmp
                }
            }
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod tests {
    use hexf::hexf32;

    use super::{no_overlap, TwoFloat};

    const ONE: f32 = 1.0;
    const ONE_NEXT: f32 =   hexf32!("0x1.000002p+0");
    const ONE_NEXT_2: f32 = hexf32!("0x1.000004p+0");
    const ONE_PREV: f32 = hexf32!("0x1.fffffep-1");
    const LOWER_MID_DIFF: f32 = hexf32!("0x1p-25");
    const LOWER_MID_DIFF_NEXT: f32 = hexf32!("0x1.000002p-54");
    const UPPER_MID_DIFF: f32 = hexf32!("0x1p-24");
    const UPPER_MID_DIFF_NEXT: f32 = hexf32!("0x1.000002p-24");
    const OFFSET_1_4: f32 = hexf32!("0x1p-25");
    const OFFSET_3_4: f32 = hexf32!("0x1.8p-24");

    #[test]
    fn no_overlap_test() {
        assert!(!no_overlap(1.0, hexf32!("0x1p-23")));
        assert!(!no_overlap(-1.0, hexf32!("-0x1p-23")));
        assert!(no_overlap(1.0, UPPER_MID_DIFF));
        assert!(!no_overlap(1.0, UPPER_MID_DIFF_NEXT));
        assert!(no_overlap(1.0, -LOWER_MID_DIFF));
        assert!(!no_overlap(1.0, -LOWER_MID_DIFF_NEXT));
        assert!(!no_overlap(1.0, -UPPER_MID_DIFF));
        assert!(!no_overlap(ONE_NEXT, UPPER_MID_DIFF));
        assert!(!no_overlap(ONE_NEXT, -UPPER_MID_DIFF));
        assert!(no_overlap(ONE_NEXT_2, UPPER_MID_DIFF));
        assert!(no_overlap(ONE_NEXT_2, -UPPER_MID_DIFF));
        assert!(no_overlap(-1.0, LOWER_MID_DIFF));
        assert!(!no_overlap(-1.0, LOWER_MID_DIFF_NEXT));
        assert!(!no_overlap(-1.0, UPPER_MID_DIFF));
        assert!(no_overlap(-1.0, -UPPER_MID_DIFF));
        assert!(!no_overlap(-1.0, -UPPER_MID_DIFF_NEXT));
        assert!(!no_overlap(-ONE_NEXT, hexf32!("0x1p-24")));
        assert!(!no_overlap(-ONE_NEXT, hexf32!("-0x1p-24")));
        assert!(no_overlap(-ONE_NEXT_2, hexf32!("-0x1p-24")));
        assert!(no_overlap(-ONE_NEXT_2, hexf32!("0x1p-24")));
        assert!(no_overlap(1.0, hexf32!("0x1p-127")));
        assert!(no_overlap(1.0, hexf32!("-0x1p-127")));
        assert!(no_overlap(1.0, 0.0));
        assert!(no_overlap(-1.0, -0.0));

        assert!(!no_overlap(hexf32!("0x1p-97"), hexf32!("0x1p-126")));
        assert!(no_overlap(hexf32!("0x1p-97"), hexf32!("0x1p-127")));
        assert!(!no_overlap(hexf32!("0x1p-98"), hexf32!("0x1p-127")));
        assert!(no_overlap(hexf32!("0x1p-98"), hexf32!("0x1p-128")));

        assert!(no_overlap(hexf32!("0x1p-127"), 0.0));
        assert!(!no_overlap(hexf32!("0x1p-127"), f32::MIN));

        assert!(!no_overlap(f32::INFINITY, 1.0));
        assert!(!no_overlap(f32::NAN, 1.0));

        assert!(!no_overlap(0.0, 1.0));
        assert!(!no_overlap(0.0, f32::MIN));
        assert!(no_overlap(0.0, 0.0));
    }

    #[test]
    fn default_test() {
        let value: TwoFloat = Default::default();
        assert_eq!(value, TwoFloat::from(0));
    }

    #[test]
    fn min_test() {
        assert!(TwoFloat::MIN.is_valid());
    }

    #[test]
    fn max_test() {
        assert!(TwoFloat::MAX.is_valid());
    }

    #[test]
    fn midpoint_eq_test() {
        let values = [
            TwoFloat::new_add(ONE, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE,
                lo: UPPER_MID_DIFF,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn midpoint_eq_test_next() {
        let values = [
            TwoFloat::new_add(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT_2, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT_2, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE_NEXT_2,
                lo: -UPPER_MID_DIFF,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn midpoint_eq_test_prev() {
        let values = [
            TwoFloat::new_add(ONE, -LOWER_MID_DIFF),
            TwoFloat::new_add(ONE_PREV, LOWER_MID_DIFF),
            TwoFloat::new_sub(ONE, LOWER_MID_DIFF),
            TwoFloat::new_sub(ONE_PREV, -LOWER_MID_DIFF),
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn quarter_eq_test() {
        let values = [
            TwoFloat::new_add(ONE, OFFSET_3_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_1_4),
            TwoFloat::new_sub(ONE, -OFFSET_3_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_1_4),
            TwoFloat {
                hi: ONE_NEXT,
                lo: -OFFSET_1_4,
            },
        ];

        assert!(values.iter().all(|v| v.is_valid()));
        values
            .iter()
            .for_each(|&a| values.iter().for_each(|&b| assert_eq!(a, b)));
    }

    #[test]
    fn ord_test() {
        let lower_values = [
            TwoFloat::new_add(ONE, OFFSET_1_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_3_4),
            TwoFloat::new_sub(ONE, -OFFSET_1_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_3_4),
            TwoFloat {
                hi: ONE,
                lo: OFFSET_1_4,
            },
        ];
        assert!(lower_values.iter().all(|v| v.is_valid()));

        let mid_values = [
            TwoFloat::new_add(ONE, UPPER_MID_DIFF),
            TwoFloat::new_add(ONE_NEXT, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE, -UPPER_MID_DIFF),
            TwoFloat::new_sub(ONE_NEXT, UPPER_MID_DIFF),
            TwoFloat {
                hi: ONE,
                lo: UPPER_MID_DIFF,
            },
        ];
        assert!(mid_values.iter().all(|v| v.is_valid()));

        let upper_values = [
            TwoFloat::new_add(ONE, OFFSET_3_4),
            TwoFloat::new_add(ONE_NEXT, -OFFSET_1_4),
            TwoFloat::new_sub(ONE, -OFFSET_3_4),
            TwoFloat::new_sub(ONE_NEXT, OFFSET_1_4),
            TwoFloat {
                hi: ONE_NEXT,
                lo: -OFFSET_1_4,
            },
        ];
        assert!(upper_values.iter().all(|v| v.is_valid()));

        lower_values.iter().for_each(|&a| {
            mid_values.iter().for_each(|&b| assert!(a < b));
            upper_values.iter().for_each(|&b| assert!(a < b));
        });

        mid_values.iter().for_each(|&a| {
            lower_values.iter().for_each(|&b| assert!(a > b));
            upper_values.iter().for_each(|&b| assert!(a < b));
        });

        upper_values.iter().for_each(|&a| {
            lower_values.iter().for_each(|&b| assert!(a > b));
            mid_values.iter().for_each(|&b| assert!(a > b));
        });
    }
}
