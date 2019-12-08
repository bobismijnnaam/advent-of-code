mod util;
use util::intcode::*;

use permutohedron::heap_recursive;

fn try_phase_input1(phase_inputs: &[i64]) -> i64 {
    let mut previous_output = 0;
    for phase_input in phase_inputs {
        let cpu = run_intcode_cpu(
            vec![*phase_input, previous_output],
            program_from_file("input.txt"),
            false);

        let current_output = cpu.output[0];
        previous_output = current_output;
    }

    return previous_output;
}

fn main1() {
    let mut phase_inputs = vec![3,1,2,4,0];
    let mut max_phase_output = None;
    heap_recursive(&mut phase_inputs, |phase_permutation| {
        let phase_output = try_phase_input1(phase_permutation);
        if max_phase_output.is_none() {
            max_phase_output = Some(phase_output);
        } else if phase_output > max_phase_output.unwrap() {
            max_phase_output = Some(phase_output);
        }
    });
    dbg!(max_phase_output);

    assert_eq!(max_phase_output.unwrap(), 95757);
}

fn try_phase_input2(phase_inputs: &[i64], program: &Vec<i64>) -> i64 {
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

    return previous_output;
}

fn main2() {
    let program = program_from_file("input.txt");
    let mut phase_inputs = vec![5, 6, 7, 8, 9];
    let mut max_permutation: Vec<i64> = vec![];
    let mut max_phase_output = None;
    heap_recursive(&mut phase_inputs, |phase_permutation| {
        let phase_output = try_phase_input2(phase_permutation, &program);
        if max_phase_output.is_none() {
            max_phase_output = Some(phase_output);
        } else if phase_output > max_phase_output.unwrap() {
            max_phase_output = Some(phase_output);
            max_permutation = phase_permutation.iter().cloned().collect();
        }
    });
    dbg!(max_phase_output);
    dbg!(max_permutation);
    assert_eq!(max_phase_output.unwrap(), 4275738);
}

fn main() {
    // 95757
    main1();
    // 4275738
    main2();
}