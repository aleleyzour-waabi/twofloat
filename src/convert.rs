use core::convert::{From, TryFrom};

use crate::{base::no_overlap, TwoFloat, TwoFloatError};

macro_rules! from_conversion {
    (|$source_i:ident : TwoFloat| -> $dest:tt $code:block) => {
        impl From<TwoFloat> for $dest {
            fn from($source_i: TwoFloat) -> Self $code
        }

        impl<'a> From<&'a TwoFloat> for $dest {
            fn from($source_i: &'a TwoFloat) -> Self $code
        }
    };
    (|$source_i:ident: TwoFloat| -> Result<$dest:tt, $err:tt> $code:block) => {
        impl TryFrom<TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: TwoFloat) -> Result<Self, Self::Error> $code
        }

        impl<'a> TryFrom<&'a TwoFloat> for $dest {
            type Error = $err;

            fn try_from($source_i: &'a TwoFloat) -> Result<Self, Self::Error> $code
        }
    };
}

from_conversion!(|value: TwoFloat| -> (f32, f32) { (value.hi, value.lo) });

impl TryFrom<(f32, f32)> for TwoFloat {
    type Error = TwoFloatError;

    fn try_from(value: (f32, f32)) -> Result<Self, Self::Error> {
        if no_overlap(value.0, value.1) {
            Ok(Self {
                hi: value.0,
                lo: value.1,
            })
        } else {
            Err(Self::Error::ConversionError {})
        }
    }
}

from_conversion!(|value: TwoFloat| -> [f32; 2] { [value.hi, value.lo] });

impl TryFrom<[f32; 2]> for TwoFloat {
    type Error = TwoFloatError;

    fn try_from(value: [f32; 2]) -> Result<Self, Self::Error> {
        if no_overlap(value[0], value[1]) {
            Ok(Self {
                hi: value[0],
                lo: value[1],
            })
        } else {
            Err(Self::Error::ConversionError {})
        }
    }
}

impl From<f32> for TwoFloat {
    fn from(value: f32) -> Self {
        Self {
            hi: value,
            lo: 0.0,
        }
    }
}

impl From<f64> for TwoFloat {
    fn from(value: f64) -> Self {
        Self {
            hi: value as f32,
            lo: (value - ((value as f32) as f64)) as f32
        }
    }
}

impl Into<f64> for TwoFloat {
    fn into(self) -> f64 {
        (self.hi as f64) + (self.lo as f64)
    }
}

impl Into<f32> for TwoFloat {
    fn into(self) -> f32 {
        self.hi
    }
}

macro_rules! int_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                Self {
                    hi: value as f32,
                    lo: 0.0,
                }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, TwoFloatError> {
            const LOWER_BOUND: f32 = $type::MIN as f32;
            const UPPER_BOUND: f32 = $type::MAX as f32;
            let truncated = value.trunc();
            if !(LOWER_BOUND..=UPPER_BOUND).contains(&truncated) {
                Err(Self::Error::ConversionError {})
            } else {
                Ok(truncated.hi() as $type)
            }
        });
    };
}

int_convert!(i32);
int_convert!(i16);
int_convert!(i8);
int_convert!(u32);
int_convert!(u16);
int_convert!(u8);

macro_rules! bigint_convert {
    ($type:tt) => {
        impl From<$type> for TwoFloat {
            fn from(value: $type) -> Self {
                let a = value as f32;
                let b = if a == $type::MAX as f32 {
                    -((($type::MAX - value) + 1) as f32)
                } else if value >= a as $type {
                    (value - a as $type) as f32
                } else {
                    -((a as $type - value) as f32)
                };

                Self { hi: a, lo: b }
            }
        }

        from_conversion!(|value: TwoFloat| -> Result<$type, TwoFloatError> {
            const LOWER_BOUND: TwoFloat = TwoFloat {
                hi: $type::MIN as f32,
                lo: 0.0,
            };

            const UPPER_BOUND: TwoFloat = TwoFloat {
                hi: $type::MAX as f32,
                lo: -1.0,
            };

            let truncated = value.trunc();
            if !(LOWER_BOUND..=UPPER_BOUND).contains(&truncated) {
                Err(Self::Error::ConversionError {})
            } else if truncated.hi() == UPPER_BOUND.hi() {
                Ok($type::MAX - (-truncated.lo() as $type) + 1)
            } else if truncated.lo() >= 0.0 {
                Ok(truncated.hi() as $type + truncated.lo() as $type)
            } else {
                Ok(truncated.hi() as $type - (-truncated.lo()) as $type)
            }
        });
    };
}

bigint_convert!(i128);
bigint_convert!(i64);
bigint_convert!(u128);
bigint_convert!(u64);
