#![feature(adt_const_params)]
// This module's core logic doesn't seem to work in the current version of Rust, but I'll leave this around in case it does in the future.
use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};
use std::marker::ConstParamTy;

pub mod polynomial;
pub mod modulo_ints;
pub mod tools;

#[derive(Clone, Copy, PartialEq, Eq, Hash, ConstParamTy)]
pub struct Indeterminate {
    name: char
}

impl Indeterminate {
    pub const fn new(name: char) -> Self {
        Indeterminate {
            name
        }
    }
}

impl Display for Indeterminate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.name)
    }
}

pub const T: Indeterminate = Indeterminate::new('T');
pub const X: Indeterminate = Indeterminate::new('X');
pub const Y: Indeterminate = Indeterminate::new('Y');
pub const Z: Indeterminate = Indeterminate::new('Z');

pub trait Ring: Add + Mul + Sub + Clone + AddAssign + MulAssign + SubAssign + Neg + for<'a> AddAssign<&'a Self> + for<'a> MulAssign<&'a Self> + for<'a> SubAssign<&'a Self> + Eq
where
    for<'a, 'b> &'a Self : Add<&'b Self, Output = Self> + Mul<&'b Self, Output = Self> + Sub<&'b Self, Output = Self> + Neg<Output = Self>
{
    fn zero() -> Self;

    fn one() -> Self;
}

pub trait Field: Ring + Div + DivAssign + for<'a> DivAssign<&'a Self>
where
    for<'a, 'b> &'a Self : Add<&'b Self, Output = Self> + Mul<&'b Self, Output = Self> + Sub<&'b Self, Output = Self> + Neg<Output = Self> + Div<&'b Self, Output = Self>
{}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
}