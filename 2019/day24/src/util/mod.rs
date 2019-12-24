#![allow(dead_code)]

// TODO: Script that runs all

use nalgebra::Vector2;

pub mod vec2;
pub mod intcode;
pub mod direction;

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

pub fn manhattan(a: Vector2<i32>, b: Vector2<i32>) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// From wikipedia?
pub fn safe_multiply(a: i64, b: i64, size: i64) -> i64 {
    let (a, b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };

    let a = (a + size) % size;
    let b = (b + size) % size;

    if a == 1 {
        return b;
    } else if b == 1 {
        return a;
    } else if a == 0 || b == 0 {
        return 0;
    }

    if (b % 2) == 0 {
        // b even
        // a * b = 2 * a * (b / 2),

        let x = (2 * a) % size;
        safe_multiply(x, b / 2, size)
    } else {
        // b odd
        // a * b = a + a * (b - 1)
        let x = safe_multiply(a, b - 1, size);
        (a + x) % size
    }
}

// Stolen from geeksforgeeks iirc
fn mod_inverse(mut a: i64, mut m: i64) -> i64 {
    let m0 = m;
    let mut y = 0;
    let mut x = 1;

    while a > 1 {
        let q = a / m;
        let mut t = m;

        m = a % m;
        a = t;
        t = y;

        y = x - q * y;
        x = t;
    }

    if x < 0 {
        x += m0;
    }

    x
}

pub static PIXEL_CHAR: char = '█';
pub static VAGUE_PIXEL_CHAR: char = '▒';
