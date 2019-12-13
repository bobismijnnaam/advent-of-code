mod util;
use util::intcode::*;
use util::{PIXEL_CHAR, VAGUE_PIXEL_CHAR};

use itertools::Itertools;
use std::collections::HashMap;
use nalgebra::Vector2;
use std::cmp::max;
use std::fmt;
use std::io::Write;

#[derive(Eq, PartialEq, Copy, Clone)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

fn parse_tile_type(tile_type: i64) -> TileType {
    use TileType::*;

    match tile_type {
        0 => Empty,
        1 => Wall,
        2 => Block,
        3 => Paddle,
        4 => Ball,
        _ => panic!()
    }
}

struct Screen {
    tiles: HashMap<Vector2<i64>, TileType>,
    score: i64,
}

fn apply_screen_delta(screen: &mut Screen, cpu: &IntcodeCPU) {
    cpu.output.chunks(3).for_each(|xs| {
        let x = xs[0];
        let y = xs[1];
        let loc_type = xs[2];
        if x == -1 && y == 0 {
            screen.score = loc_type;
        } else if x < 0 || y < 0 {
            println!("Skipped {},{}:{}", x, y, loc_type);
        } else {
            screen.tiles.insert(Vector2::new(x, y), parse_tile_type(loc_type));
        }
    });
}

impl Screen {
    fn bounds(&self) -> Vector2<i64> {
        let max_x = self.tiles.keys()
            .map(|v| v.x).fold1(|a, b| max(a, b)).unwrap();
        let max_y = self.tiles.keys().
            map(|v| v.y).fold1(|a, b| max(a, b)).unwrap();
        Vector2::new(max_x + 1, max_y + 2)
    }

    fn find_tile_pos(&self, tile_type: TileType) -> Option<Vector2<i64>> {
        let bounds = self.bounds();
        for y in 0..bounds.y {
            for x in 0..bounds.x {
                if self.get_tile(Vector2::new(x, y)) == tile_type {
                    return Some(Vector2::new(x, y));
                }
            }
        }

        None
    }

    fn get_tile(&self, pos: Vector2<i64>) -> TileType {
        *self.tiles.get(&pos).unwrap_or(&TileType::Empty)
    }
}

fn render_screen(screen: &Screen) {
    println!("SCORE: {}", screen.score);

    let bounds = screen.bounds();

    use TileType::*;
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            if let Some(field_type) = screen.tiles.get(&Vector2::new(x, y)) {
                match field_type {
                    Empty => print!(" "),
                    Wall => print!("{}", PIXEL_CHAR),
                    Block => print!("{}", VAGUE_PIXEL_CHAR),
                    Paddle => print!("_"),
                    Ball => print!(".")
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn input (message: &'_ impl fmt::Display) -> String {
    use std::*;
    print!("{}", message);
    io::stdout().flush();
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

enum UserInput {
    Neutral,
    Left,
    Right
}

fn parse_user_input(user_input: String) -> Option<UserInput> {
    let user_input = user_input.trim();
    use UserInput::*;
    match user_input {
        "l" | "left" => Some(Left),
        "r" | "right" => Some(Right),
        "n" | "neutral" => Some(Neutral),
        _ => None
    }
}

fn main() {
    println!("Hello, world!");

    {
        let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
        cpu.start();
        dbg!(&cpu.output);
        let num_blocks = cpu.output.chunks(3).map(|x| x[2]).filter(|&x| x == 2).count();
        dbg!(num_blocks);
    }

    let mut screen = Screen {
        tiles: HashMap::new(),
        score: 0
    };
    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));

    cpu.set_memory(0, 2);
    cpu.start();
    apply_screen_delta(&mut screen, &cpu);

    while cpu.state != IntcodeState::Halt {
        use TileType::*;
        use UserInput::*;

        render_screen(&screen);

        println!("Input: (l)eft, (r)ight, (n)eutral");

        let ball_pos = screen.find_tile_pos(Ball).unwrap();
        let paddle_pos = screen.find_tile_pos(Paddle).unwrap();

        let input = if ball_pos.x < paddle_pos.x {
            Left
        } else if paddle_pos.x < ball_pos.x {
            Right
        } else {
            Neutral
        };

        cpu.output.clear();
        cpu.resume(match input {
            Left => -1,
            Right => 1,
            Neutral => 0
        });
        apply_screen_delta(&mut screen, &cpu);

        // Manual mode:
//        if let Some(user_input) = parse_user_input(input(&"Input: ")) {
//            cpu.output.clear();
//            cpu.resume(match user_input {
//                Left => -1,
//                Right => 1,
//                Neutral => 0
//            });
//            apply_screen_delta(&mut screen, &cpu);
//        } else {
//            println!("Input invalid, please try again");
//        }
    }

    std::io::stdout().flush();
    render_screen(&screen);
    std::io::stdout().flush();
}
