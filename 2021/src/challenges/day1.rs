use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn solve_problem_1(input: &Vec<u32>) -> u16 {
    let mut nr_increased = 0;
    let mut last_measurement = input[0];

    for i in 1..input.len() {
        if input[i] > last_measurement {
            nr_increased += 1;
        }
        last_measurement = input[i];
    }

    return nr_increased;
}


fn solve_problem_2(input: &Vec<u32>) -> u16 {
    let mut nr_increased = 0;
    let mut last_measurement = input[0] + input[1] + input[2];

    for i in 3..input.len() {
        let current_measurement = input[i - 2] + input[i - 1] + input[i];
        if current_measurement > last_measurement {
            nr_increased += 1;
        }
        last_measurement = current_measurement;
    }

    return nr_increased;
}


pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    println!("Solution to part 1: {}\nSolution to part 2: {}", solve_problem_1(&numbers), solve_problem_2(&numbers));
}
