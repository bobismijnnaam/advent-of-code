use nalgebra::Vector2;
use std::fs::read_to_string;
use std::collections::{HashMap, BinaryHeap, HashSet};

mod util;
use util::*;
use util::direction::*;
use util::direction::Direction::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Maze(Vec<Vec<char>>);

type KeySet = HashSet<char>;

impl Maze {
    fn new(input: String) -> Maze {
        Maze(input.trim().split('\n').map(|row| row.chars().collect()).collect())
    }

    fn get(&self, pos: Vector2<i32>) -> char {
        self.0[pos.y as usize][pos.x as usize]
    }

//    fn set(&mut self, pos: Vector2<i32>, val: char) {
//        self.0[pos.y as usize][pos.x as usize] = val;
//    }

    fn is_passable(&self, pos: Vector2<i32>, keys: &KeySet) -> bool {
        match self.get(pos) {
            '.' => true,
            '@' => true,
            '#' => false,
            'a'..='z' => true,
            door @ 'A'..='Z' => keys.contains(&door.to_ascii_lowercase()),
            c => panic!("{}?", c),
        }
    }

    fn size(&self) -> Vector2<i32> {
        let field = &self.0;
        Vector2::new(field[0].len() as i32, field.len() as i32)
    }

    fn get_keys(&self) -> Vec<char> {
        (0..26)
            .map(|c| ('a' as u8 + c as u8) as char)
            .filter(|c| self.get_pos_of(*c).is_some())
            .collect()
    }

    fn get_reachable_keys(&self) -> Vec<(char, usize)> {
        let current_pos = self.get_pos_of_raw('@');
        self.get_keys().iter()
            .filter_map(|&key| {
                let key_pos = self.get_pos_of_raw(key);
                if let Some(cost) = self.a_star(current_pos, key_pos) {
                    Some((key, cost as usize))
                } else {
                    None
                }
            })
            .collect()
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

    fn get_pos_of_raw(&self, target: char) -> Vector2<i32> {
        self.get_pos_of(target).unwrap()
    }

    fn num_steps(&self, from: Vector2<i32>, to: Vector2<i32>, keys: &KeySet) -> Option<i32> {
        let mut cost: HashMap<Vector2<i32>, i32> = HashMap::new();
        let mut parent: HashMap<Vector2<i32>, Vector2<i32>> = HashMap::new();
        let mut frontier: Vec<Vector2<i32>> = vec![from];

        cost.insert(from, 0);

        while frontier.len() > 0 {
            let pos = frontier.pop().unwrap();
            let &pos_cost = cost.get(&pos).unwrap();
            for dir in &[North, East, South, West] { // TODO: Refactor to .neighbours
                let next_pos = pos + dir.to_vec();
                let next_cost = pos_cost + 1;

                if !self.is_passable(next_pos, keys) {
                    continue;
                }

                if let Some(&old_cost) = cost.get(&next_pos) {
                    // Have visited before
                    if next_cost < old_cost {
                        cost.insert(next_pos, next_cost);
                        parent.insert(next_pos, pos);

                        if next_pos != to {
                            frontier.push(next_pos);
                        }
                    }
                } else {
                    // New!
                    cost.insert(next_pos, next_cost);
                    parent.insert(next_pos, pos);

                    if next_pos != to {
                        frontier.push(next_pos);
                    }
                }
            }
        }

        if let Some(num_steps) = cost.get(&to) {
            Some(*num_steps)
        } else {
            None
        }
    }

    fn a_star(&self, from: Vector2<i32>, to: Vector2<i32>) -> Option<i32> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            initial_cost: usize,
            predicted_cost: usize,
            position: Vector2<i32>,
        }

        impl State {
            fn total_cost(&self) -> usize {
                self.initial_cost + self.predicted_cost
            }
        }

        // From std
        impl Ord for State {
            fn cmp(&self, other: &State) -> Ordering {
                // Notice that the we flip the ordering on costs.
                // In case of a tie we compare positions - this step is necessary
                // to make implementations of `PartialEq` and `Ord` consistent.
                other.total_cost().cmp(&self.total_cost())
                    .then_with(|| self.position.x.cmp(&other.position.x))
                    .then_with(|| self.position.y.cmp(&other.position.y))
            }
        }

        // `PartialOrd` needs to be implemented as well.
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &State) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap = BinaryHeap::new();
        let mut costs: HashMap<Vector2<i32>, usize> = HashMap::new();

        costs.insert(from, 0);

        heap.push(State {
            position: from,
            initial_cost: 0,
            predicted_cost: 0,
        });

        while let Some(State { initial_cost, predicted_cost, position}) = heap.pop() {
            if position == to {
                return Some(initial_cost as i32)
            }

            for dir in &[North, East, South, West] { // TODO: Refactor to .neighbours
                let next_pos = position + dir.to_vec();
                let next_cost = initial_cost + 1;

                if !self.is_passable(next_pos) {
                    continue;
                }

                if !costs.contains_key(&next_pos) || next_cost < *costs.get(&next_pos).unwrap() {
                    costs.insert(next_pos, next_cost);
                    heap.push(State {
                        position: next_pos,
                        initial_cost: next_cost,
                        predicted_cost: manhattan(next_pos, to) as usize,
                    });
                }
            }
        }

        None
    }

//    fn unlock(&self, key: char) {
//        assert!(key.is_ascii_lowercase());
//        let current_pos = self.get_pos_of_raw('@');
//        let key_pos = self.get_pos_of_raw(key);
//        let door_pos = self.get_pos_of(key.to_ascii_uppercase());
//
//        self.set(key_pos, '@');
//        if let Some(door_pos) = door_pos {
//            self.set(door_pos, '.');
//        }
//        self.set(current_pos, '.');
//    }
}

fn solve(maze: Maze, depth: i32) -> i32 {
    let reachable_keys = maze.get_reachable_keys();

    if reachable_keys.len() == 0 {
        0
    } else {
        if depth == 0 || depth == 1 || depth == 2 {
            println!("Reachable keys at depth {}: {:?}", depth, reachable_keys)
        }

        reachable_keys.iter().map(|(key, cost)| {
            let mut maze = maze.clone();
            maze.unlock(*key);
            if depth == 0 || depth == 1 || depth == 2 {
                println!("Getting key {} took {} steps at depth {}", key, cost, depth);
            }
            solve(maze, depth + 1) + (*cost as i32)
        }).min().unwrap()
    }
}

fn solve_bfs(maze: Maze) -> i32 {
    let mut frontier = vec![(maze, 0)];

    loop {
        dbg!(frontier.len());
        let min_index = frontier.iter().enumerate().min_by_key(|(i, (_, cost))| cost).unwrap().0;
        let (maze, cost) = frontier.remove(min_index);
        let reachable_keys = maze.get_reachable_keys();
        dbg!(cost);

        if reachable_keys.len() == 0 {
            return cost;
        } else {
            frontier.extend(reachable_keys.iter().map(|(key, key_cost)| {
                let mut maze = maze.clone();
                maze.unlock(*key);
                (maze, cost + *key_cost as i32)
            }));
        }
    }
}

fn main() {
    let input = read_to_string("input_test1.txt").unwrap();
    let input = read_to_string("input_test2.txt").unwrap();
    let input = read_to_string("input_test3.txt").unwrap();
    let input = read_to_string("input_test4.txt").unwrap();
    println!("{}", input);

    let maze = Maze::new(input);

    println!("{:?}", maze);
    println!("{:?}", maze.get_keys());

    println!("{:?}", maze.get_reachable_keys());

//
//    let reachable_keys = maze.get_reachable_keys();
//
//    let key_1_pos = maze.get_pos_of_raw(reachable_keys[0]);
//    let key_2_pos = maze.get_pos_of_raw(reachable_keys[1]);
//
//    dbg!(maze.num_steps(key_1_pos, key_2_pos));
//    dbg!(maze.a_star(key_1_pos, key_2_pos));
//
//    return;

    println!("{}", solve_bfs(maze));
}
