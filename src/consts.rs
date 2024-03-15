use hexf::hexf32;

use crate::TwoFloat;

/// Euler's number (e)
pub const E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.5bf0a8p+1"),
    lo: hexf32!("0x1.628aeep-24"),
};

/// 1/π
pub const FRAC_1_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.45f306p-2"),
    lo: hexf32!("0x1.b9391p-27"),
};

/// 2/π
pub const FRAC_2_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.45f306p-1"),
    lo: hexf32!("0x1.b9391p-26"),
};

/// 2/sqrt(π)
pub const FRAC_2_SQRT_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.20dd76p+0"),
    lo: hexf32!("-0x1.f7ac92p-25"),
};

/// 1/sqrt(2)
pub const FRAC_1_SQRT_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.6a09e6p-1"),
    lo: hexf32!("0x1.9fcef4p-27"),
};

/// π/2
pub const FRAC_PI_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p+0"),
    lo: hexf32!("-0x1.777a5cp-25"),
};

/// π/3
pub const FRAC_PI_3: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.0c1524p+0"),
    lo: hexf32!("-0x1.f4a326p-26"),
};

/// π/4
pub const FRAC_PI_4: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p-1"),
    lo: hexf32!("-0x1.777a5cp-26"),
};

/// π/6
pub const FRAC_PI_6: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.0c1524p-1"),
    lo: hexf32!("-0x1.f4a326p-27"),
};

/// π/8
pub const FRAC_PI_8: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p-2"),
    lo: hexf32!("-0x1.777a5cp-27"),
};

/// ln(2)
pub const LN_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.62e43p-1"),
    lo: hexf32!("-0x1.05c61p-29"),
};

/// ln(10)
pub const LN_10: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.26bb1cp+1"),
    lo: hexf32!("-0x1.12aabap-25"),
};

/// log<sub>2</sub>(e)
pub const LOG2_E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.715476p+0"),
    lo: hexf32!("0x1.4ae0cp-26"),
};

/// log<sub>10</sub>(e)
pub const LOG10_E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.bcb7b2p-2"),
    lo: hexf32!("-0x1.5b235ep-27"),
};

/// log<sub>10</sub>(2)
pub const LOG10_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.344136p-2"),
    lo: hexf32!("-0x1.ec10cp-27"),
};

/// log<sub>2</sub>(10)
pub const LOG2_10: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.a934fp+1"),
    lo: hexf32!("0x1.2f346ep-24"),
};

/// Archimedes' constant (π)
pub const PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p+1"),
    lo: hexf32!("-0x1.777a5cp-24"),
};

/// sqrt(2)
pub const SQRT_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.6a09e6p+0"),
    lo: hexf32!("0x1.9fcef4p-26"),
};

/// The full circle constant (τ)
pub const TAU: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p+2"),
    lo: hexf32!("-0x1.777a5cp-23"),
};

#[cfg(test)]
mod tests {
    use super::{
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4,
        FRAC_PI_6, FRAC_PI_8, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, SQRT_2, TAU,
    };

    macro_rules! const_check {
        ($name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        ($name:ident, $($names:ident),+) => {
            const_check! { $name }
            const_check! { $($names),+ }
        };
        ($($names:ident,)+) => {
            const_check! { $($names),+ }
        };
        (#[cfg($feature:tt)] $name:ident) => {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod $name {
                use super::$name;

                #[test]
                fn valid_test() {
                    assert!($name.is_valid());
                }

                #[cfg($feature)]
                #[test]
                fn value_test() {
                    assert_eq!($name.hi, core::f64::consts::$name);
                }
            }
        };
        (#[cfg($feature:tt)] $name:ident, $($names:ident),+) => {
            const_check! { #[cfg($feature)] $name }
            const_check! { #[cfg($feature)] $($names),+ }
        };
        (#[cfg($feature:tt)] $($names:ident,)+) => {
            const_check! { #[cfg($feature)] $($names),+ }
        }
    }

    const_check! {
        E, FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_1_SQRT_2, FRAC_PI_2,
        FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, LN_2, LN_10, LOG2_E,
        LOG10_E, PI, SQRT_2,
    }

    const_check! {
        #[cfg(extra_log_consts)]
        LOG10_2, LOG2_10, TAU
    }
}
