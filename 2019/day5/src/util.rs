#![allow(dead_code)]

pub type Vec2 = (i64, i64);

pub fn add(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
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

fn get_parameter_modes(mut instruction: i64) -> Vec<i64> {
    instruction = (instruction - get_opcode(instruction)) / 100;
    let mut parameter_modes = vec![];

    while instruction > 0 {
        parameter_modes.push(instruction % 10);
        instruction = instruction / 10;
    }

    // Append extra zeroes to be safe
    while parameter_modes.len() < 10 {
        parameter_modes.push(0);
    }

    parameter_modes
}

fn get_opcode(instruction: i64) -> i64 {
    instruction % 100
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
        let instruction = program[ip];

        let parameter_modes = get_parameter_modes(instruction);
        let opcode = get_opcode(instruction);

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

