use std::io::BufRead;

#[derive(PartialEq)]
enum BitPref {
    UPPER,
    LOWER,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let file = std::fs::File::open(filename).unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let mut oxygen_generator: Vec<String> = lines.clone();
    let mut co2_scrubber: Vec<String> = lines.clone();
    let binary_length = lines.get(0).unwrap().len();

    for index in 0..binary_length {
        oxygen_generator = reduce_universe(oxygen_generator, index, BitPref::UPPER);
        co2_scrubber = reduce_universe(co2_scrubber, index, BitPref::LOWER);
    }

    let oxygen = oxygen_generator.get(0).unwrap();
    let co2 = co2_scrubber.get(0).unwrap();
    let oxygen_number = i32::from_str_radix(&*oxygen, 2).unwrap();
    let co2_number = i32::from_str_radix(&*co2, 2).unwrap();
    println!("Oxygen Generator: {}", oxygen);
    println!("CO2 Scrubber: {}", co2);
    println!("Oxygen Generator: {}", oxygen_number);
    println!("CO2 Scrubber: {}", co2_number);
    println!("What is the life support rating of the submarine? {}", oxygen_number * co2_number);
}


fn reduce_universe(universe: Vec<String>, index: usize, preference: BitPref) -> Vec<String> {
    if universe.len() == 1 {
        return universe;
    }
    let mut start_with_zero: Vec<String> = Vec::new();
    let mut start_with_one: Vec<String> = Vec::new();
    for line in universe {
        let binary = line.chars().nth(index).unwrap();
        if binary == '0' {
            start_with_zero.push(line);
        } else {
            start_with_one.push(line);
        }
    }

    return if preference == BitPref::UPPER {
        if start_with_one.len() >= start_with_zero.len() {
            start_with_one
        } else {
            start_with_zero
        }
    } else {
        if start_with_zero.len() <= start_with_one.len() {
            start_with_zero
        } else {
            start_with_one
        }
    };
}