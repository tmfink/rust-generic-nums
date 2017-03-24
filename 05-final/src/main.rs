extern crate num;

use std::iter::Product;
use num::{PrimInt, Unsigned};

/// Find the factorial of n
fn factorial<T>(n: T) -> T
    where T: PrimInt + Unsigned + Product
{
    num::range(T::one(), n + T::one()).product()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn test_factorial_types(n: u8, expected: u8) {
        assert_eq!(factorial::<u8>(n as u8), expected as u8);
        assert_eq!(factorial::<u16>(n as u16), expected as u16);
        assert_eq!(factorial::<u32>(n as u32), expected as u32);
        assert_eq!(factorial::<u64>(n as u64), expected as u64);
    }

    #[test]
    fn test_factorial() {
        test_factorial_types(0, 1);
        test_factorial_types(1, 1);
        test_factorial_types(2, 2);
        test_factorial_types(3, 6);
        test_factorial_types(4, 24);
    }
}

fn main() {
    println!("u8: 3! = {}", factorial(3_u8));
    println!("u16: 3! = {}", factorial(3_u16));
    println!("u32: 3! = {}", factorial(3_u32));
    println!("u64: 3! = {}", factorial(3_u64));
}
