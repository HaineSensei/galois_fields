
use super::*;
use crate::{modulo_ints::IMod, X, Y};
use std::collections::HashMap;

// Helper function to create a polynomial from coefficients
fn poly_from_coeffs<R: Ring, const VAR: Indeterminate>(coeffs: &[(usize, R)]) -> Polynomial<R, VAR>
where
    for<'a, 'b> &'a R : Add<&'b R, Output = R> + Mul<&'b R, Output = R> + Sub<&'b R, Output = R> + Neg<Output = R>
{
    let mut map = HashMap::new();
    for &(power, ref coeff) in coeffs {
        map.insert(power, coeff.clone());
    }
    Polynomial::new(&map)
}

#[test]
fn test_polynomial_construction() {
    // Create polynomial 3x^2 + 2x + 1 over IMod<5>
    type F5 = IMod<5>;
    let p: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(1)),
        (1, F5::new(2)),
        (2, F5::new(3)),
    ]);
    
    assert_eq!(p.deg(), Some(2));
    assert_eq!(p.leading(), Some(&F5::new(3)));
}

#[test]
fn test_polynomial_addition() {
    type F7 = IMod<7>;
    // p1 = 2x^2 + 3x + 1
    let p1: Polynomial<F7, X> = poly_from_coeffs(&[
        (0, F7::new(1)),
        (1, F7::new(3)),
        (2, F7::new(2)),
    ]);
    
    // p2 = x^2 + 4x + 2
    let p2: Polynomial<F7, X> = poly_from_coeffs(&[
        (0, F7::new(2)),
        (1, F7::new(4)),
        (2, F7::new(1)),
    ]);
    
    // p1 + p2 = 3x^2 + 0x + 3 = 3x^2 + 3
    let result = p1 + p2;
    assert_eq!(result.deg(), Some(2));
    // Coefficient of x^2 should be 2 + 1 = 3
    // Coefficient of x^1 should be 3 + 4 = 0 (mod 7)
    // Coefficient of x^0 should be 1 + 2 = 3
}

#[test]
fn test_polynomial_multiplication_by_scalar() {
    type F5 = IMod<5>;
    // p = 2x + 3
    let p: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(3)),
        (1, F5::new(2)),
    ]);
    
    // Multiply by 2: should get 4x + 1 (since 2*3 = 6 ≡ 1 mod 5)
    let result = &p * F5::new(2);
    assert_eq!(result.deg(), Some(1));
}

#[test]
fn test_zero_and_one() {
    type F3 = IMod<3>;
    let zero: Polynomial<F3, Y> = Polynomial::zero();
    let one: Polynomial<F3, Y> = Polynomial::one();
    
    assert_eq!(zero.deg(), None); // Zero polynomial has no degree
    assert_eq!(one.deg(), Some(0)); // One polynomial has degree 0
    assert_eq!(one.leading(), Some(&F3::new(1)));
}

#[test]
fn test_indeterminate_power() {
    type F7 = IMod<7>;
    let x_cubed: Polynomial<F7, X> = Polynomial::indeterminant_power(3);
    
    assert_eq!(x_cubed.deg(), Some(3));
    assert_eq!(x_cubed.leading(), Some(&F7::new(1)));
}

#[test]
fn test_polynomial_negation() {
    type F5 = IMod<5>;
    // p = 2x + 3
    let p: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(3)),
        (1, F5::new(2)),
    ]);
    
    let neg_p = -&p;
    // In F5: -2 ≡ 3, -3 ≡ 2
    assert_eq!(neg_p.deg(), Some(1));
}

#[test]
fn test_polynomial_division_simple() {
    type F7 = IMod<7>;
    // Test dividing x^2 by x
    // Should get quotient = x, remainder = 0
    let dividend: Polynomial<F7, X> = Polynomial::indeterminant_power(2); // x^2
    let divisor: Polynomial<F7, X> = Polynomial::indeterminant_power(1);  // x
    
    let quotient = &dividend / &divisor;
    let remainder = &dividend % &divisor;
    
    assert_eq!(quotient.deg(), Some(1)); // Should be x
    assert_eq!(remainder.deg(), None);   // Should be 0
    
    // Verify: dividend = quotient * divisor + remainder
    let reconstructed = &quotient * &divisor + remainder;
    assert_eq!(dividend, reconstructed);
}

#[test]
fn test_polynomial_division_with_remainder() {
    type F5 = IMod<5>;
    // Test dividing 2x^2 + 3x + 1 by x + 1
    let dividend: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(1)), // constant term
        (1, F5::new(3)), // x term
        (2, F5::new(2)), // x^2 term
    ]);
    
    let divisor: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(1)), // constant term: 1
        (1, F5::new(1)), // x term: 1  (so this is x + 1)
    ]);
    
    let quotient = &dividend / &divisor;
    let remainder = &dividend % &divisor;
    
    // Quotient should have degree 1, remainder should have degree 0
    assert_eq!(quotient.deg(), Some(1));
    assert!(remainder.deg().unwrap_or(0) < divisor.deg().unwrap());
    
    // Verify: dividend = quotient * divisor + remainder
    let reconstructed = &quotient * &divisor + remainder;
    assert_eq!(dividend, reconstructed);
}

#[test]
fn test_polynomial_division_exact() {
    type F7 = IMod<7>;
    // Test dividing x^3 - 1 by x - 1
    // This should divide exactly: (x - 1)(x^2 + x + 1) = x^3 - 1
    let dividend: Polynomial<F7, X> = poly_from_coeffs(&[
        (0, F7::new(6)), // -1 ≡ 6 mod 7
        (3, F7::new(1)), // x^3
    ]);
    
    let divisor: Polynomial<F7, X> = poly_from_coeffs(&[
        (0, F7::new(6)), // -1 ≡ 6 mod 7
        (1, F7::new(1)), // x
    ]);
    
    let quotient = &dividend / &divisor;
    let remainder = &dividend % &divisor;
    
    // Should divide exactly, so remainder = 0
    assert_eq!(remainder.deg(), None);
    assert_eq!(quotient.deg(), Some(2)); // x^2 + x + 1
    
    // Verify: dividend = quotient * divisor
    let reconstructed = &quotient * &divisor;
    assert_eq!(dividend, reconstructed);
}

#[test]
fn test_polynomial_scalar_division() {
    type F5 = IMod<5>;
    // Test dividing 2x^2 + 4x + 2 by 2
    // Should get x^2 + 2x + 1
    let p: Polynomial<F5, X> = poly_from_coeffs(&[
        (0, F5::new(2)),
        (1, F5::new(4)), 
        (2, F5::new(2)),
    ]);
    
    let result = &p / F5::new(2);
    
    assert_eq!(result.deg(), Some(2));
    // In F5: 2/2 = 1, 4/2 = 2, 2/2 = 1
}

#[test]
#[should_panic(expected = "divide by zero")]
fn test_division_by_zero_polynomial() {
    type F5 = IMod<5>;
    let dividend: Polynomial<F5, X> = Polynomial::one();
    let zero_divisor: Polynomial<F5, X> = Polynomial::zero();
    
    let _result = &dividend / &zero_divisor; // Should panic
}
