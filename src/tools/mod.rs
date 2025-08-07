//! All created by Claude. Small edits made by human author.


/// Extended Euclidean Algorithm
/// Returns (gcd, x, y) such that ax + by = gcd(a, b)
/// This is used to find modular inverses: if gcd(a, m) = 1, then x is the inverse of a mod m
pub const fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    
    (gcd, x, y)
}

/// Find the modular inverse of a modulo m
/// Returns Some(inverse) if gcd(a, m) = 1, None otherwise
pub const fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (gcd, x, _y) = extended_gcd(a as i128, m as i128);
    
    if gcd != 1 {
        None // No inverse exists
    } else {
        // Make sure the result is non-negative cos for some reason x % y isn't always non-negative.
        let inverse = (x % (m as i128) + m as i128) % (m as i128);
        Some(inverse as u64)
    }
}

/// Compute a^b mod m using fast modular exponentiation
pub const fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    
    let mut result = 1u128;
    let mut base = (base as u128) % (modulus as u128);
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % (modulus as u128);
        }
        exp >>= 1;
        base = (base * base) % (modulus as u128);
    }
    
    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_gcd() {
        // Test basic cases
        let (gcd, x, y) = extended_gcd(10, 6);
        assert_eq!(gcd, 2);
        assert_eq!(10 * x + 6 * y, gcd);
        
        // Test coprime numbers
        let (gcd, x, y) = extended_gcd(7, 3);
        assert_eq!(gcd, 1);
        assert_eq!(7 * x + 3 * y, gcd);
    }
    
    #[test]
    fn test_mod_inverse() {
        // Test inverse of 3 mod 7
        let inv = mod_inverse(3, 7).unwrap();
        assert_eq!((3 * inv) % 7, 1);
        
        // Test inverse of 5 mod 11
        let inv = mod_inverse(5, 11).unwrap();
        assert_eq!((5 * inv) % 11, 1);
        
        // Test no inverse exists (gcd != 1)
        assert_eq!(mod_inverse(6, 9), None);
    }
    
    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24); // 2^10 = 1024, 1024 % 1000 = 24
        assert_eq!(mod_pow(3, 4, 7), 4); // 3^4 = 81, 81 % 7 = 4
        assert_eq!(mod_pow(5, 0, 13), 1); // Any number to power 0 is 1
    }
}