use std::fs;
use std::collections::{HashMap, HashSet};

fn get_orbit_trace<'a>(start: &'a str, orbit_relation: &HashMap<&'a str, &'a str>) -> HashSet<&'a str> {
    let mut current = orbit_relation[start];
    let mut objects = vec![current];
    while current != "COM" {
        current = orbit_relation[current];
        objects.push(current);
    }
    objects.into_iter().collect()
}

fn main() {
    let orbit_map = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut orbit_relation = HashMap::new();
    let mut all_objects = HashSet::new();

    for orbit in orbit_map.split("\n") {
        let subjects: Vec<_> = orbit.split(")").collect();
        orbit_relation.insert(subjects[1], subjects[0]);
        all_objects.insert(subjects[0]);
        all_objects.insert(subjects[1]);
    }

    let mut total_orbit_count = 0;
    println!("Counting...");
    for object in all_objects {
        if object != "COM" {
            total_orbit_count += get_orbit_trace(object, &orbit_relation).len();
        }
    }

    dbg!(total_orbit_count);

    // 315757: correct
    // 482: wrong

    let you_trace = get_orbit_trace("YOU", &orbit_relation);
    let san_trace = get_orbit_trace("SAN", &orbit_relation);
    let common_objects = you_trace.intersection(&san_trace).cloned().collect();
    let dist = you_trace.difference(&common_objects).count() + san_trace.difference(&common_objects).count() + 1;

    println!("Total objects inbetween: {}. So number of transfers = {} - 1 = {}", dist, dist, dist - 1);
}
