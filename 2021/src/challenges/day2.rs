use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


struct Path {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Path {
    fn area(&self) -> u32 {
        self.horizontal * self.depth
    }
}


fn solve_problem_2(input: &Vec<Vec<String>>) -> u32 {
    let mut path = Path { horizontal: 0, depth: 0, aim: 0 };
    for command in input {
        let step = command[1].parse::<u32>().unwrap();
        match command[0].as_str() {
            "forward" => {
                path.horizontal += step;
                path.depth += path.aim * step;
            },
            "up" => {
                path.aim -= step;
            },
            "down" => {
                path.aim += step;
            },
            _ => ()
        }
    }
    return path.area();
}


fn solve_problem_1(input: &Vec<Vec<String>>) -> u32 {
    let mut path = Path { horizontal: 0, depth: 0, aim: 0 };
    for command in input {
        let step = command[1].parse::<u32>().unwrap();
        match command[0].as_str() {
            "forward" => path.horizontal += step,
            "up" => path.depth -= step,
            "down" => path.depth += step,
            _ => ()
        }
    }
    return path.area();
}


pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let commands: Vec<Vec<String>> = reader
        .lines()
        .map(|line| line.unwrap().split_whitespace().map(String::from).collect::<Vec<String>>())
        .collect();

    println!("Solution to part 1: {}, solution to part 2: {}", solve_problem_1(&commands), solve_problem_2(&commands));
}
