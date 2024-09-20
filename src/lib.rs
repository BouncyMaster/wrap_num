use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct WrapNum {
    value: u64,
    wrap: u64,
}

// uint type that wraps to 0 when value exceeds `wrap`.
// When operating with multiple WrapNums, the wrap value of the former is taken.
impl WrapNum {
    pub fn new(value: u64, wrap: u64) -> WrapNum {
        assert!(value < wrap);

        WrapNum {
            value,
            wrap,
        }
    }
}

impl Add<WrapNum> for WrapNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value + rhs.value) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl Add<u64> for WrapNum {
    type Output = Self;

    fn add(self, rhs: u64) -> Self {
        Self {
            value: (self.value + rhs) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl AddAssign<WrapNum> for WrapNum {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % self.wrap;
    }
}

impl AddAssign<u64> for WrapNum {
    fn add_assign(&mut self, rhs: u64) {
        self.value = (self.value + rhs) % self.wrap;
    }
}

impl Mul<WrapNum> for WrapNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            value: (self.value * rhs.value) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl Mul<u64> for WrapNum {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self {
            value: (self.value * rhs) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl MulAssign<WrapNum> for WrapNum {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value * rhs.value) % self.wrap;
    }
}

impl MulAssign<u64> for WrapNum {
    fn mul_assign(&mut self, rhs: u64) {
        self.value = (self.value * rhs) % self.wrap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_wrapnum() {
        let num1 = WrapNum::new(2, 6);
        let num2 = WrapNum::new(2, 5);

        let num3 = num1 + num2;

        assert_eq!(num3.value, 4);
        assert_eq!(num3.wrap, 6);
    }
}
