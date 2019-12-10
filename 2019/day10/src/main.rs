#![feature(vec_remove_item)]

// TODO: Optimize cartesian products to plain maps in get_visible_asteroids?

mod util;
use util::vec2::*;
use util::gcd;

use itertools::Itertools;
use std::collections::HashSet;

fn get_visible_asteroids(pos: Vec2, space: &Vec<Vec2>, w: i64, h: i64) -> Vec<Vec2> {
    let max_side = [pos.0, pos.1, w - pos.0, h - pos.1].iter().max().unwrap() + 1;

    let mut hidden = HashSet::new();

    (1..max_side)
        .map(|side_dist| {
            let t = pos.1 - side_dist;
            let r = pos.0 + side_dist;
            let b = pos.1 + side_dist;
            let l = pos.0 - side_dist;

            let top_row = (l..r + 1).map(move |x| Vec2(x, t));
            let right_row = (t + 1..b).map(move |y| Vec2(r, y));
            let bottom_row = (l..r + 1).map(move |x| Vec2(x, b));
            let left_row = (t + 1..b).map(move |y| Vec2(l, y));

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

fn main() {
//    let input = std::fs::read_to_string("test_input1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("{}", input);

    let mut asteroids: Vec<Vec2> = input.trim().split("\n").enumerate().map(|(y, row)| {
        row.chars()
            .enumerate()
            .filter_map(move |(x, position)| {
                match position {
                    '#' => Some(Vec2(x as i64, y as i64)),
                    _ => None
                }
            })
    }).flatten().collect();

    let h = *asteroids.iter().map(|Vec2(_, y)| y).max().unwrap() + 1;
    let w = *asteroids.iter().map(|Vec2(x, _)| x).max().unwrap() + 1;

    let (best_pos, vision_score) = asteroids.iter()
        .map(|pos| (*pos, get_visible_asteroids(*pos, &asteroids, w, h).len()))
        .max_by_key(|(pos, count)| *count).unwrap();

    dbg!(best_pos);
    dbg!(vision_score);

    let calc_angle = |pos: &Vec2| {
        Vec2(0, -1).angle(&(*pos - best_pos))
    };

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

    let target = removed_asteroids[199]; // Select the 200th asteroid
    dbg!(target.0 * 100 + target.1);

    // 371 too high
    // 362 too high
    // 359 too high

    // 1723 wrong. too low
    // 2732 good!
}
