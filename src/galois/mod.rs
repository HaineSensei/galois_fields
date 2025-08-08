use crate::{modulo_ints::IMod, polynomial::Polynomial, Field, Indeterminate};

#[derive(Clone, PartialEq, Eq)]
pub struct GF<const P: u64, const N: usize, const SYMBOL: Indeterminate> {
    val: [IMod<P>; N],
    irred_poly: Polynomial<IMod<P>,SYMBOL>
}

impl<const P: u64, const N: usize, const SYMBOL: Indeterminate> GF<P, N, SYMBOL> {
    pub fn new(val: &[IMod<P>;N], irred_poly: &[IMod<P>]) -> Self {
        todo!()
    }
}