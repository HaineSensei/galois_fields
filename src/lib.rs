#![recursion_limit = "2048"]
// This module's core logic doesn't seem to work in the current version of Rust, but I'll leave this around in case it does in the future.
use std::{fmt::Display, ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg}};

pub mod polynomial;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

pub trait Variable : Clone + Eq + Display {}

impl Variable for Indeterminate {}

pub const T: Indeterminate = Indeterminate::new('T');
pub const X: Indeterminate = Indeterminate::new('X');
pub const Y: Indeterminate = Indeterminate::new('Y');
pub const Z: Indeterminate = Indeterminate::new('Z');

trait Ring: Add + Mul + Sub + Clone + AddAssign + MulAssign + SubAssign + Neg
{}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
}