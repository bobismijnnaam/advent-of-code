use std::fs::read_to_string;

#[allow(dead_code)]
static BLACK: i64 = 0;
#[allow(dead_code)]
static WHITE: i64 = 1;
static TRANSPARENT: i64 = 2;

fn main() {
    let pixels : Vec<i64> = read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as i64)
        .collect();

    let w = 25;
    let h = 6;

    let mut minimum_zeroes = 9999999999999;
    let mut minimum_slice: &[i64] = &pixels;

    for start in (0..(pixels.len())).step_by(w*h) {
        dbg!(start);
        let candidate_slice: &[i64] = &pixels[start..start+w*h];
        let num_zeroes = candidate_slice.iter().filter(|p| **p == 0).count(); // TODO: && ipv **?
        if num_zeroes < minimum_zeroes {
            minimum_slice = candidate_slice;
            minimum_zeroes = num_zeroes;
        }
    }

    dbg!(minimum_zeroes);
    dbg!(minimum_slice);

    let num_ones = minimum_slice.iter().filter(|p| **p == 1).count();
    let num_twos = minimum_slice.iter().filter(|p| **p == 2).count();
    let answer = num_ones * num_twos;
    dbg!(answer);

    println!("Hello, world!");

    let image = (0..w*h).map(|offset| {
        (offset..pixels.len())
            .step_by(w*h)
            .map(|pixel_pos| pixels[pixel_pos])
            .find(|&pixel| pixel != TRANSPARENT)
            .unwrap()
    }).collect::<Vec<_>>();

    dbg!(&image);

    let pixel_char = "█";

    for y in 0..h {
        for x in 0..w {
            let pixel = image[y*w+x];
            if pixel == 1 {
                print!("{}", pixel_char);
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // GKCKH
}
