/// Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.
pub trait Inv {
    /// The result after applying the operator.
    type Output;

    /// Returns the multiplicative inverse of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64::INFINITY;
    /// use num_traits::Inv;
    ///
    /// assert_eq!(7.0.inv() * 7.0, 1.0);
    /// assert_eq!((-0.0).inv(), -INFINITY);
    /// ```
    fn inv(self) -> Self::Output;
}

#[cfg(has_f16)]
impl Inv for f16 {
    type Output = f16;
    #[inline]
    fn inv(self) -> f16 {
        1.0 / self
    }
}
impl Inv for f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 {
        1.0 / self
    }
}
impl Inv for f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 {
        1.0 / self
    }
}
#[cfg(has_f128)]
impl Inv for f128 {
    type Output = f128;
    #[inline]
    fn inv(self) -> f128 {
        1.0 / self
    }
}
#[cfg(has_f16)]
impl<'a> Inv for &'a f16 {
    type Output = f16;
    #[inline]
    fn inv(self) -> f16 {
        1.0 / *self
    }
}
impl<'a> Inv for &'a f32 {
    type Output = f32;
    #[inline]
    fn inv(self) -> f32 {
        1.0 / *self
    }
}
impl<'a> Inv for &'a f64 {
    type Output = f64;
    #[inline]
    fn inv(self) -> f64 {
        1.0 / *self
    }
}
#[cfg(has_f128)]
impl<'a> Inv for &'a f128 {
    type Output = f128;
    #[inline]
    fn inv(self) -> f128 {
        1.0 / *self
    }
}
