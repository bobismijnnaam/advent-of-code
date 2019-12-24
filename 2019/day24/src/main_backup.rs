use std::fs::read_to_string;
use std::collections::HashSet;
use nalgebra::Vector2;

mod util;
use util::direction::*;
use util::direction::Direction::*;
use std::hash::{Hash, Hasher};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
struct MyHashSet(HashSet<Vector2<i32>>);

impl Hash for MyHashSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().sorted_by_key(|pos| {
            pos.x.cmp(&pos.y)
        }).for_each(|pos| {
            pos.x.hash(state);
            pos.y.hash(state);
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Field {
    bugs: MyHashSet,
    size: Vector2<i32>,
}

fn parse_field(input: &str) -> Field {
    let rows: Vec<Vec<_>> = input.trim().split("\n").map(|row| row.chars().collect()).collect();

    let mut bugs: HashSet<Vector2<i32>> = HashSet::new();

    for y in 0..rows.len() {
        for x in 0..rows[0].len() {
            if rows[y][x] == '#' {
                bugs.insert(Vector2::new(x as i32, y as i32));
            }
        }
    }

    Field {
        bugs: MyHashSet(bugs),
        size: Vector2::new(rows[0].len() as i32, rows.len() as i32)
    }
}

impl Field {
    fn has_bug(&self, pos: Vector2<i32>) -> bool {
        self.bugs.0.contains(&pos)
    }

    fn neighbour_bugs(&self, pos: Vector2<i32>) -> i32 {
        (&[North, East, South, West])
            .iter()
            .map(|dir| pos + dir.to_vec())
            .filter(|neighbour_pos| self.has_bug(*neighbour_pos))
            .count() as i32
    }

    fn next(&self) -> Field {
        let mut bugs = HashSet::new();

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2::new(x, y);
                if self.has_bug(pos) {
                    // Is live bug
                    if self.neighbour_bugs(pos) == 1 {
                        bugs.insert(pos);
                    }
                } else {
                    // Is empty space
                    let neighbour_bugs = self.neighbour_bugs(pos);
                    if neighbour_bugs == 1 || neighbour_bugs == 2 {
                        bugs.insert(pos);
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
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if self.has_bug(Vector2::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn biodiversity(&self) -> i32 {
        let mut p = 1;
        let mut total = 0;

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if self.has_bug(Vector2::new(x, y)) {
                    total += p;
                }
                p *= 2;
            }
        }

        total
    }
}
fn main() {
    let input = read_to_string("test1.txt").unwrap();

    let mut field = parse_field(&input);

    field.print();

    println!();
    field = field.next();
    field.print();

    println!();
    field = field.next();
    field.print();

    println!();
    field = field.next();
    field.print();

    println!();
    field = field.next();
    field.print();

    let input = read_to_string("input.txt").unwrap();

    let mut field = parse_field(&input);

    let mut seen_fields = HashSet::new();

    while !seen_fields.contains(&field) {
        seen_fields.insert(field.clone());
        field = field.next();
    }

    println!();

    field.print();

    dbg!(field.biodiversity());

    // 30933291 too low
}
