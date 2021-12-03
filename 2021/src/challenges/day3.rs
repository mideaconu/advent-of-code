use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn get_bit_size(mut number: u32) -> u32 {
    let mut size = 0;
    while number > 0 {
        size += 1;
        number >>= 1;
    }
    return size;
}


fn solve_problem_1(input: &Vec<u32>) -> u32 {
    let mut numbers = input.to_vec();
    let mut gamma_rate: u32 = 0;
    let nr_bits: u32 = get_bit_size(*input.iter().max().unwrap());

    for pos in 0..nr_bits {
        let mut ones = 0;
        for i in 0..numbers.len() {
            if numbers[i] % 2 == 1 {
                ones += 1;
            }
            numbers[i] >>= 1;
        }
        if ones > numbers.len() / 2 {
            gamma_rate += 1 << pos;
        }
    }

    let spare_bits = u32::BITS - nr_bits;
    let epsilon_rate: u32 = (!gamma_rate << spare_bits) >> spare_bits;

    return gamma_rate * epsilon_rate;
}


pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| u32::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect::<Vec<u32>>();

    println!("Solution to part 1: {}", solve_problem_1(&numbers));
}
