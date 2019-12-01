use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_fuel(mass: i32) -> i32 {
    let fuel = (mass as f64 / 3.0).floor() as i32 - 2;
    if fuel < 0 {
        0 
    } else {
        fuel + calc_fuel(fuel)
    }
}

fn main() {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        let mass: i32 = line.parse().unwrap();
        let fuel = calc_fuel(mass);
        total += fuel;
        println!("{} -> {}", mass, fuel);
    }
    dbg!(calc_fuel(1969));
    dbg!(total);
}
