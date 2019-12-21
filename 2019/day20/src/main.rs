use nalgebra::Vector2;
use std::fs::read_to_string;
use std::collections::{HashMap, BinaryHeap};

mod util;
use util::direction::Direction::*;
use std::cmp::{Reverse, Ordering};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<char>>,
}

type PortalID = (char, char);
struct Portal {
    id: PortalID,
    from: Location,
    to: Location,
}

#[derive(Eq, PartialEq, Hash, Debug, PartialOrd, Copy, Clone)]
struct Location {
    pos: Vector2<i32>,
    depth: i32,
}

impl Location {
    fn new(pos: Vector2<i32>, depth: i32) -> Location {
        Location {
            pos, depth
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.x.cmp(&other.pos.x)
            .then(self.pos.y.cmp(&other.pos.y))
            .then(self.depth.cmp(&other.depth))
    }
}

impl Maze {
    fn new(input: String) -> Maze {
        Maze {
            grid: input
                .split('\n')
                .filter(|row| row.len() > 1)
                .map(|row| row.chars().collect())
                .collect(),
        }
    }

    fn get_portal_id(&self, pos: Vector2<i32>) -> Option<PortalID> {
        if self.get(pos) != '.' {
            return None;
        }

        for dir in &[North, East, South, West] {
            let c1 = self.get(pos + dir.to_vec());
            if c1.is_ascii_uppercase() {
                let c2 = self.get(pos + dir.to_vec() * 2);
                assert!(c2.is_ascii_uppercase());
                return match dir {
                    North | West => Some((c2, c1)),
                    East | South => Some((c1, c2))
                };
            }
        }

        None
    }

    fn get_portal_positions_raw(&self, id: PortalID) -> [Vector2<i32>; 2] {
        let mut other_pos = None;
        for y in 1..self.size().y - 1 {
            for x in 1..self.size().x - 1 {
                let portal_pos = Vector2::new(x, y);

                if let Some(other_id) = self.get_portal_id(portal_pos) {
                    if id == other_id && other_pos.is_some() {
                        return [other_pos.unwrap(), portal_pos]
                    } else if id == other_id {
                        other_pos = Some(portal_pos);
                    }
                }
            }
        }

        if let Some(other_pos) = other_pos {
            return [other_pos, other_pos]
        }

        panic!();
    }

    fn is_internal_portal(&self, pos: Vector2<i32>) -> bool {
        let size = self.size();
        if pos.x == 2 || pos.y == 2 || pos.x == size.x - 3 || pos.y == size.y - 3 {
            return false;
        } else {
            return true;
        }
    }

    fn is_external_portal(&self, pos: Vector2<i32>) -> bool {
        !self.is_internal_portal(pos)
    }

    fn get_portal_from_pos(&self, pos: Location) -> Option<Portal> {
        let id = self.get_portal_id(pos.pos)?;

        let portal_positions = self.get_portal_positions_raw(id);

        // Cannot be start or end portal
        if portal_positions[0] == portal_positions[1] {
            return None;
        }

        let other_pos = if portal_positions[0] == pos.pos {
            portal_positions[1]
        } else {
            portal_positions[0]
        };

        let next_depth = if self.is_internal_portal(pos.pos) {
            pos.depth + 1
        } else {
            pos.depth - 1
        };

        if next_depth < 0 {
            None
        } else {
            Some(Portal {
                id: id,
                from: pos,
                to: Location {
                    pos: other_pos,
                    depth: next_depth,
                }
            })
        }
    }

    fn optimize(&mut self) {
        let mut changed = true;
        let mut i = 0;
        while changed {
            changed = false;
            for y in 1..self.size().y - 1 {
                for x in 1..self.size().x - 1 {
                    let pos = Vector2::new(x, y);
                    if self.get(pos) == '.' {
                        let num_wall_neighbours = *&[North, East, South, West].iter()
                            .map(|dir| { pos + dir.to_vec() })
                            .map(|neighbour| { self.get(neighbour) })
                            .filter(|c| *c == '#')
                            .count();
                        if num_wall_neighbours >= 3 as usize {
                            // Dead end!
                            self.set(pos, '#');
                            changed = true;
                            i += 1;
                        }
                    }
                }
            }
        }
        dbg!(i);
    }

    fn pretty_print(&self) {
        for y in 0..self.size().y {
            for x in 0..self.size().x {
                print!("{}", self.get(Vector2::new(x, y)));
            }
            println!();
        }
    }

    fn get(&self, pos: Vector2<i32>) -> char {
        self.grid[pos.y as usize][pos.x as usize]
    }

    fn set(&mut self, pos: Vector2<i32>, c: char) {
        self.grid[pos.y as usize][pos.x as usize] = c;
    }

    fn is_passable(&self, pos: Vector2<i32>) -> bool {
        match self.get(pos) {
            '.' => true,
            _ => false
        }
    }

    fn size(&self) -> Vector2<i32> {
        let field = &self.grid;
        Vector2::new(field[0].len() as i32, field.len() as i32)
    }

    fn num_steps(&self, from: Location, to: Location) -> i32 {
        assert!(self.is_passable(from.pos) && self.is_passable(to.pos));

        let mut cost: HashMap<Location, i32> = HashMap::new();
        let mut frontier: BinaryHeap<Reverse<(i32, Location)>> = BinaryHeap::from(vec![Reverse((0, from))]);
        frontier.reserve(500);
        cost.reserve(500);

        cost.insert(from, 0);

        loop {
            let Reverse((pos_cost, pos)) = frontier.pop().unwrap();

            if pos == to {
                return pos_cost;
            }

            for dir in &[North, East, South, West] {
                let next_pos = Location::new(pos.pos + dir.to_vec(), pos.depth);
                let next_cost = pos_cost + 1;

                if self.is_passable(next_pos.pos) {
                    cost.entry(next_pos).and_modify(|old_cost| {
                        if next_cost < *old_cost {
                            *old_cost = next_cost;
                            frontier.push(Reverse((next_cost, next_pos)));
                        }
                    }).or_insert_with(|| {
                        frontier.push(Reverse((next_cost, next_pos)));
                        next_cost
                    });
                }
            }

            if let Some(portal) = self.get_portal_from_pos(pos) {
                let next_pos = portal.to;
                let next_cost = pos_cost + 1;
                cost.entry(next_pos).and_modify(|old_cost| {
                    if next_cost < *old_cost {
                        *old_cost = next_cost;
                        frontier.push(Reverse((next_cost, next_pos)));
                    }
                }).or_insert_with(|| {
                    frontier.push(Reverse((next_cost, next_pos)));
                    next_cost
                });
            }
        }
    }
}

fn main1() {
    let input = read_to_string("input.txt").unwrap();

    let mut maze = Maze::new(input);
    maze.optimize();

    let start_portal = maze.get_portal_positions_raw(('A', 'A'))[0];
    let end_portal = maze.get_portal_positions_raw(('Z', 'Z'))[0];

    dbg!(start_portal);
    dbg!(end_portal);

    let res = maze.num_steps(
        Location::new(start_portal, 0),
        Location::new(end_portal, 0)
    );

    if res == 500 || res == 501 {
        println!("Too low!");
    }

    dbg!(res);
    assert_eq!(res, 5648);
}

fn main() {
    let start = Instant::now();
    main1();
    let duration = start.elapsed();
    println!("Time elapsed in part 1 is: {:?}", duration);
}