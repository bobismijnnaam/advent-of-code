use std::fs::read_to_string;
use std::collections::HashSet;
use nalgebra::Vector2;

mod util;
use util::direction::Direction::*;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::once;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, PartialOrd)]
struct Location {
    depth: i32,
    position: Vector2<i32>,
}

impl Location {
    fn new(depth: i32, position: Vector2<i32>) -> Location {
        Location {
            depth,
            position
        }
    }

    fn is_center(&self, size: Vector2<i32>) -> bool {
        self.position == Vector2::new(size.x / 2, size.y / 2)
    }

    fn side_left(size: Vector2<i32>, depth: i32) -> Vec<Location> {
        (0..size.y)
            .map(|y| Location::new(depth, Vector2::new(0, y)))
            .collect()
    }

    fn side_right(size: Vector2<i32>, depth: i32) -> Vec<Location> {
        (0..size.y)
            .map(|y| Location::new(depth, Vector2::new(size.x - 1, y)))
            .collect()
    }

    fn side_top(size: Vector2<i32>, depth: i32) -> Vec<Location> {
        (0..size.x)
            .map(|x| Location::new(depth, Vector2::new(x, 0)))
            .collect()
    }

    fn side_bottom(size: Vector2<i32>, depth: i32) -> Vec<Location> {
        (0..size.x)
            .map(|x| Location::new(depth, Vector2::new(x, size.y - 1)))
            .collect()
    }

    fn neighbours(&self, size: Vector2<i32>) -> Vec<Location> {
        (&[North, East, South, West]).iter()
            .map(move |dir| Location {
                depth: self.depth,
                position: self.position + dir.to_vec()
            })
            .map(|loc| {
                if loc.position.x == -1 {
                    vec![Location {
                        depth: self.depth - 1,
                        position: Vector2::new(1, 2)
                    }]
                } else if loc.position.y == -1 {
                    vec![Location {
                        depth: self.depth - 1,
                        position: Vector2::new(2, 1)
                    }]
                } else if loc.position.x == size.x {
                    vec![Location {
                        depth: self.depth - 1,
                        position: Vector2::new(3, 2)
                    }]
                } else if loc.position.y == size.y {
                    vec![Location {
                        depth: self.depth - 1,
                        position: Vector2::new(2, 3)
                    }]
                } else if loc.is_center(size) {
                    if self.position.x == 1 {
                        Location::side_left(size, loc.depth + 1)
                    } else if self.position.x == 3 {
                        Location::side_right(size, loc.depth + 1)
                    } else if self.position.y == 1 {
                        Location::side_top(size, loc.depth + 1)
                    } else if self.position.y == 3 {
                        Location::side_bottom(size, loc.depth + 1)
                    } else {
                        panic!()
                    }
                } else {
                    vec![loc]
                }.into_iter()
            })
            .flatten()
            .collect()
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth)
            .then(self.position.x.cmp(&other.position.x))
            .then(self.position.y.cmp(&other.position.y))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct MyHashSet(HashSet<Location>);

impl Hash for MyHashSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().sorted().for_each(|pos| pos.hash(state))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Field {
    bugs: MyHashSet,
    size: Vector2<i32>,
}

fn parse_field(input: &str) -> Field {
    let rows: Vec<Vec<_>> = input.trim().split("\n").map(|row| row.chars().collect()).collect();

    let mut bugs: HashSet<Location> = HashSet::new();

    for y in 0..rows.len() {
        for x in 0..rows[0].len() {
            if rows[y][x] == '#' {
                bugs.insert(Location::new(0, Vector2::new(x as i32, y as i32)));
            }
        }
    }

    Field {
        bugs: MyHashSet(bugs),
        size: Vector2::new(rows[0].len() as i32, rows.len() as i32)
    }
}

impl Field {
    fn has_bug(&self, loc: Location) -> bool {
        self.bugs.0.contains(&loc)
    }

    fn neighbour_bugs(&self, loc: Location) -> i32 {
        loc.neighbours(self.size)
            .iter()
            .filter(|&&loc| self.has_bug(loc))
            .count() as i32
    }

    fn active_depth(&self) -> (i32, i32) {
        let min_depth = self.bugs.0.iter()
            .map(|loc| loc.depth)
            .min().unwrap();

        let max_depth = self.bugs.0.iter()
            .map(|loc| loc.depth)
            .max().unwrap();

        (min_depth - 1, max_depth + 1)
    }

    fn next(&self) -> Field {
        let mut bugs = HashSet::new();
        let (min_depth, max_depth) = self.active_depth();

        for depth in min_depth ..= max_depth {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    let loc = Location::new(depth, Vector2::new(x, y));
                    let neighbour_bugs = self.neighbour_bugs(loc);
                    if loc.is_center(self.size) {
                        continue
                    } else if self.has_bug(loc) {
                        // Is live bug
                        if neighbour_bugs == 1 {
                            bugs.insert(loc);
                        }
                    } else {
                        // Is empty space
                        if neighbour_bugs == 1 || neighbour_bugs == 2 {
                            bugs.insert(loc);
                        }
                    }
                }
            }
        }

        Field {
            size: self.size,
            bugs: MyHashSet(bugs),
        }
    }

    fn print(&self) {
        let (min_depth, max_depth) = self.active_depth();

        for depth in min_depth ..= max_depth {
            self.print_depth(depth);
            println!();
        }
    }

    fn print_depth(&self, depth: i32) {
        println!("Depth {}:", depth);

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let loc = Location::new(depth, Vector2::new(x, y));
                if y == 2 && x == 2 {
                    print!("?");
                } else if self.has_bug(loc) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn num_bugs(&self) -> i32 {
        self.bugs.0.len() as i32
    }
}

fn main() {
    let input = read_to_string("test1.txt").unwrap();

    let mut field = parse_field(&input);

    println!("Begin:");
    field.print();

    for i in 0..10 {
        field = field.next();
    }

    println!("After 10:");
    field.print();





    let input = read_to_string("input.txt").unwrap();

    let mut field = parse_field(&input);

    for i in 0..200 {
        field = field.next()
    }

    println!();

    dbg!(field.num_bugs());
}
