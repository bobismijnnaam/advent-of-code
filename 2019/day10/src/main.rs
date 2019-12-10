#![feature(vec_remove_item)]

mod util;
use util::vec2::*;
use util::gcd;

use itertools::Itertools;
use std::collections::HashSet;
use std::thread::current;

// TODO: Get rid of this
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Position {
    Asteroid,
    Empty
}

fn get(space: &Vec<Vec<Position>>, pos: &Vec2) -> Position {
    space[pos.1 as usize][pos.0 as usize]
}

fn get_visible_asteroids(pos: Vec2, space: &Vec<Vec2>, w: i64, h: i64) -> Vec<Vec2> {
    let max_side = [pos.0, pos.1, w - pos.0, h - pos.1].iter().max().unwrap() + 1;

    let mut hidden = HashSet::new();

    (1..max_side)
        .map(|side_dist| {
            let t = pos.1 - side_dist;
            let r = pos.0 + side_dist + 1;
            let b = pos.1 + side_dist + 1;
            let l = pos.0 - side_dist;

            let top_row = (l..r).cartesian_product(t..t+1).map(|(x, y)| Vec2(x, y));
            let right_row = (r-1..r).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));
            let bottom_row = (l..r).cartesian_product(b-1..b).map(|(x, y)| Vec2(x, y));
            let left_row = (l..l+1).cartesian_product(t+1..b-1).map(|(x, y)| Vec2(x, y));

            top_row.chain(bottom_row).chain(left_row).chain(right_row)
        })
        .flatten()
        .filter(|pos| in_bounds(pos, w, h))
        .filter(|pos| space.contains(pos))
        .filter_map(|look_pos| {
            let visible = !hidden.contains(&look_pos);
            if visible {
                let mut delta = look_pos - pos;
                let delta_gcd = gcd(delta.0.abs(), delta.1.abs());
                delta = delta / delta_gcd;

                let mut current_pos = look_pos + delta;
                while in_bounds(&current_pos, w, h) {
                    hidden.insert(current_pos);
                    current_pos = current_pos + delta;
                }

                Some(look_pos)
            } else {
                None
            }
        })
        .collect()
}

fn count_visible_asteroids(pos: Vec2, space: &Vec<Vec2>, w: i64, h: i64) -> (Vec2, i64) {
    let visible_asteroids = get_visible_asteroids(pos, space, w, h);

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

fn main() {
//    let input = std::fs::read_to_string("test_input1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

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

    let mut asteroids: Vec<_> = (0..w)
        .cartesian_product(0..h)
        .map(|(x, y)| Vec2(x as i64, y as i64))
        .filter(|pos| get(&space, pos) == Asteroid)
        .collect();

    let (best_pos, vision_score) = asteroids.iter()
        .map(|pos| count_visible_asteroids(*pos, &asteroids, w, h))
        .max_by_key(|(pos, count)| *count).unwrap();

    dbg!(best_pos);
    dbg!(vision_score);

    let calc_angle = |pos: &Vec2| {
        angle(Vec2(0, -1), *pos - best_pos)
    };

    let mut remove_count = 0;
    let remove_target = 200;

    let mut removed_asteroids = vec![];

    while asteroids.len() > 1 {
        let visible_asteroids: Vec<_> = get_visible_asteroids(best_pos, &asteroids, w, h)
            .into_iter()
            .sorted_by(|pos_a, pos_b| {
                let angle_a = calc_angle(pos_a);
                let angle_b = calc_angle(pos_b);
                angle_a.partial_cmp(&angle_b).unwrap()
            })
            .collect();

            for asteroid in &visible_asteroids {
                asteroids.remove_item(asteroid);
                removed_asteroids.push(*asteroid);
            }
    }

    let target = removed_asteroids[199];
    dbg!(target.0 * 100 + target.1);

    // 371 too high
    // 362 too high
    // 359 too high

    // 1723 wrong. too low
    // 2732 good!
}
