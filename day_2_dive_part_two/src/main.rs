use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let file = std::fs::File::open(filename).expect("file not found");
    let lines = BufReader::new(file).lines();

    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for line in lines {
        let line = line.unwrap();
        let components: Vec<&str> = line.split(' ').collect();
        let direction = components[0];
        let x = components[1].parse::<i32>().unwrap();
        match direction {
            "forward" => {
                horizontal += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!("invalid direction in file"),
        }
    }

    println!("horizontal: {}", horizontal);
    println!("depth: {}", depth);
    println!("R = {}", horizontal * depth);
}
