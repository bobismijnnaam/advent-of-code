use crate::util::intcode::{IntcodeCPU, program_from_file, OutputMode};

mod util;

fn main1() {
    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.output_mode = OutputMode::ASCII;

    let springscript = "NOT D J
WALK
";

    let springscript = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

    cpu.str_input(springscript);

    cpu.start();

    assert_eq!(cpu.output.pop().unwrap(), 19358688);
}

fn main2() {
    let mut cpu = IntcodeCPU::new(program_from_file("input.txt"));
    cpu.output_mode = OutputMode::ASCII;

    let springscript =
"\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
";

    cpu.str_input(springscript);

    cpu.start();

    assert_eq!(cpu.output.pop().unwrap(), 1141236756);
}

fn main() {
    main1();
    main2();
}
