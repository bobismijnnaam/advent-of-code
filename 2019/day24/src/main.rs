use std::fs::read_to_string;
use std::collections::HashSet;
use nalgebra::Vector2;

mod util;
use util::direction::Direction::*;
use std::hash::{Hash, Hasher};
use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::once;
use std::time::{Instant, Duration};

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

    fn side_left_it(size: Vector2<i32>, depth: i32) -> impl Iterator<Item = Location> {
        (0..size.y)
            .map(move |y| Location::new(depth, Vector2::new(0, y)))
    }

    fn side_right_it(size: Vector2<i32>, depth: i32) -> impl Iterator<Item = Location> {
        (0..size.y)
            .map(move |y| Location::new(depth, Vector2::new(size.x - 1, y)))
    }

    fn side_top_it(size: Vector2<i32>, depth: i32) -> impl Iterator<Item = Location> {
        (0..size.x)
            .map(move |x| Location::new(depth, Vector2::new(x, 0)))
    }

    fn side_bottom_it(size: Vector2<i32>, depth: i32) -> impl Iterator<Item = Location> {
        (0..size.x)
            .map(move |x| Location::new(depth, Vector2::new(x, size.y - 1)))
    }

    fn neighbours(&self, size: Vector2<i32>) -> Box<dyn Iterator<Item = Location> + '_> {
        Box::new((&[North, East, South, West]).iter()
            .map(move |dir| Location {
                depth: self.depth,
                position: self.position + dir.to_vec()
            })
            .map(move |loc| {
                if loc.position.x == -1 {
                    Location {
                        depth: self.depth - 1,
                        position: Vector2::new(1, 2)
                    }
                } else if loc.position.y == -1 {
                    Location {
                        depth: self.depth - 1,
                        position: Vector2::new(2, 1)
                    }
                } else if loc.position.x == size.x {
                    Location {
                        depth: self.depth - 1,
                        position: Vector2::new(3, 2)
                    }
                } else if loc.position.y == size.y {
                    Location {
                        depth: self.depth - 1,
                        position: Vector2::new(2, 3)
                    }
                } else {
                    loc
                }
            })
            .map(move |loc| -> Box<dyn Iterator<Item = Location>> {
                if loc.is_center(size) {
                    if self.position.x == 1 {
                        Box::new(Location::side_left_it(size, loc.depth + 1))
                    } else if self.position.x == 3 {
                        Box::new(Location::side_right_it(size, loc.depth + 1))
                    } else if self.position.y == 1 {
                        Box::new(Location::side_top_it(size, loc.depth + 1))
                    } else if self.position.y == 3 {
                        Box::new(Location::side_bottom_it(size, loc.depth + 1))
                    } else {
                        unreachable!()
                    }
                } else {
                    Box::new(once(loc))
                }
            })
            .flatten())
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
        loc
            .neighbours(self.size)
            .filter(|&loc| self.has_bug(loc))
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

fn main1() {
//    let input = read_to_string("test1.txt").unwrap();
//
//    let mut field = parse_field(&input);
//
//    if false {
//        println!("Begin:");
//        field.print();
//    }
//
//    for _i in 0..10 {
//        field = field.next();
//    }
//
//    if false {
//        println!("After 10:");
//        field.print();
//    }

    let input = read_to_string("input.txt").unwrap();

    let mut field = parse_field(&input);

    for _i in 0..200 {
        field = field.next()
    }

    println!();

    dbg!(field.num_bugs());
}

fn main() {
    for _i in 0..100 {
        main1();
    }

    let mut total: Duration = Duration::from_millis(0);

    for _i in 0..100 {
        let start = Instant::now();
        main1();
        let end = Instant::now();
        total += end - start;
        println!("Took: {}ms", (end - start).as_millis());
    }

    println!("Average: {}", total.as_millis() / 100);
}
