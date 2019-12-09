#[derive(Eq, Ord, PartialOrd, PartialEq, Debug)]
pub struct Vec2(pub i64, pub i64);

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2  {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
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
