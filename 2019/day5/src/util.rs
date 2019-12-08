#![allow(dead_code)]

use std::fs;
use std::ops;

pub struct Vec2(pub i64, pub i64);

impl Vec2 {
    pub fn add(&self, b: Vec2) -> Vec2 {
        Vec2(a.0 + b.0, a.1 + b.1)
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2  {
        self.add(rhs);
    }
}

#[cfg(test)]
mod vec2_tests {
    use super::*;

    /////////////////
    // Day 2 tests //
    /////////////////

    #[test]
    fn test_add() {
        assert_eq!((0, 0) + (1, 1), (1, 1))
    }
}

pub fn manhattan(a: Vec2, b: Vec2) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/////////////////
// Intcode CPU //
/////////////////

// TODO: Copy tests from day 5 and other days to make a test routine

static OP_HALT: i64 = 99;

// 3 params
static OP_ADD: i64 = 1;
static OP_MUL: i64 = 2;

// 1 param
static OP_INPUT: i64 = 3;
static OP_OUTPUT: i64 = 4;

// 2 params
static OP_JUMP_IF_TRUE: i64 = 5;
static OP_JUMP_IF_FALSE: i64 = 6;

// 3 params
static OP_LESS_THAN: i64 = 7;
static OP_EQUALS: i64 = 8;

static POSITION_MODE: i64 = 0;
static IMMEDIATE_MODE: i64 = 1;

fn get_parameter_modes(parameterized_op: i64) -> Vec<i64> {
    let mut parameters = (parameterized_op - get_opcode(parameterized_op)) / 100;
    let mut parameter_modes = Vec::with_capacity(10);

    while parameters > 0 {
        parameter_modes.push(parameters % 10);
        parameters = parameters / 10;
    }

    // Append extra zeroes to be safe
    while parameter_modes.len() < 10 {
        parameter_modes.push(0);
    }

    parameter_modes
}

fn get_opcode(parameterized_op: i64) -> i64 {
    parameterized_op % 100
}

fn get_parameter(parameter_mode: i64, argument: i64, program: &Vec<i64>) -> i64 {
    if parameter_mode == POSITION_MODE {
        program[argument as usize]
    } else if parameter_mode == IMMEDIATE_MODE {
        argument
    } else {
        panic!("Unknown mode {}", parameter_mode)
    }
}

pub fn run_intcode_cpu(mut input: Vec<i64>, mut program: Vec<i64>, print_output: bool) -> Option<(Vec<i64>, Vec<i64>)> {
    let mut ip = 0;
    let mut output = vec![];
    while ip < program.len() {
        let parameterized_op = program[ip];

        let parameter_modes = get_parameter_modes(parameterized_op);
        let opcode = get_opcode(parameterized_op);

        if opcode == OP_HALT {
            break;
        } else if opcode == OP_ADD || opcode == OP_MUL || opcode == OP_LESS_THAN || opcode == OP_EQUALS {
            let arg1 = get_parameter(parameter_modes[0], program[ip+1], &program);
            let arg2 = get_parameter(parameter_modes[1], program[ip+2], &program);

            // Parameters that an instruction writes to will never be in immediate mode
            let dest = program[ip+3];

            let result = if opcode == OP_ADD {
                arg1 + arg2
            } else if opcode == OP_MUL {
                arg1 * arg2
            } else if opcode == OP_LESS_THAN {
                if arg1 < arg2 { 1 } else { 0 }
            } else if opcode == OP_EQUALS {
                if arg1 == arg2 { 1 } else { 0 }
            } else {
                panic!("Unknown logical opcode {}", opcode)
            };

            program[dest as usize] = result;

            ip += 4;
        } else if opcode == OP_INPUT {
            let dest = program[ip+1];
            let input = input.remove(0);
            program[dest as usize] = input;

            ip += 2;
        } else if opcode == OP_OUTPUT {
            let arg = get_parameter(parameter_modes[0], program[ip+1], &program);
            if print_output { println!("{}", arg) }
            output.push(arg);

            ip += 2;
        } else if opcode == OP_JUMP_IF_TRUE {
            let arg = get_parameter(parameter_modes[0], program[ip+1], &program);
            let jump_target = get_parameter(parameter_modes[1], program[ip+2], &program);
            if arg > 0 {
                ip = jump_target as usize;
            } else {
                ip += 3;
            }
        } else if opcode == OP_JUMP_IF_FALSE {
            let arg = get_parameter(parameter_modes[0], program[ip+1], &program);
            let jump_target = get_parameter(parameter_modes[1], program[ip+2], &program);
            if arg == 0 {
                ip = jump_target as usize;
            } else {
                ip += 3;
            }
        } else {
            panic!("Unknown opcode {}", opcode);
        }
    }

    Some((output, program))
}

pub fn program_from_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename).unwrap().split(",").map(|num_str| num_str.trim().parse().unwrap()).collect()
}

#[cfg(test)]
mod intcode_tests {
   use super::*;

    /////////////////
    // Day 2 tests //
    /////////////////

    #[test]
    fn test_add_1() {
        let program = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[0], 3500);
    }

    #[test]
    fn test_add_2() {
        let program = vec![1,0,0,0,99];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[0], 2);
    }

    #[test]
    fn test_add_3() {
        let program = vec![2,3,0,3,99];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[3], 6);
    }

    #[test]
    fn test_add_4() {
        let program = vec![2,4,4,5,99,0];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[5], 9801);
    }

    #[test]
    fn test_add_5() {
        let program = vec![1,1,1,4,99,5,6,0,99];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[0], 30);
        assert_eq!(program[4], 2);
    }

    /////////////////
    // Day 5 tests //
    /////////////////

    #[test]
    fn test_parameter_modes() {
        let program = vec![1002,4,3,4,33];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[4], 99);

        let program = vec![1101,100,-1,4,0];
        let (_, program) = run_intcode_cpu(vec![], program, false).unwrap();
        assert_eq!(program[4], 99);
    }

    #[test]
    fn test_branches() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8]; // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![7], program, false).unwrap();
        assert_eq!(output[0], 0);

        let program = vec![3,9,7,9,10,9,4,9,99,-1,8]; // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 0);
        let (output, _) = run_intcode_cpu(vec![7], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![9], program, false).unwrap();
        assert_eq!(output[0], 0);

        let program = vec![3,3,1108,-1,8,3,4,3,99]; // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![7], program, false).unwrap();
        assert_eq!(output[0], 0);

        let program = vec![3,3,1107,-1,8,3,4,3,99]; // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 0);
        let (output, _) = run_intcode_cpu(vec![7], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![9], program, false).unwrap();
        assert_eq!(output[0], 0);

        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]; // (using position mode)
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![1], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![0], program, false).unwrap();
        assert_eq!(output[0], 0);

        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]; // (using immediate mode)
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![1], program.clone(), false).unwrap();
        assert_eq!(output[0], 1);
        let (output, _) = run_intcode_cpu(vec![0], program, false).unwrap();
        assert_eq!(output[0], 0);

        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let (output, _) = run_intcode_cpu(vec![7], program.clone(), false).unwrap();
        assert_eq!(output[0], 999);
        let (output, _) = run_intcode_cpu(vec![8], program.clone(), false).unwrap();
        assert_eq!(output[0], 1000);
        let (output, _) = run_intcode_cpu(vec![9], program, false).unwrap();
        assert_eq!(output[0], 1001);
    }

    #[test]
    fn test_day5_input() {
        // Part 1
        let (output, _) = run_intcode_cpu(
            vec![1],
            program_from_file("src/inputs/day5_input.txt"),
            true)
                .unwrap();

        assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 9654885]);

        // part 2
        let (output, _) = run_intcode_cpu(
            vec![5],
            program_from_file("src/inputs/day5_input.txt"),
            true)
            .unwrap();

        assert_eq!(output, vec![7079459])
    }
}
