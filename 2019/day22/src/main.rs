use std::fs::read_to_string;

////////////
// Part 1 //
////////////

fn deal_into_new_stack(stack: &mut Vec<i64>) {
    stack.reverse();
}

fn cut_n(stack: &mut Vec<i64>, n: i64) {
    if n > 0 {
        let top_part: Vec<_> = stack.drain(stack.len() - n as usize ..).collect();
        stack.splice(0..0, top_part);
    } else if n < 0 {
        let bottom_part: Vec<_> = stack.drain(..n.abs() as usize).collect();
        stack.extend(bottom_part);
    }
}

fn deal_with_increment_n(stack: &mut Vec<i64>, n: i64) {
    let mut res = stack.clone();

    for i in 0..stack.len() {
        let elem_i = stack.len() - 1 - i;
        let target_i = stack.len() - 1 - ((i * n as usize) % stack.len());
        res[target_i] = stack[elem_i];
    }

    std::mem::swap(&mut res, stack);
}

#[derive(Debug, Copy, Clone)]
enum Technique {
    DealNewStack,
    CutN(i64),
    DealIncrement(i64)
}

fn parse(input: &str) -> Vec<Technique> {
    use Technique::*;

    input.trim().split("\n")
        .map(|row: &str| {
            if row.starts_with("deal into new stack") {
                DealNewStack
            } else if row.starts_with("deal with increment ") {
                DealIncrement(
                    row.split("deal with increment ")
                        .skip(1)
                        .next().unwrap().parse().unwrap())
            } else if row.starts_with("cut ") {
                CutN(
                    row.split("cut ")
                        .skip(1)
                        .next().unwrap().parse().unwrap())
            } else {
                panic!()
            }
        }).collect()
}

fn test_input(file: &str) {
    let input = parse(&read_to_string(file).unwrap());
    let mut stack: Vec<i64> = (0..10).collect();
    stack.reverse();

    apply_techniques(&input, &mut stack);

    dbg!(stack);
}

fn apply_techniques(techniques: &[Technique], stack: &mut Vec<i64>) {
    use Technique::*;

    for &technique in techniques {
        match technique {
            DealNewStack => deal_into_new_stack(stack),
            CutN(n) => cut_n(stack, n),
            DealIncrement(n) => deal_with_increment_n(stack, n),
        }
    }
}

fn main1() {
    let input = parse(&read_to_string("input.txt").unwrap());

    let mut test = vec![5, 4, 3, 2, 1, 0];

    deal_into_new_stack(&mut test);
    dbg!(&test);

    cut_n(&mut test, 2);
    dbg!(&test);

    cut_n(&mut test, -2);
    dbg!(&test);

    let mut test = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    deal_with_increment_n(&mut test, 3);
    dbg!(&test);

    test_input("test1.txt");
    test_input("test2.txt");
    test_input("test3.txt");
    test_input("test4.txt");

    let mut stack: Vec<i64> = (0..=10006).collect();
    stack.reverse();

    apply_techniques(&input, &mut stack);

//    dbg!(stack[stack.len() - 1 - 2019]);

    // 5320 too high
    // 1890 too high

    let card_index = stack.iter().position(|&c| c == 2019).unwrap();
    let card_pos = stack.len() - 1 - card_index;

    dbg!(card_pos);
}

////////////
// Part 2 //
////////////

// Stolen from the internet!
fn mod_inverse(mut a: i64, mut m: i64) -> i64 {
    let m0 = m;
    let mut y = 0;
    let mut x = 1;

    while a > 1 {
        let q = a / m;
        let mut t = m;

        m = a % m;
        a = t;
        t = y;

        y = x - q * y;
        x = t;
    }

    if x < 0 {
        x += m0;
    }

    x
}

fn safe_multiply(a: i64, b: i64, size: i64) -> i64 {
    let (a, b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };

    let a = (a + size) % size;
    let b = (b + size) % size;

    if a == 1 {
        return b;
    } else if b == 1 {
        return a;
    } else if a == 0 || b == 0 {
        return 0;
    }

    if (b % 2) == 0 {
        // b even
        // a * b = 2 * a * (b / 2),

        let x = (2 * a) % size;
        safe_multiply(x, b / 2, size)
    } else {
        // b odd
        // a * b = a + a * (b - 1)
        let x = safe_multiply(a, b - 1, size);
        (a + x) % size
    }
}

// a * x + b
#[derive(Debug, Copy, Clone)]
struct Transform {
    size: i64,
    a: i64,
    b: i64
}

impl Transform {
    fn id(size: i64) -> Transform {
        Transform {
            size,
            a: 1,
            b: 0
        }
    }

    fn sequence(&self, other: Transform) -> Transform {
        // self: ax + b
        // other: cx + d
        // result: c*(ax+b) + d
        //         (ca)*x + (cb + d)
        //

        let a = self.a;
        let b = self.b;
        let c = other.a;
        let d = other.b;

        Transform {
            size: self.size,
            a: safe_multiply(a, c, self.size),
            b: (safe_multiply(c, b, self.size) + d) % self.size
        }
    }

    fn interp(&self, x: i64) -> i64 {
        let multiply = (safe_multiply(self.a, x, self.size) + self.size) % self.size;
        (multiply + self.b) % self.size
    }
}

// old_pos = -1 * new_pos + (size-1)
fn transform_reverse_deal(size: i64) -> Transform {
    Transform {
        size,
        a: -1,
        b: size - 1
    }
}

// old_pos = 1 * new_pos + (n+size%size)
fn transform_reverse_cut_n(size: i64, n: i64) -> Transform {
    Transform {
        size,
        a: 1,
        b: (n + size) % size, // To account for negative n
    }
}

// old_pos = mod_inverse(n, size) * new_pos + 0
fn transform_reverse_deal_with_increment_n(size: i64, n: i64) -> Transform {
    Transform {
        size,
        a: mod_inverse(n, size),
        b: 0
    }
}

fn transform_techniques(size: i64, techniques: &[Technique]) -> Transform {
    let mut transform = Transform::id(size);

    for technique in techniques.iter().rev() {
        let next_transform = match technique {
            Technique::DealNewStack => transform_reverse_deal(size),
            Technique::CutN(n) => transform_reverse_cut_n(size, *n),
            Technique::DealIncrement(n) => transform_reverse_deal_with_increment_n(size, *n),
        };

        let new_transform = transform.sequence(next_transform);
        transform = new_transform;
    }

    transform
}

fn apply(transform: Transform, num_left: i64) -> Transform {
    if num_left == 0 {
        panic!()
    } else if num_left == 1 {
        transform
    } else if (num_left % 2) == 0 {
        let double_transform = transform.sequence(transform);
        apply(double_transform, num_left / 2)
    } else {
        let nested_transform = apply(transform, num_left - 1);

        transform.sequence(nested_transform)
    }
}

fn main5() {
    let input = parse(&read_to_string("test4.txt").unwrap());
    let size = 10;

    let transform = transform_techniques(size, &input);

    for i in 0..10 {
        dbg!(transform.interp(i));
    }



    let input = parse(&read_to_string("input.txt").unwrap());
    let size = 10007;

    let transform = transform_techniques(size, &input);

    dbg!(&transform);

    dbg!(transform.interp(1234));



    let input = parse(&read_to_string("input.txt").unwrap());
    let size = 119315717514047;
    let num_applications = 101741582076661;

    let transform = transform_techniques(size, &input);

    let nested_transform = apply(transform, num_applications);

    dbg!(nested_transform);

    dbg!(nested_transform.interp(2020));

    // 99438197001748 too high
    // 26976139807028 too high
}

fn main() {
    main1();
    main5();
}
