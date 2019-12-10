#![feature(vec_remove_item)]

mod util;
use util::vec2::*;

use itertools::Itertools;
use std::collections::HashSet;
use std::thread::current;

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
fn get_visible_asteroids_field(pos: Vec2, space: &Vec<Vec<Position>>) -> Vec<Vec2> {
    use Position::*;
    let h: i64 = space.len() as i64;
    let w: i64 = space[0].len() as i64;
    let asteroids: Vec<_> = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| Vec2(x as i64, y as i64))
        .filter(|pos| get(&space, pos) == Asteroid)
        .collect();
    get_visible_asteroids(pos, &asteroids, w, h)
}

fn get_visible_asteroids(pos: Vec2, space: &Vec<Vec2>, w: i64, h: i64) -> Vec<Vec2> {
//    println!("------------------------");
//    dbg!(&pos);

    let max_side = std::cmp::max(pos.0, std::cmp::max(pos.1, std::cmp::max(w - pos.0, h - pos.1))) + 1;
//    dbg!(max_side);

    let mut hidden = HashSet::new();

    if !space.contains(&pos) {
        return vec![];
    }

    (1..max_side)
        .map(|side_dist| {
            let t = pos.1 - side_dist;
            let r = pos.0 + side_dist + 1;
            let b = pos.1 + side_dist + 1;
            let l = pos.0 - side_dist;

//            println!("{:?}", (t, r, b, l));

            let top_row = (l..r).cartesian_product(t..t+1).map(|(x, y)| Vec2(x, y));
            let right_row = (r-1..r).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));
            let bottom_row = (l..r).cartesian_product(b-1..b).map(|(x, y)| Vec2(x, y));
            let left_row = (l..l+1).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));

            top_row.chain(bottom_row).chain(left_row).chain(right_row)
        })
        .flatten()
        .filter(|pos| in_bounds(pos, w, h))
        .filter(|pos| space.contains(pos))
        .map(|look_pos| {
            let visible = !hidden.contains(&look_pos);
            if visible {
//                println!("{}: visible", look_pos);

                let mut delta = look_pos - pos;
//                dbg!(&delta);
                let delta_gcd = gcd(delta.0.abs(), delta.1.abs());
                delta.0 = delta.0 / delta_gcd;
                delta.1 = delta.1 / delta_gcd;
//                dbg!(&delta);
                let mut current_pos = look_pos + delta;
                while in_bounds(&current_pos, w, h) {
//                    println!("Hiding: {}", current_pos);
                    hidden.insert(current_pos);
                    current_pos = current_pos + delta;
                }

                Some(look_pos)
            } else {
//                println!("{}: not visible", look_pos);
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

fn count_visible_asteroids(pos: Vec2, space: &Vec<Vec<Position>>) -> (Vec2, i64) {
    let visible_asteroids = get_visible_asteroids_field(pos, space);

//    println!("Result for pos {}: {}", pos, visible_asteroids.len());

    return (pos, visible_asteroids.len() as i64);
}

fn angle(a: Vec2, b: Vec2) -> f64 {
    let dot = a.0 * b.0 + a.1 * b.1;
    let det = a.0 * b.1 - a.1 * b.0;
    let angle = (det as f64).atan2(dot as f64);
    if angle < 0f64 {
        2f64 * std::f64::consts::PI + angle
    } else {
        angle
    }
}

fn get_least(space: &Vec<Vec2>, min_angle: Option<f64>) -> Option<Vec2> {
    if space.len() == 0 {
        return None;
    }

    let mut res: Option<Vec2> = None;
    let mut res_angle = 0.;
    for candidate in space {
        let candidate_angle = angle(Vec2(0, -1), *candidate);
        if res.is_none() && min_angle.is_none() {
            res = Some(*candidate);
            res_angle = candidate_angle
        } else if res.is_none() && min_angle.is_some() && candidate_angle > min_angle.unwrap() {
            res = Some(*candidate);
            res_angle = candidate_angle;
        } else if res.is_some() && min_angle.is_none() && candidate_angle < res_angle {
            res = Some(*candidate);
            res_angle = candidate_angle;
        } else if res.is_some() && min_angle.is_some() && candidate_angle < res_angle && candidate_angle > min_angle.unwrap() {
            res = Some(*candidate);
            res_angle = candidate_angle;
        }
    }

    res
}

fn main() {
//    let input = std::fs::read_to_string("test_input1.txt").unwrap();
//    let input = std::fs::read_to_string("test_input2.txt").unwrap();
//    let input = std::fs::read_to_string("test_input3.txt").unwrap();
//    let input = std::fs::read_to_string("test_input4.txt").unwrap();
//    let input = std::fs::read_to_string("test_input5.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    dbg!(angle(Vec2(0, -1), Vec2(3, -3)));
    dbg!(angle(Vec2(0, -1), Vec2(3, 3)));
    dbg!(angle(Vec2(0, -1), Vec2(-3, 3)));
    dbg!(angle(Vec2(0, -1), Vec2(-3, -3)));

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
                panic!("{}", location);
            }
        ).collect()
    }).collect();

    let h = space.len() as i64;
    let w = space[0].len() as i64;

    let (best_pos, vision_score) = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| count_visible_asteroids(Vec2(x as i64, y as i64), &space))
        .max_by_key(|(pos, count)| *count).unwrap();

    dbg!(best_pos);
    dbg!(vision_score);

    let mut asteroids: Vec<_> = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| Vec2(x as i64, y as i64))
        .filter(|pos| get(&space, pos) == Asteroid)
        .collect();

    let mut remove_count = 0;

    let mut min_angle = None;
    while asteroids.len() > 0 {
        println!("-----------------");
        // Get visible asteroids
        let visible_asteroids = get_visible_asteroids(best_pos, &asteroids, w, h).into_iter()
            .map(|pos| pos - best_pos)
            .collect();
//        dbg!(&visible_asteroids);

        let current_target = get_least(&visible_asteroids, min_angle);

        dbg!(&current_target);
        dbg!(&min_angle);

        if current_target.is_none() && min_angle.is_none() {
            // We're done
            break
        } else if current_target.is_some() && min_angle.is_none() {
            // First hit, save the angle
            min_angle = Some(angle(Vec2(0, -1), current_target.unwrap()));
            // Hit the target
            remove_count += 1;
            asteroids.remove_item(&current_target.unwrap());
            println!("Removing: {}", best_pos + current_target.unwrap());
            if remove_count == 200 {
                let target = current_target.unwrap() + best_pos;
                dbg!(target.0 * 100 + target.1);
                break;
            }
        } else if current_target.is_some() && min_angle.is_some() {
            min_angle = Some(angle(Vec2(0, -1), current_target.unwrap()));
            // Hit the target
            remove_count += 1;
            asteroids.remove_item(&current_target.unwrap());
            println!("Removing: {}", best_pos + current_target.unwrap());
            if remove_count == 200 {
                let target = current_target.unwrap() + best_pos;
                dbg!(target.0 * 100 + target.1);
                break;
            }
        } else if current_target.is_none() && min_angle.is_some() {
            // Finish rotation
            min_angle = None;
        }

//        // Do rotation
//        let mut current_target = get_least(&asteroids, None);
//
//        remove_count += 1;
//        asteroids.remove_item(&current_target.unwrap());
//        println!("Removing: {}", best_pos + current_target.unwrap());
//        if remove_count == 200 {
//            let target = current_target.unwrap() + best_pos;
//            dbg!(target.0 * 100 + target.1);
//        }
//
//        current_target = get_least(&asteroids, Some(angle(Vec2(0, -1), current_target.unwrap())));
//        while current_target.is_some() {
//            asteroids.remove_item(&current_target.unwrap());
//            remove_count += 1;
//            println!("Removing: {}", best_pos + current_target.unwrap());
//            if remove_count == 200 {
//                let target = current_target.unwrap() + best_pos;
//                dbg!(target.0 * 100 + target.1);
//            }
//            current_target = get_least(&asteroids, Some(angle(Vec2(0, -1), current_target.unwrap())))
//        }
    }

    // 371 too high
    // 362 too high
    // 359 too high

    // 1723 wrong. too low
}
