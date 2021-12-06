use std::fs::File;
use std::io::{BufRead, BufReader};


fn solve_problem(input: &Vec<usize>, days_to_deadline: u16) -> usize {
    let mut fish_by_age: [usize; 9] = [0; 9];

    for fish_age in input {
        fish_by_age[*fish_age] += 1;
    }
    
    for _ in 0..days_to_deadline {
        let mut cur; 
        let mut last = 0;
        for i in (0..9).rev() {
            cur = fish_by_age[i];
            fish_by_age[i] = last;
            last = cur;
        }
        fish_by_age[8] = last;
        fish_by_age[6] += last;
    }

    return fish_by_age.iter().sum::<usize>();
}

fn solve_problem_2(input: &Vec<usize>) -> usize {
    solve_problem(input, 256)
}

fn solve_problem_1(input: &Vec<usize>) -> usize {
    solve_problem(input, 80)
}

pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");

    let mut first_line = String::new();
    let mut reader = BufReader::new(file);
    let _ = reader.read_line(&mut first_line);

    let input: Vec<usize> = first_line
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Solution to part 1: {}\nSolution to part 2: {}", solve_problem_1(&input), solve_problem_2(&input));
}