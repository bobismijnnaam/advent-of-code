#![feature(is_sorted)]

fn main1() {
    let mut i = 0;
    for password in 134792..(675810 + 1) {
        let password = password.to_string();
        if !password.chars().is_sorted() {
            continue;
        }
        let has_adjacent = password.chars().fold(
            (None, false),
            |(previous_char, has_adjacent), current_char| {
                (Some(current_char), has_adjacent || previous_char == Some(current_char))
            }).1;
        if !has_adjacent {
            continue;
        }
//        dbg!(password);
        i += 1;
    }
    dbg!(i);
}

fn main2() {
    // 915 too low
    // 1089 too low
    let mut i = 0;
    for password_ in 134792..(675810 + 1) {
        let password = password_.to_string();
        if !password.chars().is_sorted() {
//            println!("Reject {}: not sorted", password_);
            continue;
        }
        let result = password.chars().fold(
            ('0', 0, false), // Char, current count, highest count
            |(previous_char, current_count, seen_exactly_2), current_char| {
                if previous_char == current_char {
                    let new_current_count = current_count + 1;
                    (current_char, new_current_count, seen_exactly_2)
                } else {
                    (current_char, 1, seen_exactly_2 || current_count == 2)
                }
            });
        if !(result.2 || result.1 == 2) {
//            println!("Reject {}: adjecent count = {:?}", password_, result);
            continue;
        }
//        dbg!(password);
        i += 1;
    }
    dbg!(i);
}

fn main() {
    main1();
    main2();
}
