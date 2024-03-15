/// A wrapper struct for mathematical operations on `f64`s.
///
/// It uses "libm" if it's enabled, which is required for "no_std".
/// Fallbacks to "std" otherwise.
#[cfg(not(feature = "std"))]
pub(crate) mod mathfn {
    #[inline(always)]
    pub fn abs(x: f32) -> f32 {
        libm::fabsf(x)
    }
    #[inline(always)]
    pub fn sqrt(x: f32) -> f32 {
        libm::sqrtf(x)
    }
    #[inline(always)]
    pub fn cbrt(x: f32) -> f32 {
        libm::cbrtf(x)
    }
    #[inline(always)]
    pub fn rsqrt(x: f32) -> f32 {
        1.0_f32/ libm::sqrtf(x)
    }
    #[inline(always)]
    pub fn exp2(x: f32) -> f32 {
        libm::exp2f(x)
    }
    #[inline(always)]
    pub fn ln(x: f32) -> f32 {
        libm::logf(x)
    }
    #[inline(always)]
    pub fn log2(x: f32) -> f32 {
        libm::log2f(x)
    }
    #[inline(always)]
    pub fn ln_1p(x: f32) -> f32 {
        libm::log1pf(x)
    }
    #[inline(always)]
    pub fn ceil(x: f32) -> f32 {
        libm::ceilf(x)
    }
    #[inline(always)]
    pub fn floor(x: f32) -> f32 {
        libm::floorf(x)
    }
    #[inline(always)]
    pub fn trunc(x: f32) -> f32 {
        libm::truncf(x)
    }
    #[inline(always)]
    pub fn round(x: f32) -> f32 {
        libm::roundf(x)
    }
    #[inline(always)]
    pub fn fma(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }
    #[inline(always)]
    pub fn fract(x: f32) -> f32 {
        libm::modff(x).0
    }
    #[inline(always)]
    pub fn signum(x: f32) -> f32 {
        libm::copysignf(1., x)
    }
}

#[cfg(feature = "std")]
pub(crate) mod mathfn {
    #[inline(always)]
    pub fn abs(x: f32) -> f32 {
        x.abs()
    }
    #[inline(always)]
    pub fn sqrt(x: f32) -> f32 {
        x.sqrt()
    }
    #[inline(always)]
    pub fn cbrt(x: f32) -> f32 {
        x.cbrt()
    }
    #[inline(always)]
    pub fn rsqrt(x: f32) -> f32 {
        x.sqrt().recip()
    }
    #[inline(always)]
    pub fn exp2(x: f32) -> f32 {
        x.exp2()
    }
    #[inline(always)]
    pub fn ln(x: f32) -> f32 {
        x.ln()
    }
    #[inline(always)]
    pub fn log2(x: f32) -> f32 {
        x.log2()
    }
    #[inline(always)]
    pub fn ln_1p(x: f32) -> f32 {
        x.ln_1p()
    }
    #[inline(always)]
    pub fn ceil(x: f32) -> f32 {
        x.ceil()
    }
    #[inline(always)]
    pub fn floor(x: f32) -> f32 {
        x.floor()
    }
    #[inline(always)]
    pub fn trunc(x: f32) -> f32 {
        x.trunc()
    }
    #[inline(always)]
    pub fn round(x: f32) -> f32 {
        x.round()
    }
    #[cfg(not(all(windows, target_env = "gnu")))]
    #[inline(always)]
    pub fn fma(a: f32, b: f32, c: f32) -> f32 {
        a.mul_add(b, c)
    }
    // The built-in FMA on MinGW is inaccurate, so always use the libm version
    #[cfg(all(windows, target_env = "gnu"))]
    #[inline(always)]
    pub fn fma(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }
    #[inline(always)]
    pub fn fract(x: f32) -> f32 {
        x.fract()
    }
    #[inline(always)]
    pub fn signum(x: f32) -> f32 {
        x.signum()
    }
}
