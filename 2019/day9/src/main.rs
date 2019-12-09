mod util;

use util::intcode::*;

fn main1() {
    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.input.push(1);
    cpu.print_output = true;
    cpu.start();

    // 21107 wrong
    // 2402892643 wrong
    // 2671328082 correct
}

fn main() {
//    main1();

    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.input.push(2);
    cpu.print_output = true;
    cpu.start();

    // 59095 correct
}
