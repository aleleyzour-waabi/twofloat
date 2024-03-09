use hexf::hexf32;

use crate::TwoFloat;

/// Euler's number (e)
pub const E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.5bf0a8p1"),
    lo: hexf32!("0x1.4d57eep-53"),
};

/// 1/π
pub const FRAC_1_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.45f306p-2"),
    lo: hexf32!("-0x1.dab8aap-27"),
};

/// 2/π
pub const FRAC_2_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p+0"),
    lo: hexf32!("-0x1.691296p-56"),
};

/// 2/sqrt(π)
pub const FRAC_2_SQRT_PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.20dd76p0"),
    lo: hexf32!("0x1.1ae3aap-56"),
};

/// 1/sqrt(2)
pub const FRAC_1_SQRT_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.6a09e6p-1"),
    lo: hexf32!("-0x1.bdd34p-55"),
};

/// π/2
pub const FRAC_PI_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p0"),
    lo: hexf32!("0x1.5ee4b2p-22"),
};

/// π/3
pub const FRAC_PI_3: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.0c1524p0"),
    lo: hexf32!("-0x1.ee692p-54"),
};

/// π/4
pub const FRAC_PI_4: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p-1"),
    lo: hexf32!("-0x1.3a69dcp-25"),
};

/// π/6
pub const FRAC_PI_6: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.0c1524p-1"),
    lo: hexf32!("-0x1.ee692p-55"),
};

/// π/8
pub const FRAC_PI_8: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p-2"),
    lo: hexf32!("0x1.1a6264p-56"),
};

/// ln(2)
pub const LN_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.62e430p-1"),
    lo: hexf32!("0x1.abc9e4p-56"),
};

/// ln(10)
pub const LN_10: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.26bb1cp1"),
    lo: hexf32!("-0x1.f48ad4p-53"),
};

/// log<sub>2</sub>(e)
pub const LOG2_E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.715476p0"),
    lo: hexf32!("0x1.777d10p-56"),
};

/// log<sub>10</sub>(e)
pub const LOG10_E: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.bcb7b2p-2"),
    lo: hexf32!("0x1.95355cp-57"),
};

/// log<sub>10</sub>(2)
pub const LOG10_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.344136p-2"),
    lo: hexf32!("-0x1.9dc1ep-59"),
};

/// log<sub>2</sub>(10)
pub const LOG2_10: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.a934f0p1"),
    lo: hexf32!("0x1.7f2496p-53"),
};

/// Archimedes' constant (π)
pub const PI: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p1"),
    lo: hexf32!("0x1.1a6264p-53"),
};

/// sqrt(2)
pub const SQRT_2: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.6a09e6p0"),
    lo: hexf32!("-0x1.bdd342p-54"),
};

/// The full circle constant (τ)
pub const TAU: TwoFloat = TwoFloat {
    hi: hexf32!("0x1.921fb6p2"),
    lo: hexf32!("0x1.1a6264p-52"),
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
                    assert_eq!($name.hi, core::f32::consts::$name);
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
                    assert_eq!($name.hi, core::f32::consts::$name);
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
