use cgmath::Vector3;
mod util;
use util::*;

use itertools::Itertools;

fn attract_axis(pos_a: i64, pos_b: i64) -> (i64, i64) {
    if pos_a > pos_b {
        (-1, 1)
    } else if pos_a < pos_b {
        (1, -1)
    } else {
        (0, 0)
    }
}

fn calculate_speed_delta(pos_a: &Vector3<i64>, pos_b: &Vector3<i64>) -> (Vector3<i64>, Vector3<i64>) {
    let mut delta_a = Vector3::new(0, 0, 0);
    let mut delta_b = delta_a;
    for axis in 0..3 {
        let axis_delta = attract_axis(pos_a[axis], pos_b[axis]);
        delta_a[axis] = axis_delta.0;
        delta_b[axis] = axis_delta.1;
    }

    (delta_a, delta_b)
}

fn update_velocities(positions: &[Vector3<i64>], velocities: &mut [Vector3<i64>]) {
    for planets in (0..positions.len()).combinations(2) {
        let (delta_a, delta_b) = calculate_speed_delta(
            &positions[planets[0]],
            &positions[planets[1]]
        );

        for (planet, delta) in planets.iter().zip(&[delta_a, delta_b]) {
            velocities[*planet] += *delta;
        }
    }
}

fn apply_velocities(positions: &mut [Vector3<i64>], velocities: &[Vector3<i64>]) {
    // Iterate over all current planets... Sigh
    for (i, position) in positions.iter_mut().enumerate() {
        *position += velocities[i];
    }
}

fn time_step(positions: &mut [Vector3<i64>], velocities: &mut [Vector3<i64>]) {
    update_velocities(positions, velocities);
    apply_velocities(positions, velocities);
}

fn sum(v: Vector3<i64>) -> i64 {
    v.x + v.y + v.z
}

fn total_energy(positions: &[Vector3<i64>], velocities: &[Vector3<i64>]) -> i64 {
    positions
        .iter()
        .map(|position| sum(position.map(|x| x.abs())))
        .zip(velocities
            .iter()
            .map(|velocity| sum(velocity.map(|x| x.abs())))
        )
        .map(|(potential_energy, kinetic_energy)| potential_energy * kinetic_energy)
        .sum()
}

fn parse_vec3_triple(triple_str: &str) -> Vector3<i64> {
    // Take off < and >
    let triple_str = &triple_str[1..triple_str.len() - 1];
    let elems: Vec<i64> = triple_str
        .split(",")
        .map(|elem| elem.split("=").nth(1).unwrap().parse().unwrap())
        .collect();

    Vector3::new(elems[0], elems[1], elems[2])
}

fn find_period(axis: usize, mut positions: Vec<Vector3<i64>>, mut velocities: Vec<Vector3<i64>>) -> i64 {
    let starting_positions = positions.clone();
    let starting_velocities = velocities.clone();

    println!("axis: {}", axis);
    for i in 1..10000000 {
        time_step(&mut positions, &mut velocities);

        let mutees = positions.iter().map(|pos| pos[axis])
            .chain(velocities.iter().map(|vel| vel[axis]));
        let origs = starting_positions.iter().map(|pos| pos[axis])
            .chain(starting_velocities.iter().map(|vel| vel[axis]));

        // If all the [axis] coordinates of the planets are the same we completed a full cycle
        if mutees.zip(origs).map(|(mutee, orig)| mutee == orig).fold1(|a, b| a && b).unwrap() {
            return i;
        }
    }

    panic!();
}

fn main() {
    println!("Hello, world!");

    let input = std::fs::read_to_string("input.txt").expect("");
    dbg!(&input);
    let mut positions: Vec<_> = input.trim().split("\n").map(|triple_str| parse_vec3_triple(triple_str)).collect();
    dbg!(&positions);

    let mut velocities = vec![
        Vector3::new(0, 0, 0),
        Vector3::new(0, 0, 0),
        Vector3::new(0, 0, 0),
        Vector3::new(0, 0, 0),
    ];

    let starting_positions = positions.clone();
    let starting_velocities = velocities.clone();

    for i in 0..1000 {
        time_step(&mut positions, &mut velocities);
    }

    dbg!(&positions);
    dbg!(&velocities);

    dbg!(total_energy(&positions, &velocities));

    // 5164: too low
    // 336336482000: too high
    // 84084241000: too high

    let res = (0..3)
        .map(|axis| {
            find_period( axis as usize, positions.clone(), velocities.clone())
        })
        .fold1(|a, b| {
            let res = lcm(a, b);
            println!("Res: {}", res);
            res
        })
        .unwrap();

    dbg!(res);

    assert_eq!(res, 353620566035124);

    // Too high: 1768102830175620
    // yay: 353620566035124
}
