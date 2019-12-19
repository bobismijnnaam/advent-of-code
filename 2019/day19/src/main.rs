mod util;
use util::intcode::*;
use itertools::Itertools;

use nalgebra::Vector2;
use std::convert::TryInto;
use std::collections::HashMap;

struct Drone {
    cpu: IntcodeCPU,
    width_cache: HashMap<i64, i64>,
    height_cache: HashMap<i64, i64>,
    start_x_cache: HashMap<i64, i64>,
    start_y_cache: HashMap<i64, i64>
}

impl Drone {
    fn new() -> Drone {
        Drone {
            cpu: IntcodeCPU::new(program_from_file("input.txt")),
            width_cache: Default::default(),
            height_cache: Default::default(),
            start_x_cache: Default::default(),
            start_y_cache: Default::default(),
        }
    }

    fn fly_to(&self, pos: Vector2<i64>) -> i64 {
        let mut cpu = self.cpu.clone();
        cpu.input.push(pos.x);
        cpu.input.push(pos.y);
        cpu.start();
        cpu.output.remove(0)
    }

    fn tractor_start_x(&self, y: i64) -> i64 {
        assert!(y >= 5);
        (0..)
            .take_while(|x| {
                self.fly_to(Vector2::new(*x, y.into())) == 0
            })
            .count().try_into().unwrap()
    }

    fn tractor_start_y(&self, x: i64) -> i64 {
        assert!(x >= 5);
        (0..)
            .take_while(|y| {
                self.fly_to(Vector2::new(x.into(), *y)) == 0
            })
            .count().try_into().unwrap()
    }

    fn tractor_width(&self, y: i64) -> i64 {
        assert!(y >= 5);
        (0..)
            .skip_while(|x| {
                self.fly_to(Vector2::new(*x, y.into())) == 0
            })
            .take_while(|x| {
                self.fly_to(Vector2::new(*x, y.into())) == 1
            })
            .count().try_into().unwrap()
    }

    fn tractor_height(&self, x: i64) -> i64 {
        assert!(x >= 5);
        (0..)
            .skip_while(|y| {
                self.fly_to(Vector2::new(x, *y)) == 0
            })
            .take_while(|y| {
                self.fly_to(Vector2::new(x, *y)) == 1
            })
            .count().try_into().unwrap()
    }

    fn tractor_width_c(&mut self, y: i64) -> i64 {
        if let Some(width) = self.width_cache.get(&y) {
            *width
        } else {
            let width = self.tractor_width(y);
            self.width_cache.insert(y, width);
            width
        }
    }

    fn tractor_height_c(&mut self, x: i64) -> i64 {
        if let Some(height) = self.height_cache.get(&x) {
            *height
        } else {
            let height = self.tractor_height(x);
            self.height_cache.insert(x, height);
            height
        }
    }

    fn tractor_start_x_c(&mut self, y: i64) -> i64 {
        if let Some(start_x) = self.start_x_cache.get(&y) {
            *start_x
        } else {
            let start_x = self.tractor_start_x(y);
            self.start_x_cache.insert(y, start_x);
            start_x
        }
    }

    fn tractor_start_y_c(&mut self, x: i64) -> i64 {
        if let Some(start_y) = self.start_y_cache.get(&x) {
            *start_y
        } else {
            let start_y = self.tractor_start_y(x);
            self.start_y_cache.insert(x, start_y);
            start_y
        }
    }
}

fn main() {
    let mut drone = Drone::new();

    let w = 50;
    let h = 50;
    let beams: i64 = (0..w).cartesian_product(0..h)
        .map(|(x, y)| {
            drone.fly_to(Vector2::new(x, y))
        })
        .sum();
    dbg!(beams);

    if false {
        let w = 300;
        let h = 300;

        for y in 0..h {
            for x in 0..w {
                match drone.fly_to(Vector2::new(x, y)) {
                    0 => print!("."),
                    1 => print!("#"),
                    _ => panic!()
                }
            }
            println!();
        }
    }

    let santa_side = 100;

//    for y in 10.. {
//        if drone.tractor_width(y) >= 100 {
//            println!("Candidate y: {}", y);
//        }
//    }

    let mut res = None;

    for y in 686.. {

        let tractor_width = drone.tractor_width_c(y);

        println!("y: {}, width: {}", y, tractor_width);

        if tractor_width < santa_side {
            continue;
        }

        let start_x = drone.tractor_start_x_c(y);

        for x in start_x..start_x + tractor_width {
            let start_y = drone.tractor_start_y_c(x);
            let tractor_height = drone.tractor_height_c(x);
            let leftover_space_x = start_x + tractor_width - x;
            let leftover_space_y = start_y + tractor_height - y;
            if leftover_space_x >= santa_side && leftover_space_y >= santa_side {
                res = Some(Vector2::new(x, y));
                break;
            }
        }

        if res.is_some() {
            break;
        }
    }

    dbg!(res);

    if let Some(pos) = res {
        println!("{}", pos.x * 10000 + pos.y);
    }
}
