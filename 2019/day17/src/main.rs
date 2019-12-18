use std::fs::read_to_string;

mod util;
use util::intcode::*;
use nalgebra::Vector2;
use crate::util::direction::Direction;
use itertools::Itertools;
use std::io::Write;
use std::cmp::min;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Action {
    L,
    R,
    Forward(i32),
    Function(i32)
}

impl Action {
    fn is_function(&self) -> bool {
        use Action::*;
        match self {
            Function(_) => true,
            _ => false
        }
    }

    fn to_ascii(&self) -> String {
        use Action::*;
        match self {
            L => "L".to_owned(),
            R => "R".to_owned(),
            Forward(n) => n.to_string(),
            Function(n) => (('A' as u8 + *n as u8) as char).to_string()
        }
    }
}

fn find_starting_configuration(field: &Vec<Vec<char>>) -> (Vector2<i32>, Direction) {
    let height = field.len() as i32;
    let width = field[0].len() as i32;

    let pos = (0..width).cartesian_product(0..height)
        .find(|(x, y)| {
            let pos = field[*y as usize][*x as usize];
            pos != '.' && pos != '#'
        }).unwrap();
    let pos = Vector2::new(pos.0, pos.1);

    use Direction::*;
    let dir = match field[pos.y as usize][pos.x as usize] {
        '>' => East,
        'v' => South,
        '<' => West,
        '^' => North,
        _ => panic!()
    };

    (pos, dir)
}

fn field_to_scaffolds(field: &Vec<Vec<char>>) -> (Vec<Vector2<i32>>, Vector2<i32>, Direction) {
    let (start_pos, start_dir) = find_starting_configuration(field);

    let height = field.len() as i32;
    let width = field[0].len() as i32;

    let scaffolds = (0..width).cartesian_product(0..height)
        .map(|(x, y)| Vector2::new(x, y))
        .filter(|pos| field[pos.y as usize][pos.x as usize] != '.')
        .collect();

    (scaffolds, start_pos, start_dir)
}

fn reached_the_end(scaffolds: &Vec<Vector2<i32>>, pos: Vector2<i32>) -> bool {
    let mut surrounding_scaffolds = 0;
    use Direction::*;
    for dir in &[North, East, South, West] {
        let neighbour_pos = pos + dir.to_vec();
        if scaffolds.contains(&neighbour_pos) {
            surrounding_scaffolds += 1;
        }
    }

    assert!(surrounding_scaffolds > 0);
    surrounding_scaffolds == 1
}

fn calculate_walk_dist(scaffolds: &Vec<Vector2<i32>>, start_pos: Vector2<i32>, dir: Direction) -> i32 {
    let mut pos = start_pos;
    let mut count = 0;
    loop {
        let next_pos = pos + dir.to_vec();
        if scaffolds.contains(&next_pos) {
            pos = next_pos;
            count += 1;
        } else {
            break;
        }
    }

    count
}

fn calculate_route(scaffolds: &Vec<Vector2<i32>>, start_pos: Vector2<i32>, start_dir: Direction) -> Vec<Action> {
    use Direction::*;
    use Action::*;
    let mut current_dir = West;
    let mut current_pos = start_pos;
    let mut actions = vec![L];

    loop {
        let walk_dist = calculate_walk_dist(scaffolds, current_pos, current_dir);
        actions.push(Forward(walk_dist));
        current_pos += current_dir.to_vec() * walk_dist;

        if reached_the_end(&scaffolds, current_pos) {
            break;
        } else {
            // Must turn!
            let left_dir = current_dir.left();
            let right_dir = current_dir.right();
            if scaffolds.contains(&(current_pos + left_dir.to_vec())) {
                current_dir = left_dir;
                actions.push(L);
            } else if scaffolds.contains(&(current_pos + right_dir.to_vec())) {
                current_dir = right_dir;
                actions.push(R);
            } else {
                panic!();
            }
        }
    }

    actions
}

fn begins_with(elems: &[Action], sequence: &[Action]) -> bool {
    if elems.len() == 0 && sequence.len() == 0 {
        true
    } else if elems.len() == 0 && sequence.len() != 0 {
        false
    } else if elems.len() != 0 && sequence.len() == 0 {
        true
    } else if elems.len() != 0 && sequence.len() != 0 {
        elems[0] == sequence[0] && begins_with(&elems[1..], &sequence[1..])
    } else {
        panic!()
    }
}

fn find_occurs_pos(elems: &[Action], sequence: &[Action]) -> Option<usize> {
    for i in 0..elems.len() {
        let sub_elems = &elems[i..];
        if begins_with(&sub_elems, sequence) {
            return Some(i);
        }
    }

    None
}

fn longest_recurring_prefix(actions: &[Action]) -> Option<Vec<Action>> {
    let mut longest_found = None;
    for i in 1..actions.len() {
        println!("Looking for length {}", i);
        let sub_actions = &actions[0..i];
        if let Some(_) = find_occurs_pos(&actions[1..], sub_actions) {
            println!("Ok!");
            longest_found = Some(i);
        } else {
            println!("Not found. Maximum length: {:?}", longest_found);
            break;
        }
    }

    if let Some(p) = longest_found {
        Some(actions[0..longest_found.unwrap()].iter().cloned().collect())
    } else {
        None
    }
}

fn longest_recurring_subsequence(actions: &[Action]) -> Option<Vec<Action>> {
    let mut longest_found: Option<Vec<Action>> = None;
    for i in 0..actions.len() {
        let shortened_actions = &actions[i..];
        if let Some(pos) = longest_recurring_prefix(shortened_actions) {
            if longest_found.is_some() {
                if pos.len() > longest_found.as_ref().unwrap().len() {
                    longest_found = Some(pos);
                }
            } else {
                longest_found = Some(pos);
            }
        }
    }

    longest_found
}

fn replace(actions: &mut Vec<Action>, sequence: &[Action], function_action: Action) {
    if let Some(sequence_start_pos) = find_occurs_pos(&actions, sequence) {
        actions.splice(sequence_start_pos .. sequence_start_pos + sequence.len(), vec![function_action]);
    } else {
        panic!()
    }
}

fn replace_all(actions: &mut Vec<Action>, sequence: &[Action], function_action: Action) {
    while let Some(_) = find_occurs_pos(&actions, sequence) {
//        println!("{:?}", sequence);
//        println!("Before: {:?}", actions);
//        println!("Replacing...");
        replace(actions, sequence, function_action);
//        println!("After: {:?}", actions);
    }
}

fn compress(actions: Vec<Action>) -> (Vec<Action>, Vec<Vec<Action>>) {
    use Action::*;

    println!("----- Compressing ------");

    let mut function_calls = actions;
    let mut functions = vec![];
    for i in 0..3 {
        let num_funcs_prefix = function_calls.iter().take_while(|x| match x {
            Function(_) => true,
            _ => false
        }).count();
        let (function_calls_head, function_calls_tail) = function_calls.split_at(num_funcs_prefix);
        let mut function_calls_tail= function_calls_tail.to_vec();

//        let prefix = longest_recurring_prefix(&function_calls_tail).unwrap();
        let prefix = longest_recurring_subsequence(&function_calls_tail).unwrap();
        functions.push(prefix.clone());
        println!("Prefix: {:?}", prefix);
        replace_all(&mut function_calls_tail, &prefix, Function(i));
        println!("Intermediate res: {:?}", function_calls_tail);

        function_calls = function_calls_head.to_vec();
        function_calls.extend(function_calls_tail);
    }

    println!("Res: {:?}", function_calls);

    (function_calls, functions)
}

fn sublists(actions: &[Action]) -> Vec<Vec<Action>> {
    if actions.len() == 0 {
        vec![vec![]]
    } else {
        let head = actions[0];
        let mut sublists = sublists(&actions[1..]);
        let appended_sublists: Vec<_> = sublists.iter()
            .map(|mut sublist| {
                let mut appended_sublist = sublist.clone();
                appended_sublist.insert(0, head);
                appended_sublist
            })
            .collect();
        sublists.extend(appended_sublists);
        sublists
    }
}

fn compress_specific_sequence(actions: &[Action], sequence: &[Action]) -> Vec<Action> {
    panic!()
}

fn compress2(actions: &[Action], next_func_id: i32) -> Option<Vec<Vec<Action>>> {
    use Action::*;

    let next_function_use_index = actions.iter().position(|a| a.is_function()).unwrap_or(999);

//    for i in 1..*[20, actions.len(), next_function_use_index].iter().min().unwrap() {
    for i in 1..20 {
        println!("Checking prefix {} at depth {}", i, next_func_id);
        let (prefix, tail) = actions.split_at(i);

        let mut candidate_actions = tail.to_vec();
        replace_all(&mut candidate_actions, prefix, Function(next_func_id));

        // Remove repeated functions at the prefix
        let num_funcs_prefix = candidate_actions.iter()
            .take_while(|x| x.is_function())
            .count();
        let candidate_actions_without_prefix = &candidate_actions[num_funcs_prefix..];

        if next_func_id == 0 {
            println!("Original: {:?}", actions);
            println!("Prefix: {:?}", prefix);
            println!("Candidate postfix: {:?}", candidate_actions_without_prefix);
        }

        if next_func_id == 2 {
            // Must be finished compressing!
            if candidate_actions_without_prefix.iter().filter(|f| !f.is_function()).count() > 0 {
                // Compressin failed!
                println!("Still non-functions left");
                continue;
            } else {
                // Succes!
                return Some(vec![prefix.to_vec()]);
            }
        } else {
            // Compress more
            if let Some(mut functions) = compress2(&candidate_actions_without_prefix, next_func_id + 1) {
                // Successfully compressed!
                functions.insert(0, prefix.to_vec());
                return Some(functions);
            } else {
                // Compression failed, goto next
                continue;
            }
        }
    }

    None
}

fn main() {
    // Part 1
    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.start();

    let mut output: Vec<char> = cpu.output.into_iter()
        .map(|c| std::char::from_u32(c as u32).unwrap())
        .collect();

    for a_char in &output {
        print!("{}", a_char);
    }

    output.remove(output.len() - 1);
    output.remove(output.len() - 1);

    let mut field: Vec<Vec<char>> = output.split(|c| *c == '\n').map(|cs| cs.to_vec()).collect();

    let height: i32 = field.len() as i32;
    let width: i32 = field[0].len() as i32;

    let get = |x: i32, y: i32| {
        field[y as usize][x as usize]
    };

    let mut count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if get(x, y) == '#' {
                if get(x - 1, y) == '#' && get(x + 1, y) == '#' && get(x, y - 1) == '#' && get(x, y + 1) == '#' {
                    count += x * y;
                }
            }
        }
    }
    dbg!(count);

    let (scaffolds, start_pos, start_dir) = field_to_scaffolds(&field);

    let route = calculate_route(&scaffolds, start_pos, start_dir);

    dbg!(route.len());

    let functions = compress2(&route, 0).unwrap();

    let mut route = route;

    for (i, function) in functions.iter().enumerate() {
        replace_all(&mut route, function, Action::Function(i as i32));
    }

    dbg!(&route);

    let route = route;

    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.set_memory(0, 2);
    cpu.print_output = true;

    let mut first = true;
    for function in route {
        if first {
            first = false;
        } else {
            cpu.input.push(',' as i64);
        }
        cpu.input.push(function.to_ascii().chars().next().unwrap() as i64)
    }

    cpu.input.push('\n' as i64);

    dbg!(&cpu.input);

    for function_def in functions {
        let mut first = true;
        for action in function_def {
            if first {
                first = false;
            } else {
                cpu.input.push(',' as i64);
            }
            for char in action.to_ascii().chars() {
                cpu.input.push(char as i64);
            }
        }
        cpu.input.push('\n' as i64);

        dbg!(&cpu.input);
    }

    cpu.input.push('n' as i64);
    cpu.input.push('\n' as i64);

    println!("Starting...");

    cpu.start();
}
