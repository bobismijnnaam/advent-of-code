use std::fmt;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Hash, Copy, Clone)]
pub struct Vec2(pub i64, pub i64);

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2  {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2  {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Div<i64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub fn in_bounds(pos: &Vec2, w: i64, h: i64) -> bool {
    pos.0 >= 0 && pos.0 < w && pos.1 >= 0 && pos.1 < h
}

#[cfg(test)]
mod vec2_tests {
    use super::*;
    use std::ops::Add;

    #[test]
    fn test_add() {
        assert_eq!(Vec2(0, 0) + Vec2(1, 1), Vec2(1, 1));
        assert_eq!(Vec2(0, 0).add(Vec2(1, 1)), Vec2(1, 1))
    }
}

pub fn manhattan(a: Vec2, b: Vec2) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
