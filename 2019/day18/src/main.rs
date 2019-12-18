use nalgebra::Vector2;
use std::fs::read_to_string;
use std::collections::{HashMap, BinaryHeap, HashSet};

mod util;
use util::*;
use util::direction::*;
use util::direction::Direction::*;
use std::cmp::Ordering;
use std::mem::swap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Maze {
    grid: Vec<Vec<char>>,
    reachable_keys_cache: HashMap<(Vector2<i32>, KeySet), Vec<(char, usize)>>,
}

type KeySet = [bool; 26];

fn KeySet_new() -> KeySet {
    [false; 26]
}

impl Maze {
    fn new(input: String) -> Maze {
        Maze {
            grid: input.trim().split('\n').map(|row| row.chars().collect()).collect(),
            reachable_keys_cache: HashMap::new(),
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

    fn size(&self) -> Vector2<i32> {
        let field = &self.grid;
        Vector2::new(field[0].len() as i32, field.len() as i32)
    }

    fn get_keys(&self) -> Vec<char> {
        (0..26)
            .map(|c| ('a' as u8 + c as u8) as char)
            .filter(|c| self.get_pos_of(*c).is_some())
            .collect()
    }

    fn get_new_reachable_keys(&self, me: &Me) -> Vec<(char, usize)> {
        self.get_keys().iter()
            .filter(|&&key| !me.has_key(key))
            .filter_map(|&key| {
                let key_pos = self.get_pos_of_raw(key);
                if let Some(cost) = self.num_steps(me.position, key_pos, &me) {
                    Some((key, cost as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_new_reachable_keys_cache(&mut self, me: &Me) -> Vec<(char, usize)> {
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

    fn num_steps(&self, from: Vector2<i32>, to: Vector2<i32>, me: &Me) -> Option<i32> {
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

                if !(self.is_passable(next_pos, me) || next_pos == to) {
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

    fn explode_entrance(&mut self) {
        let starting_pos = self.get_pos_of_raw('@');
        self.set(starting_pos, '#');
        for dir in &[North, East, South, West] {
            self.set(starting_pos + dir.to_vec(), '#');
            self.set(starting_pos + dir.to_vec() + dir.left().to_vec(), '@');
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq,)]
struct Me {
    position: Vector2<i32>,
    steps_taken: i32,
    keys: KeySet,
}

impl Me {
    fn grab_key(&mut self, maze: &Maze, key: char, key_distance: i32) {
        assert!(key.is_ascii_lowercase());
        let key_pos = maze.get_pos_of_raw(key);

        self.position = key_pos;
        self.add_key(key);
        self.steps_taken += key_distance;
    }

    fn add_key(&mut self, key: char) {
        self.keys[(key as u8 - 'a' as u8) as usize] = true;
    }

    fn has_key(&self, key: char) -> bool {
        self.keys[(key as u8 - 'a' as u8) as usize]
    }

    fn has_all_keys(&self, maze: &Maze) -> bool {
        maze.get_keys().iter().all(|key| self.has_key(*key))
    }

    fn merge(&self, mes: &Vec<Me>) -> Me {
        let mut me = self.clone();
        for other_me in mes {
            for i in 0..26 {
                me.keys[0] = me.keys[0] || other_me.keys[0];
            }
        }
        me
    }
}

fn solve_dp(maze: &Maze, me: Me, results_cache: &mut HashMap<Me, i32>, reachable_keys_cache: &mut HashMap<(Vector2<i32>, KeySet), Vec<(char, usize)>>) -> i32 {
    let reachable_keys: Vec<_> = if let Some(reachable_keys) = reachable_keys_cache.get(&(me.position, me.keys)) {
        reachable_keys.clone()
    } else {
        let k = maze.get_new_reachable_keys(&me);
        reachable_keys_cache.insert((me.position, me.keys), k.clone());
        k
    };

    if reachable_keys.len() == 0 {
        me.steps_taken
    } else {
        reachable_keys.iter().map(|(key, extra_steps_needed)| {
            let mut me = me.clone();
            me.grab_key(maze, *key, *extra_steps_needed as i32);

            if let Some(&entry) = results_cache.get(&me) {
                entry
            } else {
                let res = solve_dp(maze, me.clone(), results_cache, reachable_keys_cache);
                if !results_cache.contains_key(&me) {
                    results_cache.insert(me, res);
                }
                res
            }
        }).min().unwrap()
    }
}

fn solve_dp_2(maze: &mut Maze, mes: Vec<Me>, results_cache: &mut HashMap<Vec<Me>, i32>) -> i32 {
    if let Some(res) = results_cache.get(&mes) {
        *res
    } else {
        let move_candidates: Vec<_> = mes.iter().positions(|me| maze.get_new_reachable_keys_cache(me).len() > 0).collect();

        let res = if move_candidates.len() == 0 {
            mes.iter().map(|me| me.steps_taken).sum()
        } else {
            move_candidates.iter().map(|&move_candidate_i| {
                let me = &mes[move_candidate_i];
                let reachable_keys = maze.get_new_reachable_keys_cache(me);
                reachable_keys.iter().map(|(key, extra_steps_needed)| {
                    let mut mes = mes.clone();
                    mes[move_candidate_i].grab_key(maze, *key, *extra_steps_needed as i32);
                    for me in &mut mes {
                        me.add_key(*key);
                    }

                    solve_dp_2(maze, mes, results_cache)

                }).min().unwrap()
            }).min().unwrap()
        };

        results_cache.insert(mes, res);

        res
    }
}

fn main() {
//    let input = read_to_string("input_2_test1.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();
    println!("{}", input);

    let mut maze = Maze::new(input);

    maze.explode_entrance();

    let mes: Vec<_> = maze.get_positions_of_raw('@').iter().map(|start_pos| Me {
        position: *start_pos,
        steps_taken: 0,
        keys: KeySet_new()
    }).collect();

    dbg!(solve_dp_2(&mut maze, mes, &mut HashMap::new()));

    // 460 too low?
    // 1996
}

fn main1() {
    let input = read_to_string("input_test1.txt").unwrap();
    let input = read_to_string("input_test2.txt").unwrap();
    let input = read_to_string("input_test3.txt").unwrap();
    let input = read_to_string("input_test4.txt").unwrap();
    let input = read_to_string("input.txt").unwrap();
    println!("{}", input);

    let maze = Maze::new(input);

    println!("{:?}", maze);
    println!("{:?}", maze.get_keys());

    let mut me = Me {
        position: maze.get_pos_of_raw('@'),
        keys: [false; 26],
        steps_taken: 0,
    };

    println!("Reachable: {:?}", maze.get_new_reachable_keys(&me));

    println!("{}", solve_dp(&maze, me, &mut HashMap::new(), &mut HashMap::new()));

    // 5964 done
}
