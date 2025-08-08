use super::*;

#[test]
fn test_imod_construction() {
    let a = IMod::<5>::new(7);
    let b = IMod::<5>::new(2);
    
    assert_eq!(a.val, 2); // 7 % 5 = 2
    assert_eq!(b.val, 2);
    assert_eq!(a, b);
}

#[test]
fn test_imod_addition() {
    let a = IMod::<7>::new(3);
    let b = IMod::<7>::new(5);
    let result = a + b;
    
    assert_eq!(result.val, 1); // (3 + 5) % 7 = 1
}

#[test]
fn test_imod_multiplication() {
    let a = IMod::<5>::new(3);
    let b = IMod::<5>::new(4);
    let result = a * b;
    
    assert_eq!(result.val, 2); // (3 * 4) % 5 = 2
}

#[test]
fn test_imod_subtraction() {
    let a = IMod::<7>::new(2);
    let b = IMod::<7>::new(5);
    let result = a - b;
    
    assert_eq!(result.val, 4); // (2 - 5) % 7 = -3 % 7 = 4
}

#[test]
fn test_imod_negation() {
    let a = IMod::<5>::new(3);
    let neg_a = -a;
    
    assert_eq!(neg_a.val, 2); // -3 % 5 = 2
    
    let sum = a + neg_a;
    assert_eq!(sum.val, 0); // Should be additive inverse
}

#[test]
fn test_imod_division() {
    let a = IMod::<7>::new(3);
    let b = IMod::<7>::new(2);
    let result = a / b;
    
    // 3 / 2 in F7: need inverse of 2, which is 4 (since 2*4 = 8 ≡ 1 mod 7)
    // So 3 / 2 = 3 * 4 = 12 ≡ 5 mod 7
    assert_eq!(result.val, 5);
    
    // Verify: result * b should equal a
    let verify = result * b;
    assert_eq!(verify, a);
}

#[test]
fn test_imod_inverse() {
    let a = IMod::<7>::new(3);
    let inv = a.inverse().unwrap();
    
    let product = a * inv;
    assert_eq!(product.val, 1); // Should be multiplicative identity
}

#[test]
fn test_imod_zero_and_one() {
    type F5 = IMod<5>;
    let zero = F5::zero();
    let one = F5::one();
    
    assert_eq!(zero.val, 0);
    assert_eq!(one.val, 1);
    
    // Test zero properties
    let a = F5::new(3);
    assert_eq!(a + zero, a);
    assert_eq!(a * zero, zero);
    
    // Test one properties
    assert_eq!(a * one, a);
}

#[test]
fn test_imod_assign_operations() {
    let mut a = IMod::<11>::new(7);
    let b = IMod::<11>::new(4);
    
    // Test AddAssign
    a += b;
    assert_eq!(a.val, 0); // (7 + 4) % 11 = 0
    
    // Reset and test SubAssign
    a = IMod::<11>::new(3);
    a -= b;
    assert_eq!(a.val, 10); // (3 - 4) % 11 = 10
    
    // Reset and test MulAssign
    a = IMod::<11>::new(5);
    a *= b;
    assert_eq!(a.val, 9); // (5 * 4) % 11 = 9
    
    // Reset and test DivAssign
    a = IMod::<11>::new(8);
    a /= b;
    // 8 / 4 = 2 in any field
    assert_eq!(a.val, 2);
}

#[test]
fn test_imod_reference_operations() {
    let a = IMod::<5>::new(3);
    let b = IMod::<5>::new(2);
    
    // Test reference addition
    let result = &a + &b;
    assert_eq!(result.val, 0); // (3 + 2) % 5 = 0
    
    // Test reference multiplication
    let result = &a * &b;
    assert_eq!(result.val, 1); // (3 * 2) % 5 = 1
    
    // Test reference subtraction
    let result = &a - &b;
    assert_eq!(result.val, 1); // (3 - 2) % 5 = 1
    
    // Test reference division
    let result = &a / &b;
    assert_eq!(result.val, 4); // 3 / 2 in F5
}

#[test]
#[should_panic(expected = "non-unit")]
fn test_division_by_zero() {
    let a = IMod::<5>::new(3);
    let zero = IMod::<5>::new(0);
    let _result = a / zero; // Should panic
}