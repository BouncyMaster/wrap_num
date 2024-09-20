use std::ops::{Add, AddAssign, Mul, MulAssign};
use num::{Unsigned, cast, NumCast};

pub trait UnsignedUnified: Unsigned + NumCast + PartialOrd + Copy {}
impl<T> UnsignedUnified for T where T: Unsigned + NumCast + PartialOrd + Copy {}

#[derive(Debug, Clone, Copy)]
pub struct WrapNum<T: UnsignedUnified> {
    value: T,
    wrap: T,
}

// Unsigned type that wraps to 0 when value exceeds `wrap`.
// When operating with multiple WrapNums, the wrap value of the former is taken.
impl<T: UnsignedUnified> WrapNum<T> {
    pub fn new(value: T, wrap: T) -> WrapNum<T> {
        assert!(value < wrap);

        WrapNum {
            value,
            wrap,
        }
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> Add<WrapNum<U>> for WrapNum<T> {
    type Output = Self;

    fn add(self, rhs: WrapNum<U>) -> Self {
        Self {
            value: (self.value + cast(rhs.value).unwrap()) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> Add<U> for WrapNum<T> {
    type Output = Self;

    fn add(self, rhs: U) -> Self {
        Self {
            value: (self.value + cast(rhs).unwrap()) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> AddAssign<WrapNum<U>> for WrapNum<T> {
    fn add_assign(&mut self, rhs: WrapNum<U>) {
        self.value = (self.value + cast(rhs.value).unwrap()) % self.wrap;
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> AddAssign<U> for WrapNum<T> {
    fn add_assign(&mut self, rhs: U) {
        self.value = (self.value + cast(rhs).unwrap()) % self.wrap;
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> Mul<WrapNum<U>> for WrapNum<T> {
    type Output = Self;

    fn mul(self, rhs: WrapNum<U>) -> Self {
        Self {
            value: (self.value * cast(rhs.value).unwrap()) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> Mul<U> for WrapNum<T> {
    type Output = Self;

    fn mul(self, rhs: U) -> Self {
        Self {
            value: (self.value * cast(rhs).unwrap()) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> MulAssign<WrapNum<U>> for WrapNum<T> {
    fn mul_assign(&mut self, rhs: WrapNum<U>) {
        self.value = (self.value * cast(rhs.value).unwrap()) % self.wrap;
    }
}

impl<T: UnsignedUnified, U: UnsignedUnified> MulAssign<U> for WrapNum<T> {
    fn mul_assign(&mut self, rhs: U) {
        self.value = (self.value * cast(rhs).unwrap()) % self.wrap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_wrapnum_nowrap() {
        let num1 = WrapNum::new(2u32, 6u32);
        let num2 = WrapNum::new(2u32, 5u32);

        let num3 = num1 + num2;

        assert_eq!(num3.value, 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn add_wrapnum_wrap() {
        let num1 = WrapNum::new(3u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        let num3 = num1 + num2;

        assert_eq!(num3.value, 1);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn add_u64_wrap() {
        let num1 = WrapNum::new(3u32, 6u32);
        let num2 = 7u32;

        let num3 = num1 + num2;

        assert_eq!(num3.value, 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn add_assign_wrapnum_wrap() {
        let mut num1 = WrapNum::new(3u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        num1 += num2;

        assert_eq!(num1.value, 1);
        assert_eq!(num1.wrap, 6);
    }

    #[test]
    fn mul_wrapnum_wrap() {
        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        let num3 = num1 * num2;

        assert_eq!(num3.value, 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn mul_assign_wrapnum_wrap() {
        let mut num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        num1 *= num2;

        assert_eq!(num1.value, 4);
        assert_eq!(num1.wrap, 6);
    }
}
