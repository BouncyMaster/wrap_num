#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use std::ops::{Add, AddAssign, Mul, MulAssign};
use num::{Unsigned, NumCast};

pub trait UnsignedUnified: Unsigned + NumCast + PartialOrd + Copy {}
impl<T> UnsignedUnified for T where T: Unsigned + NumCast + PartialOrd + Copy {}

#[derive(Debug, Clone, Copy)]
pub struct WrapNum<T: UnsignedUnified> {
    value: T,
    wrap: T,
}

// Unsigned type that wraps back to 0 when value exceeds `wrap`.
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

macro_rules! impl_ops {
    ($trait_name:ident, $trait_fn:ident, $as_trait_name:ident, $as_trait_fn:ident) => {
        impl<T: UnsignedUnified, U: UnsignedUnified> $trait_name<U> for WrapNum<T> {
            type Output = Self;

            fn $trait_fn(self, rhs: U) -> Self {
                let result = (self.value).$trait_fn(NumCast::from(rhs).unwrap()) % self.wrap;

                Self {
                    value: result,
                    wrap: self.wrap,
                }
            }
        }

        impl<T: UnsignedUnified, U: UnsignedUnified> $trait_name<WrapNum<U>> for WrapNum<T> {
            type Output = Self;

            fn $trait_fn(self, rhs: WrapNum<U>) -> Self {
                let result = (self.value).$trait_fn(NumCast::from(rhs.value).unwrap()) % self.wrap;

                Self {
                    value: result,
                    wrap: self.wrap,
                }
            }
        }

        impl<T: UnsignedUnified, U: UnsignedUnified> $as_trait_name<U> for WrapNum<T> {
            fn $as_trait_fn(&mut self, rhs: U) {
                self.value = (self.value).$trait_fn(NumCast::from(rhs).unwrap()) % self.wrap;
            }
        }

        impl<T: UnsignedUnified, U: UnsignedUnified> $as_trait_name<WrapNum<U>> for WrapNum<T> {
            fn $as_trait_fn(&mut self, rhs: WrapNum<U>) {
                self.value = (self.value).$trait_fn(NumCast::from(rhs.value).unwrap()) % self.wrap;
            }
        }
    };
}

impl_ops!(Add, add, AddAssign, add_assign);
impl_ops!(Mul, mul, MulAssign, mul_assign);

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

    #[test]
    fn ops_different_types() {
        let mut num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u16, 5u16);

        num1 *= num2;

        assert_eq!(num1.value, 4);
        assert_eq!(num1.wrap, 6);
    }
}
