
use std::{collections::HashMap, ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign}};

use crate::{Field, Indeterminate, Ring};

#[derive(Debug)]
pub struct Polynomial<R: Ring, const VAR: Indeterminate>
where
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    vals: HashMap<usize,R>,
}

impl<R,const VAR: Indeterminate> Polynomial<R,VAR> 
where
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    pub fn new(vals: &HashMap<usize,R>) -> Self {
        Self {
            vals: vals.iter().filter_map(|(x,y)|
                if y != &R::zero() {
                    Some((*x, y.clone()))
                } else {
                    None
                }
            ).collect()
        }
    }

    fn prune_zeros(&mut self) {
        let Self {vals} = self;
        let keys = vals.keys().cloned().collect::<Vec<_>>();
        for x in keys {
            if vals.get(&x) == Some(&R::zero()) {
                vals.remove(&x);
            }
        }
    }
}

impl<R,const VAR: Indeterminate> Clone for Polynomial<R, VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn clone(&self) -> Self {
        Self::new(&self.vals)
    }
}

impl<'a, 'b, R, const VAR: Indeterminate> Add<&'b Polynomial<R,VAR>> for &'a Polynomial<R,VAR>
where 
    R: Ring,
    for<'c, 'd> &'c R : Add<&'d R, Output = R> + Mul<&'d R, Output = R> + Sub<&'d R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn add(self, rhs: &'b Polynomial<R,VAR>) -> Self::Output
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
        Self::Output::new(&vals)
    }
}

impl<R, const VAR: Indeterminate> Add for Polynomial<R,VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<'c,'d, R, const VAR: Indeterminate> Mul<&'c Polynomial<R,VAR>> for &'d Polynomial<R,VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn mul(self, rhs: &'c Polynomial<R,VAR>) -> Self::Output {
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

        Self::Output::new(&vals)
    }
}

impl<R,const VAR: Indeterminate> Mul for Polynomial<R,VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<R,const VAR: Indeterminate> Neg for &Polynomial<R,VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R,VAR>;

    fn neg(self) -> Self::Output {
        let vals = self.vals.iter().map(|(x,y)| (*x, -y)).collect();
        Self::Output::new(&vals)
    }
}

impl<R,const VAR: Indeterminate> Neg for Polynomial<R,VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl<'c,'d,R,const VAR: Indeterminate> Sub<&'c Polynomial<R,VAR>> for &'d Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R,VAR>;

    fn sub(self, rhs: &'c Polynomial<R,VAR>) -> Self::Output {
        self + &-rhs
    }
}

impl<R,const VAR: Indeterminate> Sub for Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<R,const VAR: Indeterminate> AddAssign for Polynomial<R,VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
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
        self.prune_zeros();
    }
}

impl<R,const VAR: Indeterminate> MulAssign for Polynomial<R,VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.vals = (self.clone()*rhs).vals;
    }
}

impl<R,const VAR: Indeterminate> SubAssign for Polynomial<R,VAR>
where
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<'a, R,const VAR: Indeterminate> SubAssign<&'a Polynomial<R,VAR>> for Polynomial<R,VAR>
where
    R: Ring,
    for<'c, 'b> &'c R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn sub_assign(&mut self, rhs: &'a Polynomial<R,VAR>) {
        *self += -rhs;
    }
}

impl<'a, R,const VAR: Indeterminate> AddAssign<&'a Polynomial<R,VAR>> for Polynomial<R,VAR>
where
    R: Ring,
    for<'c, 'b> &'c R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn add_assign(&mut self, rhs: &'a Polynomial<R,VAR>) {
        *self = &self.clone() + rhs;
    }
}

impl<'a, R,const VAR: Indeterminate> MulAssign<&'a Polynomial<R,VAR>> for Polynomial<R,VAR>
where
    R: Ring,
    for<'c, 'b> &'c R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn mul_assign(&mut self, rhs: &'a Polynomial<R,VAR>) {
        *self = &self.clone() * rhs;
    }
}

impl<R, const VAR: Indeterminate> PartialEq for Polynomial<R, VAR>
where
    R: Ring,
    for<'c, 'b> &'c R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn eq(&self, other: &Self) -> bool {
        // TODO: Check that `self.vals` and `other.vals` are forced to be "pruned" already.
        self.vals == other.vals
    }
}

impl<R, const VAR: Indeterminate> Eq for Polynomial<R, VAR>
where
    R: Ring,
    for<'c, 'b> &'c R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{}

impl<R, const VAR: Indeterminate> Ring for Polynomial<R, VAR>
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    fn zero() -> Self {
        Self::new(&HashMap::new())
    }

    fn one() -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(0,R::one());
        Self::new(&hash_map)
    }
}

impl<R, const VAR: Indeterminate> Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    pub fn deg(&self) -> Option<usize> {
        self.vals.keys().max().cloned()
    }

    pub fn leading<'a>(&'a self) -> Option<&'a R> {
        self.vals.get(&self.deg()?)
    }

    pub fn indeterminant_power(pow:usize) -> Self {
        let mut hash_map = HashMap::new();
        hash_map.insert(pow,R::one());
        Self::new(&hash_map)
    }
}

impl<'c, 'd, R, const VAR: Indeterminate> Mul<&'c R> for &'d Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn mul(self, rhs: &'c R) -> Self::Output {
        Self::Output::new(&self.vals.iter().map(|(&x,y)| (x, y*rhs)).collect())
    }
}

impl<'d, R, const VAR: Indeterminate> Mul<R> for &'d Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn mul(self, rhs: R) -> Self::Output {
        Self::Output::new(&self.vals.iter().map(|(&x,y)| (x, y*&rhs)).collect())
    }
}

impl<'c, 'd, R, const VAR: Indeterminate> Div<&'c R> for &'d Polynomial<R, VAR> 
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn div(self, rhs: &'c R) -> Self::Output {
        Self::Output::new(&self.vals.iter().map(|(&x,y)| (x, y/rhs)).collect())
    }
}

impl<'d, R, const VAR: Indeterminate> Div<R> for &'d Polynomial<R, VAR> 
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn div(self, rhs: R) -> Self::Output {
        Self::Output::new(&self.vals.iter().map(|(&x,y)| (x, y/&rhs)).collect())
    }
}

impl<'c, 'd, R, const VAR: Indeterminate> Div<&'c Polynomial<R, VAR>> for &'d Polynomial<R, VAR>
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn div(self, rhs: &'c Polynomial<R, VAR>) -> Self::Output {
        let mut out = Self::Output::zero();
        let mut curr = self.clone();
        let mut curr_deg = match curr.deg() {
            Some(x) => x,
            None => return out,
        };
        let rhs_deg = rhs.deg().expect("Hey, stop trying to divide by zero!");
        while curr_deg >= rhs_deg {
            let curr_leading = curr.leading().expect("er... shouldn't this be impossible to get to?");
            let rhs_leading = rhs.leading().expect("Again, this should be impossible to get to.");
            let factor = Polynomial::indeterminant_power(curr_deg-rhs_deg) * (curr_leading/rhs_leading);
            let rhs_multiple = rhs*&factor;
            assert_eq!(rhs_multiple.deg(),curr.deg());
            curr -= rhs_multiple;
            out += factor;
            curr_deg = match curr.deg() {
                Some(x) => x,
                None => return out,
            };
        }
        out
    }
}

impl<'c, 'd, R, const VAR: Indeterminate> Rem<&'c Polynomial<R, VAR>> for &'d Polynomial<R, VAR>
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Polynomial<R, VAR>;

    fn rem(self, rhs: &'c Polynomial<R, VAR>) -> Self::Output {
        let x = self/rhs;
        self - &(&x*rhs)
        // self - (self/rhs)*rhs
    }
}

impl<R, const VAR: Indeterminate> Mul<R> for Polynomial<R, VAR> 
where 
    R: Ring,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    type Output = Self;

    fn mul(self, rhs: R) -> Self::Output {
        &self * rhs
    }
}

impl<R, const VAR: Indeterminate> Div<R> for Polynomial<R, VAR> 
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Self;

    fn div(self, rhs: R) -> Self::Output {
        &self / rhs
    }
}

impl<R, const VAR: Indeterminate> Div<Polynomial<R, VAR>> for Polynomial<R, VAR>
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        &self / &rhs
    }
}

impl<R, const VAR: Indeterminate> Rem<Polynomial<R, VAR>> for Polynomial<R, VAR>
where 
    R: Field,
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R> + Div<&'b R, Output = R>
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        &self % &rhs
    }
}

#[cfg(test)]
mod test;
