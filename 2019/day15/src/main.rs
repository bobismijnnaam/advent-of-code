mod util;
use util::intcode::*;
use util::direction::*;
use util::direction::Direction::*;

struct Robot {
    cpu: IntcodeCPU,
    pos: Vector2<i64>,
}

enum Status {
    HitWall,
    Ok,
    FoundOxygen
}

use Status::*;
use nalgebra::Vector2;
use std::collections::{HashSet, HashMap};
use std::thread::sleep;
use std::io::Write;
use std::time::Duration;

impl From<i64> for Status {
    fn from(v: i64) -> Self {
        match v {
            0 => HitWall,
            1 => Ok,
            2 => FoundOxygen,
            _ => panic!()
        }
    }
}

impl Robot {
    fn give_input(&mut self, dir: Direction) -> Status {
        self.cpu.resume(dir.into());
        self.cpu.output.remove(0).into()
    }
}

fn draw_world(robot: Vector2<i64>, oxygen: Vector2<i64>, walls: &HashSet<Vector2<i64>>, oxygenised: &HashSet<Vector2<i64>>) -> (Vector2<i64>, Vector2<i64>) {
    let min_x = walls.iter().map(|p| p.x).min().unwrap();
    let min_y = walls.iter().map(|p| p.y).min().unwrap();
    let max_x = walls.iter().map(|p| p.x).max().unwrap();
    let max_y = walls.iter().map(|p| p.y).max().unwrap();

    for y in min_y..max_y + 1 {
        for x in min_x.. max_x + 1 {
            if Vector2::new(x, y) == robot {
                print!("X");
            } else if x == 0 && y == 0 {
                print!("S");
            } else if Vector2::new(x, y) == oxygen {
                print!("O");
            } else if oxygenised.contains(&Vector2::new(x, y)) {
                print!("O");
            } else if walls.contains(&Vector2::new(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    (Vector2::new(min_x, min_y), Vector2::new(max_x, max_y))
}

fn erase(bounds: (Vector2<i64>, Vector2<i64>)) {
    for _y in bounds.0.y..bounds.1.y + 1 {
        print!("\r");
        for _x in bounds.0.x..bounds.1.x + 1 {
            print!(" ");
        }
        print!("\r");
        print!("\x1B[A");
    }
    for _x in bounds.0.x..bounds.1.x + 1 {
        print!(" ");
    }
    print!("\r");
}

fn scan_location(robot: &mut Robot, walls: &mut HashSet<Vector2<i64>>) {
    for dir in &[North, East, South, West] {
        match robot.give_input(*dir) {
            HitWall => {
                walls.insert(robot.pos + dir.to_vec());
            }
            Ok => {
                robot.give_input(dir.mirror());
            },
            FoundOxygen => {
                robot.give_input(dir.mirror());
            }
        }
    }
}

fn get_preferred_dir(current_dir: Direction, robot: &Robot, walls: &HashSet<Vector2<i64>>) -> Direction {
    if !walls.contains(&(robot.pos + current_dir.right().to_vec())) {
        // Can go right
        current_dir.right()
    } else if !walls.contains(&(robot.pos + current_dir.to_vec())) {
        // Can go forward
        current_dir
    } else if !walls.contains(&(robot.pos + current_dir.left().to_vec())) {
        // Can go left
        current_dir.left()
    } else {
        // Go back!
        current_dir.mirror()
    }
}

fn moves_required(start: Vector2<i64>, end: Vector2<i64>, walls: &HashSet<Vector2<i64>>) -> i64 {
    assert!(!walls.contains(&start));

    let mut cost = HashMap::new();
    cost.insert(start, 0i64);
    let mut frontier = vec![start];

    while !cost.contains_key(&end) {
        let pos = frontier.pop().unwrap();
        let pos_cost = *cost.get(&pos).unwrap();

        for dir in &[North, East, South, West] {
            let candidate = pos + dir.to_vec();
            if walls.contains(&candidate) {
                continue;
            }
            if cost.contains_key(&candidate) {
                let candidate_cost = *cost.get(&candidate).unwrap();
                if pos_cost + 1 < candidate_cost {
                    panic!("Is a maze. Should not happen.");
                }
            } else {
                cost.insert(candidate, pos_cost + 1);
                frontier.push(candidate);
            }
        }
    }

    *cost.get(&end).unwrap()
}

fn expand_ogyen(start: Vector2<i64>, walls: &HashSet<Vector2<i64>>) -> i64 {
    let mut oxygenised: HashSet<Vector2<i64>> = HashSet::new();
    oxygenised.insert(start);
    let mut frontier: Vec<Vector2<i64>> = vec![start];
    let mut prev_world: (Vector2<i64>, Vector2<i64>) = (Vector2::zeros(), Vector2::zeros());
    let draw = false;

    println!("");

    let mut count = 0;
    while frontier.len() > 0 {
        let current_frontier = frontier;
        frontier = vec![];

        for pos in current_frontier {
            for dir in &[North, East, South, West] {
                let candidate_pos = pos + dir.to_vec();
                if !oxygenised.contains(&candidate_pos) && !walls.contains(&candidate_pos) {
                    oxygenised.insert(candidate_pos);
                    frontier.push(candidate_pos);
                }
            }
        }

        // Only did an expansion of the previous frontier oxygenised any new positions, which means
        // they should've been added to the new frontier.
        if frontier.len() > 0 {
            count += 1;
        }

        if draw {
            erase(prev_world);
            prev_world = draw_world(Vector2::new(0, 0), start, &walls, &oxygenised);
            std::io::stdout().flush().ok();
            sleep(Duration::from_millis(50));
        }
    }

    count
}

fn main() {
    let mut robot = Robot {
        cpu: IntcodeCPU::new(program_from_file("input.txt")),
        pos: Vector2::zeros(),
    };

    println!("Looking...");
    println!("");

    let mut walls: HashSet<Vector2<i64>> = HashSet::new();
    let mut current_dir = North;
    let mut prev_world: (Vector2<i64>, Vector2<i64>) = (Vector2::zeros(), Vector2::zeros());
    let mut oxygen = Vector2::new(9999, 9999);
    let draw = true;
    loop {
        scan_location(&mut robot, &mut walls);

        let pref_dir = get_preferred_dir(current_dir, &robot, &walls);
        current_dir = pref_dir;

        match robot.give_input(current_dir) {
            HitWall => {
                panic!();
            }
            Ok => {
                robot.pos = robot.pos + current_dir.to_vec();
            },
            FoundOxygen => {
                robot.pos = robot.pos + current_dir.to_vec();
                oxygen = robot.pos;
            }
        }

        scan_location(&mut robot, &mut walls);

        if draw {
            erase(prev_world);
            prev_world = draw_world(robot.pos, oxygen, &walls, &HashSet::new());
            std::io::stdout().flush().ok();
            sleep(Duration::from_millis(50));
        }

        if robot.pos == Vector2::zeros() {
            println!("Made full round!");
            break;
        }
    }

    // 403 too low
    // 404 ok!
    assert_eq!(moves_required(Vector2::new(0, 0), oxygen, &walls), 404);

    // 407 too high
    // 406 ok!
    assert_eq!(expand_ogyen(oxygen, &walls), 406);
}
