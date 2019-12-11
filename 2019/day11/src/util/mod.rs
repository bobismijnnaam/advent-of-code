#![allow(dead_code)]

// TODO: Script that runs all

pub mod vec2;
pub mod intcode;

pub fn gcd(a: i64, b: i64) -> i64 {
    assert!(!(a == 0 && b == 0));
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a > b {
        gcd(a - b, b)
    } else if b > a {
        gcd(a, b - a)
    } else {
        assert_eq!(a, b);
        a
    }
}

pub static PIXEL_CHAR: char = '█';
