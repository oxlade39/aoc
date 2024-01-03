use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

pub trait Number:
    From<u8>
    + PartialEq
    + Add
    + Sub
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Display
{
}

impl<T> Number for T where
    T: From<u8>
        + PartialEq
        + Add
        + Sub
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + Copy
        + Display
{
}

pub fn lcm<T>(nums: &[T]) -> T
where
    T: Number,
{
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    let prod = a * b;
    prod / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers<T>(a: T, b: T) -> T
where
    T: Number,
{
    if b == 0.into() {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[test]
fn test_lcm_examples() {
    let n = [1, 2, 3];
    assert_eq!(6, lcm(&n));
}
