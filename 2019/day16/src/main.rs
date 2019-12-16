use std::fs::read_to_string;
use itertools::Itertools;
use std::io::Write;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;

fn doublify(input: &Vec<i32>, amount: i32) -> Vec<i32> {
    assert!(amount > 0);

    if amount == 1 {
        return input.clone();
    }

    input.iter().map(|elem| {
        vec![*elem].into_iter().cycle().take(amount as usize)
    }).flatten().collect()
}

fn do_phase(input: &Vec<i32>) -> Vec<i32> {
    let base_pattern = vec![0, 1, 0, -1];

    (1..input.len() + 1)
//        .into_par_iter()
        .map(|i| {
            if (4 * i + 1) <= input.len() {
                let mut base_pattern_iter = base_pattern.iter().map(|elem| {
                    std::iter::repeat(*elem).take(i)
                }).flatten().cycle();

                base_pattern_iter.next();

                let v: i32 = input.iter()
                    .zip(base_pattern_iter)
                    .map(|(a, b)| a * b)
                    .sum();
                v.abs() % 10
            } else {
                let seg_one: i32 = input.iter()
                    .skip(i - 1)
                    .take(i)
                    .sum();

                let seg_minus_one: i32 = input.iter()
                    .skip(3 * i - 1)
                    .take(i)
                    .sum();

                (seg_one - seg_minus_one).abs() % 10
            }
        }).collect()
}

fn main2() {
    // Part 1
    let input = read_to_string("input.txt").unwrap();
    let mut input: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    dbg!(&input);

    dbg!(doublify(&vec![0, 1, 0, -1], 3));

    dbg!(do_phase(&vec![1, 2, 3, 4, 5, 6, 7, 8]));
    dbg!(do_phase(&do_phase(&vec![1, 2, 3, 4, 5, 6, 7, 8])));

    for i in 0..100 {
        input = do_phase(&input);
    }

    dbg!(input.iter().take(8).collect::<Vec<_>>());

    // 78009100

//    return;

    // Part 2

    let input = read_to_string("input.txt").unwrap();
    let mut input: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let offset: i32 = input.iter()
        .take(7)
        .map(|v| v.to_string())
        .fold1(|mut a, b| { a.push_str(b.as_str()); a })
        .unwrap()
        .parse()
        .unwrap();

    dbg!(offset);

    input = input.repeat(10000);

    for i in 0..100 {
        println!("{}", i);
        input = do_phase(&input);
        std::io::stdout().flush();
    }

    let res: Vec<_> = input.iter().skip(offset as usize).take(8).collect();
    dbg!(res);
}

fn main444() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let mut lut = HashMap::new();

    dbg!(calc_rec(&input, &mut lut, 0, 1));
    dbg!(calc_rec(&input, &mut lut, 1, 1));
    dbg!(calc_rec(&input, &mut lut, 2, 1));
    dbg!(calc_rec(&input, &mut lut, 3, 1));
    dbg!(calc_rec(&input, &mut lut, 4, 1));

    println!();

    dbg!(calc_rec(&input, &mut lut, 0, 2));
    dbg!(calc_rec(&input, &mut lut, 1, 2));
    dbg!(calc_rec(&input, &mut lut, 2, 2));
    dbg!(calc_rec(&input, &mut lut, 3, 2));
    dbg!(calc_rec(&input, &mut lut, 4, 2));

    let input = read_to_string("input.txt").unwrap();
    let input: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let offset: i32 = input.iter()
        .take(7)
        .map(|v| v.to_string())
        .fold1(|mut a, b| { a.push_str(b.as_str()); a })
        .unwrap()
        .parse()
        .unwrap();

    dbg!(offset);
    return;

    let input = input.repeat(10000);

    let mut lut = HashMap::new();

    println!("-------------------------");
    dbg!(calc_rec(&input, &mut lut, 0, 1));
    println!("-------------------------");
    dbg!(calc_rec(&input, &mut lut, 0, 2));
}

fn sub_phase(base: &[i32]) -> Vec<i32> {
    (0..base.len()).map(|i| {
        base[i..].iter().sum::<i32>() % 10
    }).collect()
}

fn sub_phase2(base: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = base.iter().rev().scan(0, |state, &x| {
        *state = *state + x;
        Some(*state % 10)
    }).collect();
    res.reverse();
    res

//    (0..base.len()).map(|i| {
//        base[i..].iter().sum::<i32>() % 10
//    }).collect()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let offset: i32 = input.iter()
        .take(7)
        .map(|v| v.to_string())
        .fold1(|mut a, b| { a.push_str(b.as_str()); a })
        .unwrap()
        .parse()
        .unwrap();

    dbg!(offset);

    let mut input = input.repeat(10000);
    let mut input = input[offset as usize..].to_vec();

    for i in 0..100 {
        dbg!(i);
        input = sub_phase2(&input);
    }

    dbg!(&input[0..8]);

    // 00555005 too low
}

// base: input

// result[i] = (alle indices van input die (index + 1) % 4 == 0) - (alle indices van input die (index + 1) % 4 == 3)
// let start_i = -1;
//  while start_i < input.len()
// from: start_i + 1 * i to start_i + 2 * i
// from: start_i + 3 * i to start_i + 4 * i

fn calc_rec(base: &[i32], lut: &mut HashMap<(i32, i32), i32>, p /* position of interest */: i32, depth: i32) -> i32 {
    if depth == 0 {
        base[p as usize]
    } else if let Some(v) = lut.get(&(p, depth)) {
        *v
    } else {
        dbg!(p);
        let mut start_i: i32 = -1;
        let width: i32 = p + 1;
        let mut pos_seg: i32 = 0;
        let mut neg_seg: i32 = 0;
        while start_i < base.len() as i32 {
//            dbg!(start_i);
//            println!("{} ({}%)", start_i, ((start_i as f64 / base.len() as f64) * 100 as f64).floor());

            let b1 = min(start_i + 1 * width, base.len() as i32);
            let b2 = min(start_i + 2 * width, base.len() as i32);
//            dbg!((b1, b2));
            for i in b1..b2 {
//                println!("(plus) Recalculate pos {} at depth {}", i, depth - 1);
                let res = calc_rec(base, lut, i, depth - 1);
//                dbg!(res);
                pos_seg += res;
            }

            let b1 = min(start_i + 3 * width, base.len() as i32);
            let b2 = min(start_i + 4 * width, base.len() as i32);
//            dbg!((b1, b2));
            for i in b1..b2 {
//                println!("(minus) Recalculate pos {} at depth {}", i, depth - 1);
                neg_seg += calc_rec(base, lut, i, depth - 1);
            }

            start_i += width * 4;
        }

        let res = (pos_seg - neg_seg).abs() % 10;
        lut.insert((p, depth), res);
        res
    }
}

// Offset == 12
// lcm(12, 650) == 39.00
// (10.000 * 650) == 1.666 2/3
// (10.000 * 650) - (1.666 * 3.900) == 2600
// v1 == base over 3.900 of elements (i.e. 6 repetitions of the input) (i.e. input.cycle.take(3900).zip(doublify(base_pattern).cycle()) )
// res == v1 * 1666 + base over first 2600 elements (i.e. just input.cycle().take(2600).zip(doublify(base_pattern).cycle())
