use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("file not found");
    let lines = BufReader::new(file).lines();

    let mut increases: i32 = 0;
    let mut previous: i32 = -1;
    for line in lines {
        if previous == -1 {
            previous = line.unwrap().parse::<i32>().unwrap();
            continue;
        } else {
            let current = line.unwrap().parse::<i32>().unwrap();
            if current > previous {
                increases += 1;
            }
            previous = current;
        }
    }
    println!("How many measurements are larger than the previous measurement?");
    println!("R = {}", increases);
}