
fn compute(mut input: Vec<usize>) -> Option<Vec<usize>> {
    for i in (0..input.len()).step_by(4) {
        //println!("---- {}", i);
        //dbg!(&input);
        //dbg!(input[i]);

        let op = input[i];
        if op == 99 {
            break;
        } else {
            //dbg!(input[i+1]);
            //dbg!(input[i+2]);
            //dbg!(input[i+3]);

            let A = input[input[i+1]];
            let B = input[input[i+2]];
            let dest = input[i+3];

            match op {
                1 => input[dest] = A + B,
                2 => input[dest] = A * B,
                _ => return None,
            }
        }

        //println!("after");
        //dbg!(input[i]);
        //dbg!(input[i+1]);
        //dbg!(input[i+2]);
        //dbg!(input[i+3]);
    }

    Some(input)
}

fn main() {
    println!("Hello, world!");

    let input: Vec<usize> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,5,19,23,1,6,23,27,1,27,10,31,1,31,5,35,2,10,35,39,1,9,39,43,1,43,5,47,1,47,6,51,2,51,6,55,1,13,55,59,2,6,59,63,1,63,5,67,2,10,67,71,1,9,71,75,1,75,13,79,1,10,79,83,2,83,13,87,1,87,6,91,1,5,91,95,2,95,9,99,1,5,99,103,1,103,6,107,2,107,13,111,1,111,10,115,2,10,115,119,1,9,119,123,1,123,9,127,1,13,127,131,2,10,131,135,1,135,5,139,1,2,139,143,1,143,5,0,99,2,0,14,0];
    
    let mut q1input = input.clone();
    q1input[1] = 12;
    q1input[2] = 2;

    if let Some(input) = compute(q1input) {
        dbg!(input[0]);
    } else {
        dbg!("oops");
    }

    dbg!(compute(vec![1,1,1,4,99,5,6,0,99]));

    println!("--------------------");

    for noun in 0..100 {
        for verb in 0..100 {
            let mut input = input.clone();
            input[1] = noun;
            input[2] = verb;

            if let Some(res) = compute(input) {
                if res[0] == 19690720 {
                    dbg!((noun, verb));
                    dbg!(noun*100+verb);
                    break;
                }
            } else {
                // ok
            }
        }
    }
}
