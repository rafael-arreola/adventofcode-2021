use std::{env, io::{BufRead, BufReader}, fs::File};


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("file not found");
    let lines: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let binary_length = lines.get(0).unwrap().len();
    let max_length = lines.len();

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for index in 0..binary_length {
        let mut zeros = 0;
        for line in &lines {
            let binary = line.chars().nth(index).unwrap();
            if binary == '0' {
                zeros += 1;
            }
        }
        if zeros > max_length / 2 {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);

    let power_consuption = i32::from_str_radix(&*gamma_rate, 2).unwrap() * i32::from_str_radix(&*epsilon_rate, 2).unwrap();
    println!("Power consumption: {}", power_consuption);
}
