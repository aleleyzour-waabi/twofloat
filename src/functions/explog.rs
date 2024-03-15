use hexf::hexf32;

use crate::{consts::LN_2, TwoFloat};
use crate::math_util::mathfn;

// 1/ln(2)
const FRAC_1_LN_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.71547652b82fep0"),
    lo: hexf64!("0x1.777d0ffda0d24p-56"),
};

// ln(10)
const LN_10: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.26bb1bbb55516p1"),
    lo: hexf64!("-0x1.f48ad494ea3e9p-53"),
};

// ln(3/2)
const LN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.9f323ecbf984cp-2"),
    lo: hexf64!("-0x1.a92e513217f5cp-59"),
};

// limits
const EXP_UPPER_LIMIT: f32 = hexf32!("0x1.62e430p9"); // ln(0x1.0p128) as f32
const EXP_LOWER_LIMIT: f32 = hexf32!("-0x1.743854p9"); // ln(0x1.0p-127) as f32

// Coefficients for polynomial approximation of x*(exp(x)+1)/(exp(x)-1)
const EXP_COEFFS: [TwoFloat; 6] = [
    TwoFloat {
        hi: hexf32!("0x1.555556p-3"),
        lo: hexf32!("-0x1.555556p-28"),
	},
    TwoFloat {
        hi: hexf32!("-0x1.6c16c2p-9"),
        lo: hexf32!("0x1.27d2a2p-34"),
    },
    TwoFloat {
        hi: hexf32!("0x1.1566acp-14"),
        lo: hexf32!("-0x1.0182c2p-40"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.bbd776p-20"),        
		lo: hexf32!("-0x1.196a5p-45"),
    },
    TwoFloat {
        hi: hexf32!("0x1.66a4e6p-25"),        
		lo: hexf32!("-0x1.d9ee04p-52"),
    },
    TwoFloat {
        hi: hexf32!("-0x1.1f6dc2p-30"),        
		lo: hexf32!("0x1.d599f4p-60"),
    },
];

const EXP_M1_COEFFS: [TwoFloat; 12] = [
    TwoFloat {
        hi: hexf32!("0x1p-1"),
        lo: hexf32!("0x1.bd7304p-56"),
	},
    TwoFloat {
        hi: hexf32!("0x1.555556p-3"),
        lo: hexf32!("-0x1.555556p-28"),
	},
    TwoFloat {
        hi: hexf32!("0x1.555556p-5"),
        lo: hexf32!("-0x1.555582p-30"),
	},
    TwoFloat {
        hi: hexf32!("0x1.111112p-7"),
        lo: hexf32!("-0x1.dddd48p-32"),
	},
    TwoFloat {
        hi: hexf32!("0x1.6c16c2p-10"),
        lo: hexf32!("-0x1.2731ecp-35"),
	},
    TwoFloat {
        hi: hexf32!("0x1.a01a02p-13"),
        lo: hexf32!("-0x1.7d25a4p-39"),
	},
    TwoFloat {
        hi: hexf32!("0x1.a01ap-16"),
        lo: hexf32!("0x1.5d650cp-41"),
	},
    TwoFloat {
        hi: hexf32!("0x1.71de32p-19"),
        lo: hexf32!("0x1.60a154p-44"),
	},
    TwoFloat {
        hi: hexf32!("0x1.27e62ep-22"),
        lo: hexf32!("-0x1.fc994cp-49"),
	},
    TwoFloat {
        hi: hexf32!("0x1.ae852ep-26"),
        lo: hexf32!("-0x1.d7be22p-51"),
	},
    TwoFloat {
		
        hi: hexf32!("0x1.1e22aap-29"),
        lo: hexf32!("0x1.bb432ep-54"),
	},
    TwoFloat {
        hi: hexf32!("0x1.36ab7p-33"),
        lo: hexf32!("-0x1.106d44p-58"),
	},
];

// Coefficients for polynomial approximation of 2^x on [-0.5, 0.5]
const EXP2_COEFFS: [TwoFloat; 14] = [
    TwoFloat {
        hi: hexf32!("0x1.62e43p-1"),
        lo: hexf32!("-0x1.05c61p-29"),
	},
    TwoFloat {
        hi: hexf32!("0x1.ebfbep-3"),
        lo: hexf32!("-0x1.f4e9c4p-33"),
	},
    TwoFloat {
        hi: hexf32!("0x1.c6b08ep-5"),
        lo: hexf32!("-0x1.1f6be8p-30"),
	},
    TwoFloat {
        hi: hexf32!("0x1.3b2ab6p-7"),
        lo: hexf32!("0x1.f749cep-32"),
	},
    TwoFloat {
        hi: hexf32!("0x1.5d87fep-10"),
        lo: hexf32!("0x1.e299cep-36"),
	},
    TwoFloat {
        hi: hexf32!("0x1.430912p-13"),
        lo: hexf32!("0x1.f0d8f2p-38"),
	},
    TwoFloat {
        hi: hexf32!("0x1.ffcbfcp-17"),
        lo: hexf32!("0x1.621fc6p-43"),
	},
    TwoFloat {
        hi: hexf32!("0x1.62c022p-20"),
        lo: hexf32!("0x1.d2d36ep-47"),
	},
    TwoFloat {
        hi: hexf32!("0x1.b5253ep-24"),
        lo: hexf32!("-0x1.6ee866p-49"),
	},
    TwoFloat {
        hi: hexf32!("0x1.e4cf52p-28"),
        lo: hexf32!("-0x1.2dbbc6p-53"),
	},
    TwoFloat {
        hi: hexf32!("0x1.e8ca78p-32"),
        lo: hexf32!("-0x1.01b71cp-58"),
	},
    TwoFloat {
        hi: hexf32!("0x1.c3bd1cp-36"),
        lo: hexf32!("0x1.b35c3p-61"),
	},
    TwoFloat {
        hi: hexf32!("0x1.823566p-40"),
        lo: hexf32!("-0x1.c071f6p-65"),
	},
    TwoFloat {
        hi: hexf32!("0x1.31efcap-44"),
        lo: hexf32!("0x1.64e6e4p-69"),
	},

];

fn mul_pow2(mut x: f32, mut y: i32) -> f32 {
    //const EXP_LEN=8;
    //const FRAC_LEN=23;
    loop {
        // (2^(EXP_LEN -1 /*sign*/)) + (FRAC_LEN -1) = 127 + 22= 149
        if y < -149 {
            x *= hexf32!("0x1p-149");
            y += 149;
            // (2^(EXP_LEN -1) -1  + FRAC_LEN -1 = -126
        } else if y < -128 {
            return x * f32::from_bits(1u32 << (y + 149));
            // (2^EXP_LEN -1) = -128
        } else if y < 128 {
            return x * f32::from_bits(((y + 127) as u32) << 23);
        } else {
            x *= hexf32!("0x1.0p127");
            y -= 127;
        }
    }
}

impl TwoFloat {
    /// Returns `e^(self)`, (the exponential function).
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.exp();
    /// let e2 = twofloat::consts::E * twofloat::consts::E;
    ///
    /// assert!((b - e2).abs() / e2 < 1e-16);
    /// ```
    pub fn exp(self) -> Self {
        if self.hi <= EXP_LOWER_LIMIT {
            Self::from(0.0)
        } else if self.hi >= EXP_UPPER_LIMIT {
            Self {
                hi: f32::INFINITY,
                lo: 0.0,
            }
        } else if self.hi == 0.0 {
            Self::from(1.0)
        } else {
            // reduce value to range |r| <= ln(2)/2
            // where self = k*ln(2) + r

            let k = mathfn::trunc((FRAC_1_LN_2 * self).hi + 0.5);
            let r = self - LN_2 * k;

            // Now approximate the function
            //
            // R(r^2) = r*(exp(r)+1)/(exp(r)-1) = 2 + P1*r^2 + P2*r^4 + ...
            //
            // using a polynomial obtained by the Remez algorithm on the
            // interval [0, ln(2)/2], then:
            //
            // exp(r) = 1 + 2*r/(R-r) = 1 + r + (r*R1) / (2-R1)
            //
            // where R1 = r - (P1*r^2 + P2*r^4 + ...)

            let rr = r * r;
            let r1 = r - rr * polynomial!(rr, EXP_COEFFS);

            let exp_r = 1.0 - ((r * r1) / (r1 - 2.0) - r);

            // then scale back

            if k == 0.0 {
                exp_r
            } else {
                Self {
                    hi: mul_pow2(exp_r.hi, k as i32),
                    lo: mul_pow2(exp_r.lo, k as i32),
                }
            }
        }
    }

    /// Returns `e^(self) - 1` in a way that provides additional accuracy
    /// when the value is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.05);
    /// let b = a.exp_m1();
    /// let c = 0.05f64.exp_m1();
    ///
    /// assert!((b - c).abs() < 1e-16);
    /// ```
    pub fn exp_m1(self) -> Self {
        if self < -LN_2 || self > LN_FRAC_3_2 {
            self.exp() - 1.0
        } else {
            let r = polynomial!(self, 1.0, EXP_M1_COEFFS);
            self * r
        }
    }

    /// Returns `2^(self)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.5).exp2();
    /// let b = TwoFloat::from(2).sqrt();
    ///
    /// assert!((a - b).abs() < 1e-15);
    /// ```
    pub fn exp2(self) -> Self {
        if self < -1074.0 {
            Self::from(0.0)
        } else if self >= 1023.0 {
            Self {
                hi: f32::INFINITY,
                lo: f32::INFINITY,
            }
        } else {
            let k = mathfn::round(self.hi);
            let r = self - k;
            let r1 = polynomial!(r, 1.0, EXP2_COEFFS);
            if k == 0.0 {
                r1
            } else {
                Self {
                    hi: mul_pow2(r1.hi, k as i32),
                    lo: mul_pow2(r1.lo, k as i32),
                }
            }
        }
    }

    /// Returns the natural logarithm of the value.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp` function, so
    /// may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Example
    ///
    /// ```
    /// let a = twofloat::consts::E.ln();
    /// assert!((a - 1.0).abs() < 1e-11);
    /// ```
    pub fn ln(self) -> Self {
        if self == 1.0 {
            Self::from(0.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(mathfn::ln(self.hi));
            x += self * (-x).exp() - 1.0;
            x + self * (-x).exp() - 1.0
        }
    }

    /// Returns the natural logarithm of `1 + self`.
    ///
    /// Uses Newton–Raphson iteration which depends on the `expm1` function,
    /// so may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Example
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.1);
    /// let b = a.ln_1p();
    /// let c = 0.1f64.ln_1p();
    /// assert!((b - c).abs() < 1e-10);
    /// ```
    pub fn ln_1p(self) -> Self {
        if self == 0.0 {
            Self::from(0.0)
        } else if self <= -1.0 {
            Self::NAN
        } else {
            let mut x = Self::from(mathfn::ln_1p(self.hi));
            let mut e = x.exp_m1();
            x -= (e - self) / (e + 1.0);
            e = x.exp_m1();
            x - (e - self) / (e + 1.0)
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// This is a convenience method that computes `self.ln() / base.ln()`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// let a = TwoFloat::from(81.0);
    /// let b = TwoFloat::from(3.0);
    /// let c = TwoFloat::log(a, b);
    ///
    /// assert!((c - 4.0).abs() < 1e-12);
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp2` function,
    /// so may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(64.0).log2();
    ///
    /// assert!((a - 6.0).abs() < 1e-12, "{}", a);
    /// ```
    pub fn log2(self) -> Self {
        if self == 1.0 {
            Self::from(1.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(mathfn::log2(self.hi));
            x += (self * (-x).exp2() - 1.0) * FRAC_1_LN_2;
            x + (self * (-x).exp2() - 1.0) * FRAC_1_LN_2
        }
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// This is a convenience method that computes `self.ln() / LN_10`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(100.0).log10();
    ///
    /// assert!((a - 2.0).abs() < 1e-12);
    /// ```
    pub fn log10(self) -> Self {
        self.ln() / LN_10
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoFloat;

    #[test]
    fn exp_test() {
        assert_eq!(
            TwoFloat::from(-1000.0).exp(),
            0.0,
            "Large negative exponent produced non-zero value"
        );
        assert!(
            !TwoFloat::from(1000.0).exp().is_valid(),
            "Large positive exponent produced valid value"
        );
        assert_eq!(
            TwoFloat::from(0.0).exp(),
            TwoFloat::from(1.0),
            "exp(0) did not return 1"
        );
    }

    #[test]
    fn ln_test() {
        assert!(
            !TwoFloat::from(0.0).ln().is_valid(),
            "ln(0) produced valid result"
        );
        assert!(
            !TwoFloat::from(-5.0).ln().is_valid(),
            "ln(negative) produced valid result"
        );
        assert_eq!(
            TwoFloat::from(1.0).ln(),
            TwoFloat::from(0.0),
            "ln(1) did not return 0"
        );
    }
}
