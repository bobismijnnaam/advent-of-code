use nalgebra::Vector2;
use std::fs::read_to_string;
use std::collections::{HashMap, BinaryHeap, HashSet};

mod util;
use util::direction::Direction::*;
use itertools::Itertools;
use std::cmp::{Reverse, Ordering};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<char>>,
    reachable_keys_cache: HashMap<(Vector2<i32>, KeySet), Vec<RelativeKey>>,
    key_positions: [Option<Vector2<i32>>; 26],
}

type KeySet = [bool; 26];

fn key_set_new() -> KeySet {
    [false; 26]
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct RelativeKey {
    key: char,
    distance: usize,
    position: Vector2<i32>,
}

impl RelativeKey {
    fn new(key: char, distance: usize, position: Vector2<i32>) -> RelativeKey {
        RelativeKey {
            key,
            distance,
            position
        }
    }
}

impl Maze {
    fn new(input: String) -> Maze {
        let mut res = Maze {
            grid: input.trim().split('\n').map(|row| row.chars().collect()).collect(),
            reachable_keys_cache: HashMap::new(),
            key_positions: [None; 26],
        };

        for key in 0..26 {
            res.key_positions[key] = res.get_pos_of(('a' as u8 + key as u8) as char);
        }

        res
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

    fn is_passable(&self, pos: Vector2<i32>, me: &Me) -> bool {
        match self.get(pos) {
            '.' => true,
            '@' => true,
            '#' => false,
            key @ 'a'..='z' => me.has_key(key),
            door @ 'A'..='Z' => me.has_key(door.to_ascii_lowercase()),
            c => panic!("{}?", c),
        }
    }

    fn is_key(&self, pos: Vector2<i32>) -> bool {
        match self.get(pos) {
            'a' ..= 'z' => true,
            _ => false
        }
    }

    fn size(&self) -> Vector2<i32> {
        let field = &self.grid;
        Vector2::new(field[0].len() as i32, field.len() as i32)
    }

    fn get_keys(&self) -> Vec<char> {
        (0..26)
            .map(|c| ('a' as u8 + c as u8) as char)
            .filter(|c| self.get_pos_of_key(*c).is_some())
            .collect()
    }

    fn get_new_reachable_keys(&self, me: &Me) -> Vec<RelativeKey> {
        self.num_steps_to_keys(me.position, me)
    }

    fn get_new_reachable_keys_cache(&mut self, me: &Me) -> Vec<RelativeKey> {
        // Bottleneck? Also allocation
        if let Some(reachable_keys) = self.reachable_keys_cache.get(&(me.position, me.keys)) {
            reachable_keys.clone()
        } else {
            let keys = self.get_new_reachable_keys(&me);
            self.reachable_keys_cache.insert((me.position, me.keys), keys.clone());
            keys
        }
    }


    fn get_pos_of(&self, target: char) -> Option<Vector2<i32>> {
        let size = self.size();
        for y in 0..size.y {
            for x in 0..size.x {
                if self.get(Vector2::new(x, y)) == target {
                    return Some(Vector2::new(x, y));
                }
            }
        }

        None
    }

    fn get_pos_of_key(&self, target: char) -> Option<Vector2<i32>> {
        self.key_positions[(target as u8 - 'a' as u8) as usize]
    }

    fn get_positions_of_raw(&self, target: char) -> Vec<Vector2<i32>> {
        let size = self.size();
        (0..size.x)
            .cartesian_product(0..size.y)
            .map(|(x, y)| Vector2::new(x, y))
            .filter_map(|pos| if self.get(pos) == target {
                Some(pos)
            } else {
                None
            })
            .collect()
    }

    fn get_pos_of_raw(&self, target: char) -> Vector2<i32> {
        self.get_pos_of(target).unwrap()
    }

    fn num_steps_to_keys(&self, from: Vector2<i32>, me: &Me) -> Vec<RelativeKey> {
        let mut cost: HashMap<Vector2<i32>, i32> = HashMap::new();
        let mut frontier: Vec<Vector2<i32>> = vec![from];
        frontier.reserve(500);
        cost.reserve(500);

        cost.insert(from, 0);

        while frontier.len() > 0 {
            let pos = frontier.pop().unwrap();
            let &pos_cost = cost.get(&pos).unwrap();
            for dir in &[North, East, South, West] { // TODO: Refactor to .neighbours
                let next_pos = pos + dir.to_vec();
                let next_cost = pos_cost + 1;

                if !self.is_passable(next_pos, me) {
                    if self.is_key(next_pos) {
                        // Add key to cost if cost is lower
                        cost.entry(next_pos).and_modify(|old_cost| {
                            if next_cost < *old_cost {
                                *old_cost = next_cost;
                            }
                        }).or_insert(next_cost);
                    }
                    continue;
                }

                cost.entry(next_pos).and_modify(|old_cost| {
                    if next_cost < *old_cost {
                        *old_cost = next_cost;
                        frontier.push(next_pos);
                    }
                }).or_insert_with(|| {
                    frontier.push(next_pos);
                    next_cost
                });
            }
        }

        self.get_keys().iter()
            .filter(|&&key| !me.has_key(key))
            .filter_map(|&key| {
                let pos = self.get_pos_of_key(key).unwrap();
                if let Some(cost) = cost.get(&pos) {
                    Some(RelativeKey::new(key, *cost as usize, pos))
                } else {
                    None
                }
            })
            .collect()
    }

    fn explode_entrance(&mut self) {
        let starting_pos = self.get_pos_of_raw('@');
        self.set(starting_pos, '#');
        for dir in &[North, East, South, West] {
            self.set(starting_pos + dir.to_vec(), '#');
            self.set(starting_pos + dir.to_vec() + dir.left().to_vec(), '@');
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
struct Me {
    position: Vector2<i32>,
    steps_taken: i32,
    keys: KeySet,
}

impl Ord for Me {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.x.cmp(&other.position.x)
            .then(self.steps_taken.cmp(&other.steps_taken))
            .then(self.keys.cmp(&other.keys))
    }
}

impl Me {
    fn grab_key(&mut self, relative_key: RelativeKey) {
        assert!(relative_key.key.is_ascii_lowercase());
        self.position = relative_key.position;
        self.add_key(relative_key.key);
        self.steps_taken += relative_key.distance as i32;
    }

    fn add_key(&mut self, key: char) {
        self.keys[(key as u8 - 'a' as u8) as usize] = true;
    }

    fn has_key(&self, key: char) -> bool {
        self.keys[(key as u8 - 'a' as u8) as usize]
    }
}

fn solve_dp_bfs(maze: &mut Maze, mes: Vec<Me>) -> i32 {
    let mut heap = BinaryHeap::with_capacity(500000);
    let mut seen_states = HashSet::with_capacity(500000);

    heap.push(Reverse((0, mes)));
    let mut i = 0;

    loop {
        i += 1;
        if i >= 1000 {
            print!("\r{}               ", heap.len());
            i = 0;
        }

        let Reverse((_, mes)) = heap.pop().unwrap();

        let move_candidates: Vec<_> = mes.iter().positions(|me| maze.get_new_reachable_keys_cache(me).len() > 0).collect();

        if move_candidates.len() == 0 {
            return mes.iter().map(|me| me.steps_taken).sum()
        } else {
            move_candidates.iter().for_each(|&move_candidate_i| {
                let me = &mes[move_candidate_i];
                let reachable_keys = maze.get_new_reachable_keys_cache(me);
                reachable_keys.iter().for_each(|&relative_key| {
                    let mut mes = mes.clone();
                    mes[move_candidate_i].grab_key(relative_key);
                    for me in &mut mes {
                        me.add_key(relative_key.key);
                    }

                    let total_steps = mes.iter().map(|me| me.steps_taken).sum();

                    let heap_elem = Reverse((total_steps, mes));

                    if seen_states.insert(heap_elem.clone()) {
                        heap.push(heap_elem.clone());
                    }
                })
            })
        }
    }
}

fn main2() {
//    let input = read_to_string("input_2_test1.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    let mut maze = Maze::new(input);
    maze.optimize();

    maze.explode_entrance();

    let mes: Vec<_> = maze.get_positions_of_raw('@').iter().map(|start_pos| Me {
        position: *start_pos,
        steps_taken: 0,
        keys: key_set_new()
    }).collect();

//    dbg!(solve_dp_2(&mut maze, mes, &mut HashMap::new()));
    dbg!(solve_dp_bfs(&mut maze, mes));

    // 460 too low?
    // 1996
}

fn main1() {
//    let input = read_to_string("input_test1.txt").unwrap();
//    let input = read_to_string("input_test2.txt").unwrap();
//    let input = read_to_string("input_test3.txt").unwrap();
//    let input = read_to_string("input_test4.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();

    let mut maze = Maze::new(input);
    maze.optimize();
    maze.pretty_print();

    println!("{:?}", maze.get_keys());

    let me = Me {
        position: maze.get_pos_of_raw('@'),
        keys: key_set_new(),
        steps_taken: 0,
    };

    println!("Reachable: {:?}", maze.get_new_reachable_keys(&me));

//    println!("{}", solve_dp_2(&mut maze, vec![me], &mut HashMap::new()));
    println!("{}", solve_dp_bfs(&mut maze, vec![me]));

    // 5964 done
}

fn main() {
    let start = Instant::now();
    main1();
    let duration = start.elapsed();
    println!("Time elapsed in part 1 is: {:?}", duration);

    if true {
        let start = Instant::now();
        main2();
        let duration = start.elapsed();
        println!("Time elapsed in part 2 is: {:?}", duration);
    }
}