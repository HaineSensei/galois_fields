use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg, Div, DivAssign};

use crate::{Field, Ring};

#[derive(Clone, Copy)]
pub struct IMod<const BASE: u64> {
    val: u128
}

impl<const BASE: u64> IMod<BASE> {
    pub const fn new(val:u128) -> Self {
        Self {
            val: (val%(BASE as u128))
        }
    }
}
impl<const BASE: u64> Add for IMod<BASE> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val + rhs.val)
    }
}

// Mul implementation
impl<const BASE: u64> Mul for IMod<BASE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.val*rhs.val)
    }
}

// Sub implementation
impl<const BASE: u64> Sub for IMod<BASE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(match self.val.cmp(&rhs.val) {
            std::cmp::Ordering::Less => {
                self.val + ((BASE as u128) - rhs.val)
            },
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.val - rhs.val,
        })
    }
}

// Neg implementation
impl<const BASE: u64> Neg for IMod<BASE> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(BASE as u128 - self.val)
    }
}

// AddAssign implementation
impl<const BASE: u64> AddAssign for IMod<BASE> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// AddAssign<&Self> implementation
impl<const BASE: u64> AddAssign<&IMod<BASE>> for IMod<BASE> {
    fn add_assign(&mut self, rhs: &IMod<BASE>) {
        *self += *rhs;
    }
}

// MulAssign implementation
impl<const BASE: u64> MulAssign for IMod<BASE> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// MulAssign<&Self> implementation
impl<const BASE: u64> MulAssign<&IMod<BASE>> for IMod<BASE> {
    fn mul_assign(&mut self, rhs: &IMod<BASE>) {
        *self *= *rhs;
    }
}

// SubAssign implementation
impl<const BASE: u64> SubAssign for IMod<BASE> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

// SubAssign<&Self> implementation
impl<const BASE: u64> SubAssign<&IMod<BASE>> for IMod<BASE> {
    fn sub_assign(&mut self, rhs: &IMod<BASE>) {
        *self += -rhs;
    }
}

impl<const BASE: u64> IMod<BASE> {
    pub const fn inverse(self) -> Option<Self> {
        use crate::tools::mod_inverse;
        let val = self.val;
        let base = BASE;
        let inverse = mod_inverse(val as u64, base);
        if let Some(inverse) = inverse {
            Some(Self::new(inverse as u128))
        } else {
            None
        }
    }

}

// Div implementation
impl<const BASE: u64> Div for IMod<BASE> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse().expect("Attempted to divide by a non-unit")
    }
}

// DivAssign implementation
impl<const BASE: u64> DivAssign for IMod<BASE> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// DivAssign<&Self> implementation (for Field trait bounds)
impl<const BASE: u64> DivAssign<&IMod<BASE>> for IMod<BASE> {
    fn div_assign(&mut self, rhs: &IMod<BASE>) {
        *self /= *rhs;
    }
}

impl<const BASE: u64> PartialEq for IMod<BASE> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<const BASE: u64> Eq for IMod<BASE> {}

// Reference operations for Ring trait bounds
impl<'a, 'b, const BASE: u64> Add<&'b IMod<BASE>> for &'a IMod<BASE> {
    type Output = IMod<BASE>;

    fn add(self, rhs: &'b IMod<BASE>) -> Self::Output {
        *self + *rhs
    }
}

impl<'a, 'b, const BASE: u64> Mul<&'b IMod<BASE>> for &'a IMod<BASE> {
    type Output = IMod<BASE>;

    fn mul(self, rhs: &'b IMod<BASE>) -> Self::Output {
        *self * *rhs
    }
}

impl<'a, 'b, const BASE: u64> Sub<&'b IMod<BASE>> for &'a IMod<BASE> {
    type Output = IMod<BASE>;

    fn sub(self, rhs: &'b IMod<BASE>) -> Self::Output {
        *self - *rhs
    }
}

impl<'a, const BASE: u64> Neg for &'a IMod<BASE> {
    type Output = IMod<BASE>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<const BASE: u64> Ring for IMod<BASE> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn one() -> Self {
        Self::new(1)
    }
}

impl<'a, 'b, const BASE: u64> Div<&'b IMod<BASE>> for &'a IMod<BASE> {
    type Output = IMod<BASE>;

    fn div(self, rhs: &'b IMod<BASE>) -> Self::Output {
        *self / *rhs
    }
}

impl<const BASE: u64> Field for IMod<BASE> {
    
}
