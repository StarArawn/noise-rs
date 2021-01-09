/// Performs linear interpolation between two values.
#[cfg(not(target_os = "emscripten"))]
#[inline]
pub(crate) fn linear(a: f64, b: f64, x: f64) -> f64 {
    x.mul_add(b - a, a)
}

/// Performs linear interpolation between two values.
#[cfg(target_os = "emscripten")]
#[inline]
pub(crate) fn linear(a: f64, b: f64, x: f64) -> f64 {
    (x * (b - a)) + a
}

/// Performs cubic interpolation between two values bound between two other
/// values.
///
/// - n0 - The value before the first value.
/// - n1 - The first value.
/// - n2 - The second value.
/// - n3 - The value after the second value.
/// - alpha - The alpha value.
///
/// The alpha value should range from 0.0 to 1.0. If the alpha value is
/// 0.0, this function returns _n1_. If the alpha value is 1.0, this
/// function returns _n2_.
#[inline]
pub(crate) fn cubic(n0: f64, n1: f64, n2: f64, n3: f64, alpha: f64) -> f64 {
    let p = (n3 - n2) - (n0 - n1);
    let q = (n0 - n1) - p;
    let r = n2 - n0;
    let s = n1;
    p * alpha * alpha * alpha + q * alpha * alpha + r * alpha + s
}

/// Maps a value onto a cubic S-curve.
#[inline]
pub(crate) fn s_curve3(x: f64) -> f64 {
    x * x * (3.0 - (x * 2.0))
}

/// Maps a value onto a quintic S-curve.
#[inline]
pub(crate) fn s_curve5(x: f64) -> f64 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}
