
use std::{collections::HashMap, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::{Ring, Variable};

trait NotHighDepthPolynomial {}

struct Polynomial<R: Ring + NotHighDepthPolynomial, Var: Variable>
where
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    vals: HashMap<usize,R>,
    variable: Var
}

impl<R,V> Clone for Polynomial<R, V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
{
    fn clone(&self) -> Self {
        Self { vals: self.vals.clone(), variable: self.variable.clone() }
    }
}

impl<'a, 'b, R, V> Add<&'b Polynomial<R,V>> for &'a Polynomial<R,V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'c, 'd> &'c R : Add<&'d R, Output = R>
{
    type Output = Polynomial<R, V>;

    fn add(self, rhs: &'b Polynomial<R,V>) -> Self::Output
    {
        let mut vals = self.vals.clone();
        for (i, x) in &rhs.vals {
            match vals.get_mut(i) {
                Some(y) => {
                    *y += x.clone();
                },
                None => {
                    vals.insert(*i, x.clone());
                },
            }
        }
        Self::Output {
            vals,
            variable: self.variable.clone(),
        }
    }
}

impl<R, V> Add for Polynomial<R,V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Add<&'b R, Output = R>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'c,'d, R,V> Mul<&'c Polynomial<R,V>> for &'d Polynomial<R,V> 
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Mul<&'b R, Output = R>
{
    type Output = Polynomial<R, V>;

    fn mul(self, rhs: &'c Polynomial<R,V>) -> Self::Output {
        let mut vals = HashMap::new();
        for (i, x) in &self.vals {
            for (j, y) in &rhs.vals {
                match vals.get_mut(&(i+j)) {
                    Some(curr) => {
                        *curr += x*y
                    },
                    None => {
                        vals.insert(i+j, x*y);
                    },
                }
            }
        }

        Self::Output {
            vals,
            variable: self.variable.clone()
        }
    }
}

impl<R,V> Mul for Polynomial<R,V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Mul<&'b R, Output = R>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<R,V> Neg for &Polynomial<R,V> 
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a> &'a R : Neg<Output = R>
{
    type Output = Polynomial<R,V>;

    fn neg(self) -> Self::Output {
        let vals = self.vals.iter().map(|(x,y)| (*x, -y)).collect();
        Self::Output {
            vals,
            variable: self.variable.clone()
        }
    }
}

impl<R,V> Neg for Polynomial<R,V> 
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a> &'a R : Neg<Output = R>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'c,'d,R,V> Sub<&'c Polynomial<R,V>> for &'d Polynomial<R, V> 
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Sub<&'b R, Output = R>
{
    type Output = Polynomial<R,V>;

    fn sub(self, rhs: &'c Polynomial<R,V>) -> Self::Output {
        self + &-rhs
    }
}

impl<R,V> Sub for Polynomial<R, V> 
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Sub<&'b R, Output = R>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<R,V> AddAssign for Polynomial<R,V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
{
    fn add_assign(&mut self, rhs: Self) {
        for (i, x) in &rhs.vals {
            match self.vals.get_mut(i) {
                Some(y) => {
                    *y += x.clone();
                },
                None => {
                    self.vals.insert(*i,x.clone());
                },
            }
        }
    }
}

impl<R,V> MulAssign for Polynomial<R,V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.vals = (self.clone()*rhs).vals
    }
}

impl<R,V> SubAssign for Polynomial<R,V>
where
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<R, V> Ring for Polynomial<R, V>
where 
    R: Ring + NotHighDepthPolynomial,
    V: Variable,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{}