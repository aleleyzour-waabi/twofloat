use core::convert::TryFrom;

use hexf::hexf32;

use crate::{
    consts::{FRAC_PI_2, FRAC_PI_4, PI},
    TwoFloat,
};

// Polynomial coefficients of sin(x)-x on [0,pi/4]
const SIN_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: hexf32!("-0x1.555556p-3"),
        lo: hexf32!("0x1.555556p-28"),
    },
    TwoFloat {
        hi: hexf32!("0x1.111112p-7"),
        lo: hexf32!("-0x1.ddddep-32"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.a01a02p-13"),
        lo: hexf32!("0x1.7fa07ep-39"),
    },
    TwoFloat {
        hi: hexf32!("0x1.71de3ap-19"),
        lo: hexf32!("0x1.498c54p-45"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.ae6452p-26"),
        lo: hexf32!("0x1.4a55c6p-52"),
    },
    TwoFloat {
        hi: hexf32!("0x1.61201p-33"),
        lo: hexf32!("0x1.e6c7dp-58"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.aa6c44p-41"),
        lo: hexf32!("0x1.d5d212p-66"),
    },
];

// Polynomial coefficients of cos(x)-1+x^2/2 on [0,pi/4]
const COS_COEFFS: [TwoFloat; 7] = [
    TwoFloat {
        hi: hexf32!("0x1.555556p-5"),
        lo: hexf32!("-0x1.555556p-30"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.6c16c2p-10"),
        lo: hexf32!("0x1.27d27ep-35"),
    },
    TwoFloat {
        hi: hexf32!("0x1.a01a02p-16"),
        lo: hexf32!("-0x1.7f9d0cp-42"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.27e4fcp-22"),
        lo: hexf32!("0x1.143ffap-47"),
    },
    TwoFloat {
        hi: hexf32!("0x1.1eed8cp-29"),
        lo: hexf32!("0x1.0f4b4ap-54"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.93931cp-37"),
        lo: hexf32!("-0x1.52d784p-62"),
    },
    TwoFloat {
        hi: hexf32!("0x1.aabaa8p-45"),
        lo: hexf32!("0x1.65c67p-75"),
    },
];

// Polynomial coefficients of tan(x)-x on [0,pi/4]
const TAN_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: hexf32!("0x1.555556p-2"),
        lo: hexf32!("-0x1.55559ep-27"),
    },
    TwoFloat {
        hi: hexf32!("0x1.111112p-3"),
        lo: hexf32!("-0x1.dda77ep-28"),
    },
    TwoFloat {
        hi: hexf32!("0x1.ba1ba2p-5"),
        lo: hexf32!("-0x1.59ec58p-31"),
    },
    TwoFloat {
        hi: hexf32!("0x1.664f4cp-6"),
        lo: hexf32!("-0x1.78b602p-31"),
    },
    TwoFloat {
        hi: hexf32!("0x1.226deep-7"),
        lo: hexf32!("-0x1.f8c99ep-32"),
    },
    TwoFloat {
        hi: hexf32!("0x1.d6ddbp-9"),
        lo: hexf32!("-0x1.7dfeb6p-34"),
    },
    TwoFloat {
        hi: hexf32!("0x1.7d2be8p-10"),
        lo: hexf32!("0x1.b3c2ecp-35"),
    },
    TwoFloat {
        hi: hexf32!("0x1.395c8cp-11"),
        lo: hexf32!("-0x1.0c9c34p-36"),
    },
    TwoFloat {
        hi: hexf32!("0x1.c3c79cp-13"),
        lo: hexf32!("0x1.b57be8p-38"),
    },
    TwoFloat {
        hi: hexf32!("0x1.399daep-13"),
        lo: hexf32!("-0x1.3783c6p-41"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.6a82acp-15"),
        lo: hexf32!("0x1.51ade6p-40"),
    },
    TwoFloat {
        hi: hexf32!("0x1.b3221ep-14"),
        lo: hexf32!("-0x1.24e76ap-39"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.b4a2a4p-15"),
        lo: hexf32!("0x1.7eeb0cp-42"),
    },
    TwoFloat {
        hi: hexf32!("0x1.7917ap-16"),
        lo: hexf32!("0x1.7227e4p-42"),
    },
];

// Polynomial coefficients of asin(x)-x on [0,0.5]
const ASIN_COEFFS: [TwoFloat; 10] = [
    TwoFloat {
        hi: hexf32!("0x1.555556p-3"),
        lo: hexf32!("-0x1.55f4aep-28"),
    },
    TwoFloat {
        hi: hexf32!("0x1.333334p-4"),
        lo: hexf32!("-0x1.f3da76p-30"),
    },
    TwoFloat {
        hi: hexf32!("0x1.6db6bcp-5"),
        lo: hexf32!("-0x1.8a85eep-30"),
    },
    TwoFloat {
        hi: hexf32!("0x1.f1ce02p-6"),
        lo: hexf32!("-0x1.a1d4aap-31"),
    },
    TwoFloat {
        hi: hexf32!("0x1.6e1af8p-6"),
        lo: hexf32!("0x1.65d05p-31"),
    },
    TwoFloat {
        hi: hexf32!("0x1.20d826p-6"),
        lo: hexf32!("0x1.4d49b4p-31"),
    },
    TwoFloat {
        hi: hexf32!("0x1.8d6db6p-7"),
        lo: hexf32!("0x1.9e2b38p-34"),
    },
    TwoFloat {
        hi: hexf32!("0x1.3c048p-6"),
        lo: hexf32!("-0x1.533276p-31"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.401c6ap-7"),
        lo: hexf32!("0x1.7bb1bap-33"),
    },
    TwoFloat {
        hi: hexf32!("0x1.119828p-5"),
        lo: hexf32!("-0x1.749e56p-31"),
    },
];

// Polynomial coefficients of atan(x) - x on [0, 7/16]
const ATAN_COEFFS: [TwoFloat; 15] = [
    TwoFloat {
        hi: hexf32!("-0x1.555556p-2"),
        lo: hexf32!("0x1.555556p-27"),
    },
    TwoFloat {
        hi: hexf32!("0x1.99999ap-3"),
        lo: hexf32!("-0x1.99999ap-29"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.24924ap-3"),
        lo: hexf32!("0x1.b6db86p-28"),
    },
    TwoFloat {
        hi: hexf32!("0x1.c71c72p-4"),
        lo: hexf32!("-0x1.c757f2p-31"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.745d18p-4"),
        lo: hexf32!("0x1.77482p-29"),
    },
    TwoFloat {
        hi: hexf32!("0x1.3b13b2p-4"),
        lo: hexf32!("-0x1.ec37acp-29"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.11110cp-4"),
        lo: hexf32!("-0x1.062eaap-29"),
    },
    TwoFloat {
        hi: hexf32!("0x1.e1e146p-5"),
        lo: hexf32!("-0x1.184c38p-30"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.af20aep-5"),
        lo: hexf32!("-0x1.3002ecp-33"),
    },
    TwoFloat {
        hi: hexf32!("0x1.85cf5ep-5"),
        lo: hexf32!("0x1.94240ep-30"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.622b3ep-5"),
        lo: hexf32!("-0x1.19952cp-30"),
    },
    TwoFloat {
        hi: hexf32!("0x1.3d3eaap-5"),
        lo: hexf32!("-0x1.d8156cp-30"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.07d294p-5"),
        lo: hexf32!("0x1.192a0cp-32"),
    },
    TwoFloat {
        hi: hexf32!("0x1.5f9188p-6"),
        lo: hexf32!("0x1.abf73p-33"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.09daeep-7"),
        lo: hexf32!("-0x1.1d8a9cp-33"),
    },
];

const ATAN_FRAC_1_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.dac67p-2"),
    lo: hexf32!("0x1.586ed4p-28"),
};

const ATAN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.f730bep-1"),
    lo: hexf32!("-0x1.afc12cp-26"),
};

fn quadrant(value: TwoFloat) -> (TwoFloat, i8) {
    if value.abs() < FRAC_PI_4 {
        (value, 0)
    } else {
        let quotient = (value / FRAC_PI_2).round();
        let remainder = value - quotient * FRAC_PI_2;
        match i8::try_from(quotient % 4.0) {
            Ok(quadrant) if quadrant >= 0 => (remainder, quadrant),
            Ok(quadrant) if quadrant >= -4 => (remainder, 4 + quadrant),
            _ => (TwoFloat::NAN, 0),
        }
    }
}

fn restricted_sin(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, SIN_COEFFS)
}

fn restricted_cos(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    polynomial!(x2, 1.0, -0.5, COS_COEFFS)
}

fn restricted_tan(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, TAN_COEFFS)
}

fn restricted_asin(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, ASIN_COEFFS)
}

fn restricted_atan(x: TwoFloat) -> TwoFloat {
    let x2 = x * x;
    x * polynomial!(x2, 1.0, ATAN_COEFFS)
}

impl TwoFloat {
    /// Computes the sine of the value (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.sin();
    /// let c = 2.5f64.sin();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn sin(self) -> Self {
        if !self.is_valid() {
            return Self::NAN;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_sin(x),
            1 => restricted_cos(x),
            2 => -restricted_sin(x),
            _ => -restricted_cos(x),
        }
    }

    /// Computes the cosine of the value (in radians)
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.cos();
    /// let c = 2.5f64.cos();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn cos(self) -> Self {
        if !self.is_valid() {
            return Self::NAN;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 => restricted_cos(x),
            1 => -restricted_sin(x),
            2 => -restricted_cos(x),
            _ => restricted_sin(x),
        }
    }

    /// Simultaneously computes the sine and cosine of the value. Returns a
    /// tuple with the sine as the first element and the cosine as the second
    /// element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let (s, c) = a.sin_cos();
    ///
    /// assert!((s - 2.5f64.sin()).abs() < 1e-10);
    /// assert!((c - 2.5f64.cos()).abs() < 1e-10);
    /// ```
    pub fn sin_cos(self) -> (Self, Self) {
        if !self.is_valid() {
            return (Self::NAN, Self::NAN);
        }
        let (x, quadrant) = quadrant(self);
        let s = restricted_sin(x);
        let c = restricted_cos(x);
        match quadrant {
            0 => (s, c),
            1 => (c, -s),
            2 => (-s, -c),
            _ => (-c, s),
        }
    }

    /// Computes the tangent of the value (in radians).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.5);
    /// let b = a.tan();
    /// let c = 2.5f64.tan();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn tan(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        let (x, quadrant) = quadrant(self);
        match quadrant {
            0 | 2 => restricted_tan(x),
            _ => -1.0 / restricted_tan(x),
        }
    }

    /// Computes the arcsine of the value. Return value is in radians in the
    /// range [-π/2, π/2] or an invalid value if the input value is outside
    /// the range [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.7);
    /// let b = a.asin();
    /// let c = 0.7f64.asin();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn asin(self) -> Self {
        let abs_val = self.abs();
        if !self.is_valid() || abs_val > 1.0 {
            Self::NAN
        } else if abs_val <= 0.5 {
            restricted_asin(self)
        } else {
            let result = FRAC_PI_2 - 2.0 * restricted_asin(((1.0 - self.abs()) / 2.0).sqrt());
            if self.is_sign_positive() {
                result
            } else {
                -result
            }
        }
    }

    /// Computes the arccosine of the value. Return value is in radians in
    /// the range [0, π] or an invalid value if the input value is outside
    /// the range [-1, 1].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(-0.8);
    /// let b = a.acos();
    /// let c = (-0.8f64).acos();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn acos(self) -> Self {
        let x = self.asin();
        if x.is_valid() {
            FRAC_PI_2 - x
        } else {
            x
        }
    }

    /// Computes the arctangent of the value. Return value is in radians in
    /// the range [-π/2, π/2].
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(3.5);
    /// let b = a.atan();
    /// let c = 3.5f64.atan();
    ///
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn atan(self) -> Self {
        if !self.is_valid() {
            Self::NAN
        } else if self.hi.is_infinite() {
            if self.hi.is_sign_positive() {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            }
        } else {
            let x = self.abs();
            let k = 4.0 * x + 0.25;
            if k <= 2.0 {
                return restricted_atan(self);
            }

            let result = if k < 3.0 {
                ATAN_FRAC_1_2 + restricted_atan((x - 0.5) / (1.0 + 0.5 * x))
            } else if k < 5.0 {
                FRAC_PI_4 + restricted_atan((x - 1.0) / (1.0 + x))
            } else if k < 10.0 {
                ATAN_FRAC_3_2 + restricted_atan((x - 1.5) / (1.0 + 1.5 * x))
            } else {
                FRAC_PI_2 - restricted_atan(x.recip())
            };

            if self.is_sign_positive() {
                result
            } else {
                -result
            }
        }
    }

    /// Computes the four quadrant arctangent of `self` (y) and `other` (x)
    /// in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let y = TwoFloat::from(-1.0);
    /// let x = TwoFloat::from(-1.0);
    /// let theta = TwoFloat::atan2(y, x);
    ///
    /// assert!((theta + 3.0 * twofloat::consts::FRAC_PI_4).abs() < 1e-10);
    /// ```
    pub fn atan2(self, other: Self) -> Self {
        if self.hi == 0.0 {
            if other.hi.is_sign_positive() {
                Self::from(0.0)
            } else if self.hi.is_sign_positive() {
                PI
            } else {
                -PI
            }
        } else if other.hi == 0.0 {
            if self.hi.is_sign_positive() {
                FRAC_PI_2
            } else {
                -FRAC_PI_2
            }
        } else {
            let a = (self / other).atan();
            if other.hi.is_sign_positive() {
                a
            } else if self.hi.is_sign_positive() {
                a + PI
            } else {
                a - PI
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::quadrant;
    use crate::{
        consts::{FRAC_PI_2, FRAC_PI_4, PI},
        TwoFloat,
    };

    const THRESHOLD: f32 = 1e-6;

    fn approx_eq(a: f64, b: TwoFloat) -> bool {
        const THRESHOLD64: f64 = 1e-10;
        (a - f64::from(b)).abs() < THRESHOLD64
    }

    #[test]
    fn quadrant_test() {
        assert_eq!(0, quadrant(TwoFloat::from(0.5)).1);
        assert_eq!(0, quadrant(TwoFloat::from(-0.5)).1);

        assert_eq!(1, quadrant(TwoFloat::from(2.0)).1);
        assert_eq!(3, quadrant(TwoFloat::from(-2.0)).1);

        assert_eq!(2, quadrant(TwoFloat::from(3.14)).1);
        assert_eq!(2, quadrant(TwoFloat::from(-3.14)).1);

        assert_eq!(3, quadrant(TwoFloat::from(4.0)).1);
        assert_eq!(1, quadrant(TwoFloat::from(-4.0)).1);

        assert_eq!(0, quadrant(TwoFloat::from(6.0)).1);
        assert_eq!(0, quadrant(TwoFloat::from(-6.0)).1);
    }

    #[test]
    fn sin_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).sin());

        assert!(approx_eq(0.5f64.sin(), TwoFloat::from(0.5).sin()));
        assert!(approx_eq(1.4f64.sin(), TwoFloat::from(1.4).sin()));
        assert!(approx_eq(3.0f64.sin(), TwoFloat::from(3.0).sin()));
        assert!(approx_eq(4.0f64.sin(), TwoFloat::from(4.0).sin()));
        assert!(approx_eq(6.0f64.sin(), TwoFloat::from(6.0).sin()));

        assert!(approx_eq(0.5f64.sin(), TwoFloat::from(-0.5).sin()));
        assert!(approx_eq(1.4f64.sin(), TwoFloat::from(-1.4).sin()));
        assert!(approx_eq(3.0f64.sin(), TwoFloat::from(-3.0).sin()));
        assert!(approx_eq(4.0f64.sin(), TwoFloat::from(-4.0).sin()));
        assert!(approx_eq(6.0f64.sin(), TwoFloat::from(-6.0).sin()));
    }

    #[test]
    fn cos_test() {
        assert_eq!(1.0, TwoFloat::from(0.0).cos());

        assert!(approx_eq(0.5f64.cos(), TwoFloat::from(0.5).cos()));
        assert!(approx_eq(1.4f64.cos(), TwoFloat::from(1.4).cos()));
        assert!(approx_eq(3.0f64.cos(), TwoFloat::from(3.0).cos()));
        assert!(approx_eq(4.0f64.cos(), TwoFloat::from(4.0).cos()));
        assert!(approx_eq(6.0f64.cos(), TwoFloat::from(6.0).cos()));

        assert!(approx_eq(0.5f64.cos(), TwoFloat::from(-0.5).cos()));
        assert!(approx_eq(1.4f64.cos(), TwoFloat::from(-1.4).cos()));
        assert!(approx_eq(3.0f64.cos(), TwoFloat::from(-3.0).cos()));
        assert!(approx_eq(4.0f64.cos(), TwoFloat::from(-4.0).cos()));
        assert!(approx_eq(6.0f64.cos(), TwoFloat::from(-6.0).cos()));
    }

    #[test]
    fn tan_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).tan());

        assert!(approx_eq(0.5f64.tan(), TwoFloat::from(0.5).tan()));
        assert!(approx_eq(1.4f64.tan(), TwoFloat::from(1.4).tan()));
        assert!(approx_eq(3.0f64.tan(), TwoFloat::from(3.0).tan()));
        assert!(approx_eq(4.0f64.tan(), TwoFloat::from(4.0).tan()));
        assert!(approx_eq(6.0f64.tan(), TwoFloat::from(6.0).tan()));

        assert!(approx_eq(0.5f64.tan(), TwoFloat::from(-0.5).tan()));
        assert!(approx_eq(1.4f64.tan(), TwoFloat::from(-1.4).tan()));
        assert!(approx_eq(3.0f64.tan(), TwoFloat::from(-3.0).tan()));
        assert!(approx_eq(4.0f64.tan(), TwoFloat::from(-4.0).tan()));
        assert!(approx_eq(6.0f64.tan(), TwoFloat::from(-6.0).tan()));
    }

    #[test]
    fn asin_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).asin());
        assert!((0.25f64.asin() - TwoFloat::from(0.25).asin()) < THRESHOLD);
        assert!((0.75f64.asin() - TwoFloat::from(0.75).asin()) < THRESHOLD);
        assert!((TwoFloat::from(1.0).asin() - FRAC_PI_2).abs() < THRESHOLD);

        assert!((0.25f64.asin() + TwoFloat::from(-0.25).asin()) < THRESHOLD);
        assert!((0.75f64.asin() + TwoFloat::from(-0.75).asin()) < THRESHOLD);
        assert!((TwoFloat::from(-1.0).asin() + FRAC_PI_2).abs() < THRESHOLD);
    }

    #[test]
    fn acos_test() {
        assert!((TwoFloat::from(0.0).acos() - FRAC_PI_2).abs() < THRESHOLD);

        assert!((0.25f64.acos() - TwoFloat::from(0.25).acos()) < THRESHOLD);
        assert!((0.75f64.acos() - TwoFloat::from(0.75).acos()) < THRESHOLD);
        assert_eq!(0.0, TwoFloat::from(1.0).acos());

        assert!((0.25f64.asin() - TwoFloat::from(-0.25).acos()) < THRESHOLD);
        assert!((0.75f64.asin() - TwoFloat::from(-0.75).acos()) < THRESHOLD);
        assert!((TwoFloat::from(-1.0).acos() - PI).abs() < THRESHOLD);
    }

    #[test]
    fn atan_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).atan());

        assert!((0.25f64.atan() - TwoFloat::from(0.25).atan()).abs() < THRESHOLD);
        assert!((0.5f64.atan() - TwoFloat::from(0.5).atan()).abs() < THRESHOLD);
        assert!((FRAC_PI_4 - TwoFloat::from(1.0).atan()).abs() < THRESHOLD);
        assert!((2.25f64.atan() - TwoFloat::from(2.25).atan()).abs() < THRESHOLD);
        assert!((10.0f64.atan() - TwoFloat::from(10.0).atan()).abs() < THRESHOLD);

        assert!((0.25f64.atan() + TwoFloat::from(-0.25).atan()).abs() < THRESHOLD);
        assert!((0.5f64.atan() + TwoFloat::from(-0.5).atan()).abs() < THRESHOLD);
        assert!((FRAC_PI_4 + TwoFloat::from(-1.0).atan()).abs() < THRESHOLD);
        assert!((2.25f64.atan() + TwoFloat::from(-2.25).atan()).abs() < THRESHOLD);
        assert!((10.0f64.atan() + TwoFloat::from(-10.0).atan()).abs() < THRESHOLD);
    }

    #[test]
    fn atan2_test() {
        assert_eq!(0.0, TwoFloat::from(0.0).atan2(TwoFloat::from(0.0)));
        assert_eq!(0.0, TwoFloat::from(0.0).atan2(TwoFloat::from(1.0)));
        assert_eq!(PI, TwoFloat::from(0.0).atan2(TwoFloat::from(-1.0)));
        assert_eq!(-PI, TwoFloat::from(-0.0).atan2(TwoFloat::from(-1.0)));
        assert_eq!(FRAC_PI_2, TwoFloat::from(1.0).atan2(TwoFloat::from(0.0)));
        assert_eq!(-FRAC_PI_2, TwoFloat::from(-1.0).atan2(TwoFloat::from(0.0)));
        assert!(
            (0.73f64.atan2(0.21f64) - TwoFloat::from(0.73).atan2(TwoFloat::from(0.21))).abs()
                < THRESHOLD
        );
    }
}
