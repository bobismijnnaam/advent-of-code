pub type Vec2 = (i64, i64);

pub fn add(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
}

pub fn manhattan(a: Vec2, b: Vec2) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn run_intcode_cpu(mut input: Vec<usize>) -> Option<Vec<usize>> {
    for i in (0..input.len()).step_by(4) {
        let op = input[i];
        if op == 99 {
            break;
        } else {
            let A = input[input[i+1]];
            let B = input[input[i+2]];
            let dest = input[i+3];

            match op {
                1 => input[dest] = A + B,
                2 => input[dest] = A * B,
                _ => return None,
            }
        }
    }

    Some(input)
}

