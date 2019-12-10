mod util;
use util::vec2::*;

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Position {
    Asteroid,
    Empty
}

fn get(space: &Vec<Vec<Position>>, pos: &Vec2) -> Position {
    space[pos.1 as usize][pos.0 as usize]
}

fn in_bounds(pos: &Vec2, w: i64, h: i64) -> bool {
    pos.0 >= 0 && pos.0 < w && pos.1 >= 0 && pos.1 < h
}

fn gcd(a: i64, b: i64) -> i64 {
    assert!(!(a == 0 && b == 0));
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if (a > b) {
        gcd(a - b, b)
    } else if (b > a) {
        gcd(a, b - a)
    } else {
        assert_eq!(a, b);
        a
    }
}

fn count_visible_asteroids(pos: Vec2, space: &Vec<Vec<Position>>) -> (Vec2, i64) {
    println!("------------------------");
    dbg!(&pos);

    let h: i64 = space.len() as i64;
    let w: i64 = space[0].len() as i64;

//    dbg!((w, h));

    use Position::*;
    if get(space, &pos) == Empty {
        return (pos, 0);
    }

    let max_side = std::cmp::max(pos.0, std::cmp::max(pos.1, std::cmp::max(w - pos.0, h - pos.1))) + 1;
    dbg!(max_side);

    let mut hidden = HashSet::new();

    let asteroids_visible: i64 = (1..max_side)
        .map(|side_dist| {
            let t = pos.1 - side_dist;
            let r = pos.0 + side_dist + 1;
            let b = pos.1 + side_dist + 1;
            let l = pos.0 - side_dist;

            println!("{:?}", (t, r, b, l));

            let top_row = (l..r).cartesian_product(t..t+1).map(|(x, y)| Vec2(x, y));
            let bottom_row = (l..r).cartesian_product(b-1..b).map(|(x, y)| Vec2(x, y));
            let left_row = (l..l+1).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));
            let right_row = (r-1..r).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));

            top_row.chain(bottom_row).chain(left_row).chain(right_row)
        })
        .flatten()
        .filter(|pos| in_bounds(pos, w, h))
        .filter(|pos| get(space, pos) == Asteroid)
        .map(|look_pos| {
            let visible = !hidden.contains(&look_pos);
            if visible {
                println!("{}: visible", look_pos);

                let mut delta = look_pos - pos;
                dbg!(&delta);
                let delta_gcd = gcd(delta.0.abs(), delta.1.abs());
                delta.0 = delta.0 / delta_gcd;
                delta.1 = delta.1 / delta_gcd;
                dbg!(&delta);
                // Normalize straight vectors
//                if delta.0 == 0 {
//                    delta.1 = delta.1 / delta.1.abs(); // Preserve sign
//                } else if delta.1 == 0 {
//                    delta.0 = delta.0 / delta.0.abs();
//                } else if delta.0 == delta.1 {
//                    delta.1 = delta.1 / delta.1.abs();
//                    delta.0 = delta.0 / delta.0.abs();
//                }
//                dbg!(&delta);
                let mut current_pos = look_pos + delta;
                while in_bounds(&current_pos, w, h) {
                    println!("Hiding: {}", current_pos);
                    hidden.insert(current_pos);
                    current_pos = current_pos + delta;
                }

                1
            } else {
                println!("{}: not visible", look_pos);
                0
            }
        })
        .sum();

    println!("Result for pos {}: {}", pos, asteroids_visible);

    return (pos, asteroids_visible);
}

fn main() {
//    let input = std::fs::read_to_string("test_input1.txt").unwrap();
//    let input = std::fs::read_to_string("test_input2.txt").unwrap();
//    let input = std::fs::read_to_string("test_input3.txt").unwrap();
//    let input = std::fs::read_to_string("test_input4.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("gcd(2, 4) = {}", gcd(2, 4));
    dbg!(gcd(10, 0));

    println!("{}", input);

    use Position::*;

    let space: Vec<Vec<_>> = input.trim().split("\n").map(|row| {
        row.chars().map(|location|
            if location == '#' {
                Asteroid
            } else if location == '.' {
                Empty
            } else {
                panic!();
            }
        ).collect()
    }).collect();

    let h = space.len();
    let w = space[0].len();

    dbg!((0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| count_visible_asteroids(Vec2(x as i64, y as i64), &space))
        .max_by_key(|(pos, count)| *count)
    );

    // 371 too high
    // 362 too high
    // 359 too high
}
