/// A wrapper struct for mathematical operations on `f64`s.
///
/// It uses "libm" if it's enabled, which is required for "no_std".
/// Fallbacks to "std" otherwise.
#[cfg(not(feature = "std"))]
pub(crate) mod mathfn {
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        libm::fabs(x)
    }
    #[inline(always)]
    pub fn fabsf(x: f32) -> f32 {
        libm::fabsf(x)
    }
    #[inline(always)]
    pub fn ceil(x: f64) -> f64 {
        libm::ceil(x)
    }
    #[inline(always)]
    pub fn ceilf(x: f32) -> f32 {
        libm::ceilf(x)
    }
    #[inline(always)]
    pub fn exp2(x: f64) -> f64 {
        libm::exp2(x)
    }
    #[inline(always)]
    pub fn exp2f(x: f32) -> f32 {
        libm::exp2f(x)
    }
    #[inline(always)]
    pub fn floor(x: f64) -> f64 {
        libm::floor(x)
    }
    #[inline(always)]
    pub fn floorf(x: f32) -> f32 {
        libm::floorf(x)
    }
    #[inline(always)]
    pub fn fma(a: f64, b: f64, c: f64) -> f64 {
        libm::fma(a, b, c)
    }
    #[inline(always)]
    pub fn fmaf(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }
    #[inline(always)]
    pub fn fract(x: f64) -> f64 {
        libm::modf(x).0
    }
    #[inline(always)]
    pub fn fractf(x: f32) -> f32 {
        libm::modff(x).0
    }
    #[inline(always)]
    pub fn round(x: f64) -> f64 {
        libm::round(x)
    }
    #[inline(always)]
    pub fn roundf(x: f32) -> f32 {
        libm::roundf(x)
    }
    #[inline(always)]
    pub fn signum(x: f64) -> f64 {
        libm::copysign(1., x)
    }
    #[inline(always)]
    pub fn signumf(x: f32) -> f32 {
        libm::copysignf(1., x)
    }
    #[inline(always)]
    pub fn trunc(x: f64) -> f64 {
        libm::trunc(x)
    }
    #[inline(always)]
    pub fn truncf(x: f32) -> f32 {
        libm::truncf(x)
    }
}

#[cfg(feature = "std")]
pub(crate) mod mathfn {
    #[inline(always)]
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }
    #[inline(always)]
    pub fn fabsf(x: f32) -> f32 {
        x.abs()
    }
    #[inline(always)]
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }
    #[inline(always)]
    pub fn ceilf(x: f32) -> f32 {
        x.ceil()
    }
    #[inline(always)]
    pub fn exp2(x: f64) -> f64 {
        x.exp2()
    }
    #[inline(always)]
    pub fn exp2f(x: f32) -> f32 {
        x.exp2()
    }
    #[inline(always)]
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }
    #[inline(always)]
    pub fn floorf(x: f32) -> f32 {
        x.floor()
    }
    #[cfg(not(all(windows, target_env = "gnu")))]
    #[inline(always)]
    pub fn fma(a: f64, b: f64, c: f64) -> f64 {
        a.mul_add(b, c)
    }
    #[cfg(not(all(windows, target_env = "gnu")))]
    #[inline(always)]
    pub fn fmaf(a: f32, b: f32, c: f32) -> f32 {
        a.mul_add(b, c)
    }
    // The built-in FMA on MinGW is inaccurate, so always use the libm version
    #[cfg(all(windows, target_env = "gnu"))]
    #[inline(always)]
    pub fn fma(a: f64, b: f64, c: f64) -> f64 {
        libm::fma(a, b, c)
    }
    #[cfg(all(windows, target_env = "gnu"))]
    #[inline(always)]
    pub fn fmaf(a: f32, b: f32, c: f32) -> f32 {
        libm::fmaf(a, b, c)
    }
    #[inline(always)]
    pub fn fract(x: f64) -> f64 {
        x.fract()
    }
    #[inline(always)]
    pub fn fractf(x: f32) -> f32 {
        x.fract()
    }
    #[inline(always)]
    pub fn round(x: f64) -> f64 {
        x.round()
    }
    #[inline(always)]
    pub fn roundf(x: f32) -> f32 {
        x.round()
    }
    #[inline(always)]
    pub fn signum(x: f64) -> f64 {
        x.signum()
    }
    #[inline(always)]
    pub fn signumf(x: f32) -> f32 {
        x.signum()
    }
    #[inline(always)]
    pub fn trunc(x: f64) -> f64 {
        x.trunc()
    }
    #[inline(always)]
    pub fn truncf(x: f32) -> f32 {
        x.trunc()
    }
}
