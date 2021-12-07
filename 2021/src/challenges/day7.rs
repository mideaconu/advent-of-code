use std::fs::File;
use std::io::{BufRead, BufReader};


fn median(numbers: &Vec<isize>) -> isize {
    numbers[numbers.len()/2]
}

fn mean(numbers: &Vec<isize>) -> isize {
    numbers.iter().sum::<isize>() as isize / numbers.len() as isize
}


fn solve_problem_1(input: &Vec<isize>) -> isize {
    let median = median(&input);

    let mut total_fuel: isize = 0;
    for number in input {
        total_fuel += isize::abs(number - median);
    }   
    
    total_fuel
}


fn get_constant_rate_fuel(input: &Vec<isize>, mean: isize) -> isize {
    let mut total_fuel: isize = 0;
    for number in input {
        let distance = isize::abs(number - mean);
        total_fuel += distance * (distance + 1) / 2;
    }
    total_fuel
}


fn solve_problem_2(input: &Vec<isize>) -> isize {
    let total_fuels: [isize; 2] = [get_constant_rate_fuel(&input, mean(&input)), get_constant_rate_fuel(&input, mean(&input) + 1)];
    *total_fuels.iter().min().unwrap()
}


pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");

    let mut first_line = String::new();
    let mut reader = BufReader::new(file);
    let _ = reader.read_line(&mut first_line);

    let mut input: Vec<isize> = first_line
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    
    input.sort();

    println!("Solution to part 1: {}\nSolution to part 2: {}", solve_problem_1(&input), solve_problem_2(&input));
}