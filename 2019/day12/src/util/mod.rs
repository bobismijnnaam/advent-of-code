#![allow(dead_code)]

// TODO: Script that runs all

pub mod vec2;
pub mod intcode;

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    assert!(a > 0 && b > 0);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    println!("gcd({},{})", a, b);
    (a / gcd(a, b)) * b
}

pub static PIXEL_CHAR: char = '█';
