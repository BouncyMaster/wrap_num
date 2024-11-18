#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Rem, RemAssign};
use std::hash::Hash;
use num::{Unsigned, NumCast, ToPrimitive};

pub trait UnsignedUnified: Unsigned + NumCast + PartialOrd + Copy {}
impl<T> UnsignedUnified for T where T: Unsigned + NumCast + PartialOrd + Copy {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    pub fn get_value(self) -> T {
        self.value
    }
}

impl<T: UnsignedUnified> ToPrimitive for WrapNum<T> {
    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
}

macro_rules! impl_ops {
    ($trait_name:ident, $trait_fn:ident, $as_trait_name:ident, $as_trait_fn:ident) => {
        impl<T: UnsignedUnified, U: ToPrimitive> $trait_name<U> for WrapNum<T> {
            type Output = Self;

            fn $trait_fn(self, rhs: U) -> Self {
                let result = (self.value).$trait_fn(NumCast::from(rhs).unwrap()) % self.wrap;

                Self {
                    value: result,
                    wrap: self.wrap,
                }
            }
        }

        impl<T: UnsignedUnified, U: ToPrimitive> $as_trait_name<U> for WrapNum<T> {
            fn $as_trait_fn(&mut self, rhs: U) {
                self.value = (self.value).$trait_fn(NumCast::from(rhs).unwrap()) % self.wrap;
            }
        }
    };
}

macro_rules! impl_ops_nowrap {
    ($trait_name:ident, $trait_fn:ident, $as_trait_name:ident, $as_trait_fn:ident) => {
        impl<T: UnsignedUnified, U: ToPrimitive> $trait_name<U> for WrapNum<T> {
            type Output = Self;

            fn $trait_fn(self, rhs: U) -> Self {
                let result = (self.value).$trait_fn(NumCast::from(rhs).unwrap());

                Self {
                    value: result,
                    wrap: self.wrap,
                }
            }
        }

        impl<T: UnsignedUnified, U: ToPrimitive> $as_trait_name<U> for WrapNum<T> {
            fn $as_trait_fn(&mut self, rhs: U) {
                self.value = (self.value).$trait_fn(NumCast::from(rhs).unwrap());
            }
        }
    };
}

impl_ops!(Add, add, AddAssign, add_assign);
impl_ops!(Mul, mul, MulAssign, mul_assign);
impl_ops_nowrap!(Sub, sub, SubAssign, sub_assign);
impl_ops_nowrap!(Rem, rem, RemAssign, rem_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_wrapnum_nowrap() {
        let num1 = WrapNum::new(2u32, 6u32);
        let num2 = WrapNum::new(2u32, 5u32);

        let num3 = num1 + num2;

        assert_eq!(num3.get_value(), 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn add_wrapnum_wrap() {
        let num1 = WrapNum::new(3u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        let num3 = num1 + num2;

        assert_eq!(num3.get_value(), 1);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn add_u32_wrap() {
        let num1 = WrapNum::new(3u32, 6u32);
        let num2 = 7u32;

        let num3 = num1 + num2;

        assert_eq!(num3.get_value(), 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn sub_u32() {
        let num1 = WrapNum::new(5u32, 6u32);
        let num2 = 2u32;

        let num3 = num1 - num2;

        assert_eq!(num3.get_value(), 3);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    #[should_panic]
    fn sub_u32_overflow() {
        let num1 = WrapNum::new(5u32, 7u32);
        let num2 = 6u32;

        let _ = num1 - num2;
    }

    #[test]
    fn add_assign_wrapnum_wrap() {
        let mut num1 = WrapNum::new(3u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        num1 += num2;

        assert_eq!(num1.get_value(), 1);
        assert_eq!(num1.wrap, 6);
    }

    #[test]
    fn mul_wrapnum_wrap() {
        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        let num3 = num1 * num2;

        assert_eq!(num3.get_value(), 4);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn mul_assign_wrapnum_wrap() {
        let mut num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        num1 *= num2;

        assert_eq!(num1.get_value(), 4);
        assert_eq!(num1.wrap, 6);
    }

    #[test]
    fn ops_different_types() {
        let mut num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u16, 5u16);

        num1 *= num2;

        assert_eq!(num1.get_value(), 4);
        assert_eq!(num1.wrap, 6);
    }

    #[test]
    fn rem_u32() {
        let num1 = WrapNum::new(5u32, 6u32);
        let num2 = 2u32;

        let num3 = num1 % num2;

        assert_eq!(num3.get_value(), 1);
        assert_eq!(num3.wrap, 6);
    }

    #[test]
    fn rem_assign_u32() {
        let mut num1 = WrapNum::new(9u32, 10u32);
        let num2 = 5u32;

        num1 %= num2;

        assert_eq!(num1.get_value(), 4);
        assert_eq!(num1.wrap, 10);
    }

    #[test]
    fn hash_eq() {
        use std::hash::{Hasher, DefaultHasher};

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 6u32);

        assert_eq!(calculate_hash(&num1), calculate_hash(&num2));
    }

    #[test]
    fn hash_ne() {
        use std::hash::{Hasher, DefaultHasher};

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        assert_ne!(calculate_hash(&num1), calculate_hash(&num2));
    }

    #[test]
    fn eq() {
        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 6u32);

        assert_eq!(num1 == num2, true);
    }

    #[test]
    fn ne() {
        let num1 = WrapNum::new(4u32, 6u32);
        let num2 = WrapNum::new(4u32, 5u32);

        assert_eq!(num1 == num2, false);
    }
}
