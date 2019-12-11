mod util;
use util::intcode::*;
use util::vec2::*;
use std::collections::HashMap;
use crate::util::PIXEL_CHAR;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum PaintColor {
    Black,
    White
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum TurnDirection {
    Left90,
    Right90
}

#[derive(Clone)]
struct HullPaintingRobot {
    pos: Vec2,
    dir: Vec2,
    cpu: IntcodeCPU,
}

impl HullPaintingRobot {
    fn cycle(&mut self, current_color: PaintColor) -> (PaintColor, TurnDirection)  {
        use PaintColor::*;
        use TurnDirection::*;

        self.cpu.resume(match current_color {
            Black => 0,
            White => 1
        });

        let paint_color = match self.cpu.output.remove(0) {
            0 => Black,
            1 => White,
            _ => panic!()
        };

        let turn_direction = match self.cpu.output.remove(0) {
            0 => Left90,
            1 => Right90,
            _ => panic!()
        };

        (paint_color, turn_direction)
    }

    fn is_running(&self) -> bool {
        self.cpu.state != IntcodeState::Halt
    }

    fn rotate(&mut self, turn_direction: TurnDirection) {
        use TurnDirection::*;
        self.dir = match turn_direction {
            Left90 => self.dir.cc90(),
            Right90 => self.dir.cw90()
        }
    }

    fn move_forward(&mut self) {
        self.pos = self.pos + self.dir;
    }
}

fn main() {
    use PaintColor::*;

    println!("Hello, world!");

    let mut hpr = HullPaintingRobot {
        pos: Vec2(0, 0),
        dir: Vec2(0, -1),
        cpu: IntcodeCPU::new(program_from_file("input.txt"))
    };

    let mut hull: HashMap<Vec2, PaintColor> = HashMap::new();
    hull.insert(Vec2(0, 0), White);

    while hpr.is_running() {
        let current_color =
            if let Some(&color) = hull.get(&hpr.pos) {
                color
            } else {
                Black
            };

        let (paint_color, turn_direction) = hpr.cycle(current_color);
        hull.insert(hpr.pos, paint_color);

        hpr.rotate(turn_direction);
        hpr.move_forward();
    }

    dbg!(hull.keys().len());

    let min_x = hull.keys().map(|p| p.0).min().unwrap();
    let max_x = hull.keys().map(|p| p.0).max().unwrap();
    let min_y = hull.keys().map(|p| p.1).min().unwrap();
    let max_y = hull.keys().map(|p| p.1).max().unwrap();

    dbg!(min_x);
    dbg!(max_x);
    dbg!(min_y);
    dbg!(max_y);

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if let Some(Black) = hull.get(&Vec2(x, y)) {
                print!("{0:}{0:}", PIXEL_CHAR);
            } else {
                print!("  ");
            }
        }
        println!();
    }

    // RJLFBUCU
}
