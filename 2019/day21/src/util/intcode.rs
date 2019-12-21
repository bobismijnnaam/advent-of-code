use std::fs;
use std::collections::HashMap;

// TODO: Add other day9 tests

static OP_HALT: i64 = 99;

// 3 params
static OP_ADD: i64 = 1;
static OP_MUL: i64 = 2;

// 1 param
static OP_INPUT: i64 = 3;
static OP_OUTPUT: i64 = 4;
static OP_ADJUST_RELATIVE_BASE: i64 = 9;

// 2 params
static OP_JUMP_IF_TRUE: i64 = 5;
static OP_JUMP_IF_FALSE: i64 = 6;

// 3 params
static OP_LESS_THAN: i64 = 7;
static OP_EQUALS: i64 = 8;

static POSITION_MODE: i64 = 0;
static IMMEDIATE_MODE: i64 = 1;
static RELATIVE_MODE: i64 = 2;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Pointer {
    Absolute(i64),
    Relative(i64)
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Param {
    Immediate(i64),
    Indirect(Pointer),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Instruction {
    Halt,

    Add(Param, Param, Pointer),
    Mul(Param, Param, Pointer),

    Input(Pointer),
    Output(Param),
    AdjustBase(Param),

    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),

    LessThan(Param, Param, Pointer),
    Equals(Param, Param, Pointer)
}

fn decode_pointer(parameter_mode: i64, value: i64) -> Pointer {
    if parameter_mode == POSITION_MODE {
        Pointer::Absolute(value)
    } else if parameter_mode == RELATIVE_MODE {
        Pointer::Relative(value)
    } else {
        panic!();
    }
}

fn decode_param(parameter_mode: i64, value: i64) -> Param {
    if parameter_mode == POSITION_MODE || parameter_mode == RELATIVE_MODE {
        Param::Indirect(decode_pointer(parameter_mode, value))
    } else if parameter_mode == IMMEDIATE_MODE {
        Param::Immediate(value)
    } else {
        panic!();
    }
}

fn decode(program: &[i64]) -> Option<(Instruction, usize)> {
    assert!(program.len() > 0);

    let parameterized_op = program[0];

    let parameter_modes = get_parameter_modes(parameterized_op);
    let opcode = get_opcode(parameterized_op);

    if opcode == OP_HALT {
        Some((Instruction::Halt, 1))
    } else if opcode == OP_ADD || opcode == OP_MUL || opcode == OP_LESS_THAN || opcode == OP_EQUALS {
        let param1 = decode_param(parameter_modes[0], program[1]);
        let param2 = decode_param(parameter_modes[1], program[2]);
        let pointer = decode_pointer(parameter_modes[2], program[3]);

        let instruction = if opcode == OP_ADD {
            Instruction::Add(param1, param2, pointer)
        } else if opcode == OP_MUL {
            Instruction::Mul(param1, param2, pointer)
        } else if opcode == OP_LESS_THAN {
            Instruction::LessThan(param1, param2, pointer)
        } else if opcode == OP_EQUALS {
            Instruction::Equals(param1, param2, pointer)
        } else {
            panic!("Unexpected opcode {}", opcode);
        };

        Some((instruction, 4))
    } else if opcode == OP_INPUT {
        Some((Instruction::Input(decode_pointer(parameter_modes[0], program[1])), 2))
    } else if opcode == OP_OUTPUT {
        Some((Instruction::Output(decode_param(parameter_modes[0], program[1])), 2))
    } else if opcode == OP_JUMP_IF_TRUE || opcode == OP_JUMP_IF_FALSE {
        let test_param = decode_param(parameter_modes[0], program[1]);
        let target_param = decode_param(parameter_modes[1], program[2]);
        let instr = if opcode == OP_JUMP_IF_TRUE  {
            Instruction::JumpIfTrue(test_param, target_param)
        } else if opcode == OP_JUMP_IF_FALSE {
            Instruction::JumpIfFalse(test_param, target_param)
        } else {
            panic!("Unexpected opcode {}", opcode)
        };
        Some((instr, 3))
    } else if opcode == OP_ADJUST_RELATIVE_BASE {
        let param = decode_param(parameter_modes[0], program[1]);
        Some((Instruction::AdjustBase(param), 2))
    } else {
        panic!("Unknown opcode {}", opcode);
    }
}

fn get_parameter_modes(parameterized_op: i64) -> Vec<i64> {
    let mut parameters = (parameterized_op - get_opcode(parameterized_op)) / 100;
    let mut parameter_modes = Vec::with_capacity(10);

    while parameters > 0 {
        let parameter = parameters % 10;
        assert!(parameter == 0 || parameter == 1 || parameter == 2);
        parameter_modes.push(parameter);
        parameters = parameters / 10;
    }

    // Append extra zeroes to be safe
    // Max 3 params
    while parameter_modes.len() < 3 {
        parameter_modes.push(0);
    }

    parameter_modes
}

fn get_opcode(parameterized_op: i64) -> i64 {
    parameterized_op % 100
}

#[derive(Clone)]
pub struct IntcodeCPU {
    pub program: HashMap<usize, i64>,
    pub ip: usize,
    pub rbp: usize,
    pub input: Vec<i64>,
    pub output: Vec<i64>,
    pub state: IntcodeState,
    pub output_mode: OutputMode,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum IntcodeState {
    Running,
    WaitingForInput,
    Halt
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutputMode {
    Disabled,
    Integer,
    ASCII
}

impl IntcodeCPU {
    pub fn new(program: Vec<i64>) -> IntcodeCPU {
        IntcodeCPU {
            program: program.into_iter().enumerate().collect(),
            ip: 0,
            rbp: 0,
            input: vec![],
            output: vec![],
            state: IntcodeState::Running,
            output_mode: OutputMode::Disabled,
        }
    }

    fn next_4_ints(&self) -> [i64; 4] {
        let mut ints = [0; 4];
        for i in 0..4 {
            ints[i] = self.get_memory(self.ip + i);
        }
        ints
    }

    fn resolve_param(&self, param: Param) -> i64 {
        match param {
            Param::Indirect(ptr) => self.dereference_pointer(ptr),
            Param::Immediate(val) => val
        }
    }

    fn dereference_pointer(&self, ptr: Pointer) -> i64 {
        match ptr {
            Pointer::Relative(offset) => self.get_memory((self.rbp as i64 + offset) as usize),
            Pointer::Absolute(raw_ptr) => self.get_memory(raw_ptr as usize)
        }
    }

    fn write_to_pointer(&mut self, p: Pointer, val: i64) {
        match p {
            Pointer::Relative(offset) => self.set_memory((self.rbp as i64 + offset) as usize, val),
            Pointer::Absolute(raw_ptr) => self.set_memory(raw_ptr as usize, val)
        }
    }

    fn cycle(&mut self) {
        let (instr, instruction_byte_size) = decode(&self.next_4_ints()).unwrap();
        let mut next_ip = self.ip + instruction_byte_size;

//        println!("-----");
//        dbg!(self.ip);
//        dbg!(instr);

        use Instruction::*;

        match instr {
            Halt => {
                self.state = IntcodeState::Halt;
            },

            Add(a, b, dest) => {
                self.write_to_pointer(dest, self.resolve_param(a) + self.resolve_param(b));
            },
            Mul(a, b, dest) => {
                self.write_to_pointer(dest, self.resolve_param(a) * self.resolve_param(b));
            }
            LessThan(a, b, dest) => {
                self.write_to_pointer(dest, if self.resolve_param(a) < self.resolve_param(b) { 1 }  else { 0 });
            }
            Equals(a, b, dest) => {
                self.write_to_pointer(dest, if self.resolve_param(a) == self.resolve_param(b) { 1 }  else { 0 });
            }

            Input(dest) => {
                if self.input.len() > 0 {
                    let value = self.input.remove(0);
                    self.write_to_pointer(dest, value);
                } else {
                    self.state = IntcodeState::WaitingForInput;
                    next_ip = self.ip;
                }
            },
            Output(param) => {
                let value = self.resolve_param(param);
                use OutputMode::*;
                match self.output_mode {
                    Disabled => (),
                    Integer => println!("{}", value),
                    ASCII => if (0i64..=255i64).contains(&value) {
                        print!("{}", value as u8 as char)
                    } else {
                        println!("{}", value)
                    }
                }
                self.output.push(value);
            },

            AdjustBase(param) => {
                let value = self.resolve_param(param);
                self.rbp = (self.rbp as i64 + value) as usize;
            },

            JumpIfTrue(test_param, target_param) => {
                let test_value = self.resolve_param(test_param);
                let target_value = self.resolve_param(target_param);
                if test_value > 0 {
                    next_ip = target_value as usize;
                }
            },
            JumpIfFalse(test_param, target_param) => {
                let test_value = self.resolve_param(test_param);
                let target_value = self.resolve_param(target_param);
                if test_value == 0 {
                    next_ip = target_value as usize;
                }
            },
        }

        self.ip = next_ip;
    }

    pub fn start(&mut self) {
        assert_eq!(self.state, IntcodeState::Running);
        while self.state == IntcodeState::Running {
            self.cycle();
        }
    }

    pub fn force_start(&mut self) {
        self.state = IntcodeState::Running;
        self.start();
    }

    pub fn resume(&mut self, input: i64) {
        assert!(self.state == IntcodeState::WaitingForInput || self.state == IntcodeState::Running);
        self.input.push(input);
        self.state = IntcodeState::Running;
        self.start();
    }

    fn get_memory(&self, position: usize) -> i64 {
        if self.program.contains_key(&(position as usize)) {
            self.program[&position]
        } else {
            0
        }
    }

    pub fn set_memory(&mut self, position: usize, val: i64) {
        self.program.insert(position, val);
    }

    pub fn char_input(&mut self, input: char) {
        self.input.push(input as i64);
    }

    pub fn str_input(&mut self, input: &str) {
        for c in input.chars() {
            self.char_input(c);
        }
    }
}

pub fn run_intcode_cpu(input: Vec<i64>, program: Vec<i64>, print_output: bool) -> IntcodeCPU {
    let mut cpu = IntcodeCPU::new(program);
    cpu.output_mode = if print_output { OutputMode::Integer } else { OutputMode::Disabled };
    cpu.input = input;

    cpu.start();

    cpu
}

pub fn program_from_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename).unwrap().trim().split(",").map(|num_str| num_str.trim().parse().unwrap()).collect()
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
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(0), 3500);
    }

    #[test]
    fn test_add_2() {
        let program = vec![1,0,0,0,99];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(0), 2);
    }

    #[test]
    fn test_add_3() {
        let program = vec![2,3,0,3,99];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(3), 6);
    }

    #[test]
    fn test_add_4() {
        let program = vec![2,4,4,5,99,0];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(5), 9801);
    }

    #[test]
    fn test_add_5() {
        let program = vec![1,1,1,4,99,5,6,0,99];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(0), 30);
        assert_eq!(program.get_memory(4), 2);
    }

    /////////////////
    // Day 5 tests //
    /////////////////

    #[test]
    fn test_parameter_modes() {
        let program = vec![1002,4,3,4,33];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(4), 99);

        let program = vec![1101,100,-1,4,0];
        let program = run_intcode_cpu(vec![], program, false);
        assert_eq!(program.get_memory(4), 99);
    }

    #[test]
    fn test_branches() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8]; // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![7], program, false).output;
        assert_eq!(output[0], 0);

        let program = vec![3,9,7,9,10,9,4,9,99,-1,8]; // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 0);
        let output = run_intcode_cpu(vec![7], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![9], program, false).output;
        assert_eq!(output[0], 0);

        let program = vec![3,3,1108,-1,8,3,4,3,99]; // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![7], program, false).output;
        assert_eq!(output[0], 0);

        let program = vec![3,3,1107,-1,8,3,4,3,99]; // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 0);
        let output = run_intcode_cpu(vec![7], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![9], program, false).output;
        assert_eq!(output[0], 0);

        // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
        let program = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9]; // (using position mode)
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![1], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![0], program, false).output;
        assert_eq!(output[0], 0);

        let program = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1]; // (using immediate mode)
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![1], program.clone(), false).output;
        assert_eq!(output[0], 1);
        let output = run_intcode_cpu(vec![0], program, false).output;
        assert_eq!(output[0], 0);

        let program = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let output = run_intcode_cpu(vec![7], program.clone(), false).output;
        assert_eq!(output[0], 999);
        let output = run_intcode_cpu(vec![8], program.clone(), false).output;
        assert_eq!(output[0], 1000);
        let output = run_intcode_cpu(vec![9], program, false).output;
        assert_eq!(output[0], 1001);
    }

    #[test]
    fn test_day5_input() {
        // Part 1
        let output = run_intcode_cpu(
            vec![1],
            program_from_file("src/util/inputs/day5_input.txt"),
            true)
            .output;

        assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 9654885]);

        // part 2
        let output = run_intcode_cpu(
            vec![5],
            program_from_file("src/util/inputs/day5_input.txt"),
            true)
            .output;

        assert_eq!(output, vec![7079459])
    }

    #[test]
    fn test_day7_input() {
        let program = program_from_file("src/util/inputs/day7_input.txt");
        let phase_inputs = vec![6, 9, 5, 8, 7];

        let mut amplifiers = [
            IntcodeCPU::new(program.clone()),
            IntcodeCPU::new(program.clone()),
            IntcodeCPU::new(program.clone()),
            IntcodeCPU::new(program.clone()),
            IntcodeCPU::new(program.clone()),
        ];

        let mut previous_output = 0;
        let mut first_time = true;
        while amplifiers[0].state != IntcodeState::Halt {
            let mut i = 0;
            for amplifier in &mut amplifiers {
                if first_time {
                    amplifier.input.push(phase_inputs[i]);
                    amplifier.input.push(previous_output);
                    amplifier.start();
                } else {
                    amplifier.resume(previous_output);
                }

                let current_output = amplifier.output.remove(0);
                previous_output = current_output;

                i += 1;
            }

            first_time = false;
        }

        assert_eq!(previous_output, 4275738);
    }

    #[test]
    fn test_day9_input() {
        let mut cpu = IntcodeCPU::new(program_from_file("src/util/inputs/day9_input.txt"));
        cpu.input.push(1);
        cpu.output_mode = OutputMode::Integer;
        cpu.start();

        assert_eq!(cpu.output, vec![2671328082]);

        let mut cpu = IntcodeCPU::new(program_from_file("src/util/inputs/day9_input.txt"));
        cpu.input.push(2);
        cpu.output_mode = OutputMode::Integer;
        cpu.start();

        assert_eq!(cpu.output, vec![59095]);
    }
}
