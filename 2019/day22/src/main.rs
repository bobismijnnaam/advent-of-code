use std::fs::read_to_string;
use std::io::Write;
use std::collections::{HashMap, HashSet};

fn deal_into_new_stack(stack: &mut Vec<i64>) {
    stack.reverse();
}

fn cut_n(stack: &mut Vec<i64>, n: i64) {
    if n > 0 {
        let top_part: Vec<_> = stack.drain(stack.len() - n as usize ..).collect();
        stack.splice((0..0), top_part);
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
        println!("Stack len: {}", stack.len());
        println!("Technique: {:?}", technique);
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

// Deal into new deck:
// new_i = size() - 1 - i

// Cut n:
// if size() - n < i {

fn deal_into_new_deck_i(size: i64, old_i: i64) -> i64 {
    size - 1 - old_i
}

fn deal_with_increment_n_i(size: i64, n: i64, old_i: i64) -> i64 {
    (old_i * n) % size
}

fn cut_n_i(size: i64, n: i64, old_i: i64) -> i64 {
   (old_i - n + size) % size
}

fn apply_techniques_i(techniques: &[Technique], size: i64, mut i: i64) -> i64 {
    use Technique::*;

    for &technique in techniques {
        i = match technique {
            DealNewStack => deal_into_new_deck_i(size, i),
            CutN(n) => cut_n_i(size, n, i),
            DealIncrement(n) => deal_with_increment_n_i(size, n, i),
        }
    }

    i
}

/*

Inverses:

deal:
    old_i = (SIZE - 1) - new_i

cut n:
    old_i = (new_i + n + size) % size

deal with increment n:
    old_i = new_i / n
    old_i = new_i * (1 / n)
    old_i = new_i * multiplicative_inverse(n)

*/

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

#[derive(Debug)]
enum ModExpr {
    Lit(i64),
    Var,
    Sub(i64, Box<ModExpr>),
    Add(Box<ModExpr>, i64),
    Mul(Box<ModExpr>, i64),
}

use ModExpr::*;

// old_pos = -1 * new_pos + (size-1)
fn reverse_deal(size: i64, new_i: ModExpr) -> ModExpr {
    let m = Mul(Box::new(new_i), -1);
    Add(Box::new(m), size - 1)
}

// old_pos = 1 * new_pos + (n+size%size)
fn reverse_cut_n(size: i64, n: i64, new_i: ModExpr) -> ModExpr {
//    Add(Box::new(new_i), (n + size) % size)
    let m = Mul(Box::new(new_i), 1);
    Add(Box::new(m), (n + size) % size)
}

// old_pos = mod_inverse(n, size) * new_pos + 0
fn reverse_deal_with_increment_n(size: i64, n: i64, new_i: ModExpr) -> ModExpr {
//    Mul(Box::new(new_i), mod_inverse(n, size))
    let m = Mul(Box::new(new_i), mod_inverse(n, size));

    Add(Box::new(m), 0)
}

fn interp(me: ModExpr, size: i64, var: i64) -> i64 {
    match me {
        Lit(n) => n % size,
        Sub(a, b) => {
            let mut b = interp(*b, size, var);
            a.checked_sub(b).unwrap().checked_add(size).unwrap() % size
        }
        Add(a, b) => {
            (interp(*a, size, var) + b) % size
        },
        Mul(a, b) => {
            let a = interp(*a, size, var);
            a.checked_mul(b).unwrap() % size
        },
        Var => var
    }
}

fn calc_back(mut modulo_expr: ModExpr, size: i64, techniques: &[Technique]) -> ModExpr {
    for technique in techniques.iter().rev() {
        use Technique::*;
        modulo_expr = match technique {
            DealNewStack => reverse_deal(size, modulo_expr),
            CutN(n) => reverse_cut_n(size, *n, modulo_expr),
            DealIncrement(n) => reverse_deal_with_increment_n(size, *n, modulo_expr),
        }
    }
    modulo_expr
}

// a(cx + d) + b
// (ac)x + (ad + b)

fn simplify(me: ModExpr, size: i64) -> ModExpr {
    match me {
        me @ Lit(_) => me,
        me @ Var => me,
        Sub(n, me) => panic!(),
        Add(me, n) => simplify_add(*me, n, size),
        Mul(me, n) => simplify_mul(*me, n, size),
    }
}

fn simplify_add(me: ModExpr, n: i64, size: i64) -> ModExpr {
    match me {
        Lit(m) => Lit((n + m) % size),
        Var => panic!(),
        Sub(m, me) => Sub((m + n) % size, me),
        Add(me, m) => Add(me, (n + m) % size),
        Mul(_, _) => panic!(),
    }
}

fn simplify_mul(me: ModExpr, n: i64, size: i64) -> ModExpr {
    match me {
        Lit(m) => Lit(safe_multiply(m, n, size)),
        Var => panic!(),
        Sub(m, me) => Sub(safe_multiply(n, m, size), Box::new(simplify_mul(*me, n, size))),
        Add(me, m) => Add(Box::new(simplify_mul(*me, n, size)), safe_multiply(m, n, size)),
        Mul(me, m) => Mul(me, safe_multiply(n, m, size)),
    }
}

//fn remove_sub(me: ModExpr, size: i64) -> ModExpr {
//    match me {
//        me @ Lit(_) => me,
//        me @ Var => me,
//        me@ Add(me, n) => Add(Box::new(remove_sub(me, size)), n) ,
//        me@ Mul(me, n) => Mul(Box::new(remove_sub(me, size)), n) ,
//        Sub(n, me) => {
//            let l = Box::new(remove_sub(me, size))
//            Add()
//        }
//    }
//}

fn print(me: ModExpr) -> String {
    match me {
        Lit(n) => n.to_string(),
        Var => "var".to_owned(),
        Sub(n, me) => {
            let mut res = "(".to_string();
            res.push_str(&n.to_string());
            res.push_str(" - ");
            let me_str = print(*me);
            res.push_str(&me_str);
            res.push_str(")");
            res
        },
        Add(me, n) => {
            let mut res = "(".to_string();
            let me_str = print(*me);
            res.push_str(&me_str);
            res.push_str(" + ");
            res.push_str(&n.to_string());
            res.push_str(")");
            res
        }
        Mul(me, n) => {
            let mut res = "(".to_string();
            let me_str = print(*me);
            res.push_str(&me_str);
            res.push_str(" * ");
            res.push_str(&n.to_string());
            res.push_str(")");
            res
        }
    }
}

fn main3() {
    let input = parse(&read_to_string("input.txt").unwrap());
//    let input = parse(&read_to_string("test4.txt").unwrap());

//    let mut modulo_expr = Lit(2020);
//    let mut modulo_expr = Lit(1234);
    let size = 10007;
//    let size = 10;
//    let big_size: i64 = 119315717514047;
//
//    dbg!(&modulo_expr);
//
//    dbg!(calc_back(Lit(1234), size, &input));
//
//    dbg!(interp(calc_back(Var, size, &input), size, 1234));

//    print!("{}", print(calc_back(Var, size, &input)));
//    print!("{}", print(calc_back(Lit(0), size, &input)));

//    let base_expr = Add(Box::new(Mul(Box::new(Var), 1)), 0);
    let base_expr = Var;

    let mod_expr = calc_back(base_expr, size, &input);
    dbg!(&mod_expr);

    let mut mod_expr = simplify(mod_expr, size);

    loop {
        dbg!(&mod_expr);
        mod_expr = simplify(mod_expr, size);
    }
}

fn main2() {
    dbg!(mod_inverse(7, 3));

    let input = parse(&read_to_string("input.txt").unwrap());

    let normal_size = 10007;

    let res = apply_techniques_i(&input, 10007, 2019);
    assert_eq!(res, 1234);

    let big_size: i64 = 119315717514047;
    let reps: i64 = 101741582076661;

    let mut index = 2020;

    for i in 0..reps {
        if i % 1000000 == 0 {
            let percent = ((i as f64 / reps as f64) * 100f64).floor() as i32;
            print!("\r{} ({}%) ({})          ", i, percent, 0);
            std::io::stdout().flush().ok();
        }

        index = apply_techniques_i(&input, big_size, index);
    }

    dbg!(index);
}

fn reverse_deal_c(size: i64, new_i: i64) -> i64 {
    ((size - 1) - new_i) % size
}

fn reverse_cut_n_c(size: i64, n: i64, new_i: i64) -> i64 {
    (new_i + n + size) % size
}

fn reverse_deal_with_increment_n_c(size: i64, n: i64, new_i: i64) -> i64 {
//    (new_i.checked_mul(mod_inverse(n, size)).unwrap()) % size
    safe_multiply(new_i, mod_inverse(n, size), size)
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

fn calc_back_c(mut new_i: i64, size: i64, techniques: &[Technique]) -> i64 {
    for technique in techniques.iter().rev() {
        use Technique::*;
        new_i = match technique {
            DealNewStack => reverse_deal_c(size, new_i),
            CutN(n) => reverse_cut_n_c(size, *n, new_i),
            DealIncrement(n) => reverse_deal_with_increment_n_c(size, *n, new_i),
        }
    }

    new_i
}

fn my_mod(a: i64, size: i64) -> i64 {
    (if a < 0 {
        a + size
    } else {
        a
    }) % size
}

fn main4() {
    let input = parse(&read_to_string("input.txt").unwrap());
//    let input = parse(&read_to_string("test4.txt").unwrap());

    let small_size = 10007;
    let big_size: i64 = 119315717514047;

//    dbg!(&modulo_expr);

    dbg!(calc_back_c(1234, small_size, &input));

    let z = 2020;
    let y = calc_back_c(z, big_size, &input);
    let x = calc_back_c(y, big_size, &input);
    let w = calc_back_c(x, big_size, &input);

    dbg!(x, y, z);
    dbg!(my_mod(z - y, big_size));
    dbg!(my_mod(y - z, big_size));
    dbg!(my_mod(y - x, big_size));
    dbg!(my_mod(x - y, big_size));
    dbg!(my_mod(x - w, big_size));
    dbg!(my_mod(w - x, big_size));
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
//    let m = Mul(Box::new(new_i), -1);
//    Add(Box::new(m), size - 1)

    Transform {
        size,
        a: -1,
        b: size - 1
    }
}

// old_pos = 1 * new_pos + (n+size%size)
fn transform_reverse_cut_n(size: i64, n: i64) -> Transform {
//    let m = Mul(Box::new(new_i), 1);
//    Add(Box::new(m), (n + size) % size)


//fn reverse_cut_n_c(size: i64, n: i64, new_i: i64) -> i64 {
//    (new_i + n + size) % size
//}

    Transform {
        size,
        a: 1,
        b: (n + size) % size,
    }
}

// old_pos = mod_inverse(n, size) * new_pos + 0
fn transform_reverse_deal_with_increment_n(size: i64, n: i64) -> Transform {
//    Mul(Box::new(new_i), mod_inverse(n, size))
//    let m = Mul(Box::new(new_i), mod_inverse(n, size));
//
//    Add(Box::new(m), 0)

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
    dbg!(num_left);
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
//    main2();
//    main3();
//    main4();
    main5();
}
