use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct WrapNum {
    number: u64,
    wrap: u64,
}

// uint type that wraps to 0 when value exceeds `wrap`
impl WrapNum {
    pub fn new(number: u64, wrap: u64) -> WrapNum {
        assert!(number < wrap);

        WrapNum {
            number,
            wrap,
        }
    }

    pub fn get_wrap(&self) -> u64 {
        self.wrap
    }
}

impl Add<WrapNum> for WrapNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            number: (self.number + rhs.number) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl Add<u64> for WrapNum {
    type Output = Self;

    fn add(self, rhs: u64) -> Self {
        Self {
            number: (self.number + rhs) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl AddAssign<WrapNum> for WrapNum {
    fn add_assign(&mut self, rhs: Self) {
        self.number = (self.number + rhs.number) % self.wrap;
    }
}

impl AddAssign<u64> for WrapNum {
    fn add_assign(&mut self, rhs: u64) {
        self.number = (self.number + rhs) % self.wrap;
    }
}

impl Mul<WrapNum> for WrapNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            number: (self.number * rhs.number) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl Mul<u64> for WrapNum {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self {
            number: (self.number * rhs) % self.wrap,
            wrap: self.wrap,
        }
    }
}

impl MulAssign<WrapNum> for WrapNum {
    fn mul_assign(&mut self, rhs: Self) {
        self.number = (self.number * rhs.number) % self.wrap;
    }
}

impl MulAssign<u64> for WrapNum {
    fn mul_assign(&mut self, rhs: u64) {
        self.number = (self.number * rhs) % self.wrap;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_wrap() {
        let num = WrapNum::new(0, 6);
        assert_eq!(num.get_wrap(), 6);
    }
}
