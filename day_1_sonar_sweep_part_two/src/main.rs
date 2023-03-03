use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("file not found");


    let mut lines = Vec::new();
    BufReader::new(file).lines().for_each(|line| {
        lines.push(line.unwrap().parse::<i32>().unwrap());
    });

    let mut iteration = 0;
    let mut results: Vec<i32> = Vec::new();
    while iteration + 2 < lines.len() {
        results.push(lines.get(iteration).unwrap() + lines.get(iteration + 1).unwrap() + lines.get(iteration + 2).unwrap());
        iteration += 1;
    }

    let mut increases: i32 = 0;
    let mut previous: i32 = -1;
    for line in results.iter() {
        if previous == -1 {
            previous = line.clone();
            continue;
        } else {
            let current = line.clone();
            if current > previous {
                increases += 1;
            }
            previous = current;
        }
    }
    println!("How many measurements are larger than the previous measurement?");
    println!("R = {}", increases);
}