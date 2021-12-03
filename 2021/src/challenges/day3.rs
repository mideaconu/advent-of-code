use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


type ChildNode = Option<Box<Node>>;

struct Node {
    bit_level: u32,
    values: Vec<u32>,
    most_common: ChildNode,
    least_common: ChildNode,
}

impl Node {
    pub fn new(bit_level: u32, values: &Vec<u32>) -> Self {
        if values.len() == 1 {
            return Node { bit_level: bit_level, values: values.to_vec(), most_common: None, least_common: None };
        } else {
            let mut ones = Vec::new();
            let mut zeros = Vec::new();
            for i in 0..values.len() {
                let nth_bit = get_nth_bit(values[i], bit_level);
                if nth_bit == 1 {
                    ones.push(values[i]);
                } else {
                    zeros.push(values[i]);
                }
            }

            if ones.len() >= zeros.len() {
                if zeros.len() == 0 {
                    return Node {
                        bit_level: bit_level,
                        values: values.to_vec(),
                        most_common: Some(Box::new(Node::new(bit_level-1, &ones))),
                        least_common: None,
                    };
                }

                return Node {
                    bit_level: bit_level,
                    values: values.to_vec(),
                    most_common: Some(Box::new(Node::new(bit_level-1, &ones))),
                    least_common: Some(Box::new(Node::new(bit_level-1, &zeros))),
                };
            } else {
                if ones.len() == 0 {
                    return Node {
                        bit_level: bit_level,
                        values: values.to_vec(),
                        most_common: Some(Box::new(Node::new(bit_level-1, &zeros))),
                        least_common: None,
                    };
                }
                
                return Node {
                    bit_level: bit_level,
                    values: values.to_vec(),
                    most_common: Some(Box::new(Node::new(bit_level-1, &zeros))),
                    least_common: Some(Box::new(Node::new(bit_level-1, &ones))),
                };
            }
        }
    }
}

struct Tree {
    root: ChildNode,
}

impl Tree {
    pub fn new(values: &Vec<u32>) -> Self {
        let n_bits: u32 = get_bit_size(*values.iter().max().unwrap());

        if values.len() == 0 {
            return Tree { root: None };
        }
        
        return Tree { root: Some(Box::new(Node::new(n_bits, &values))) };
    }

    pub fn get_o2_rating(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }

        let mut node = self.root.as_ref().unwrap();
        while node.values.len() != 1 {
            node = node.most_common.as_ref().unwrap();
        }

        return node.values[0];
    }

    pub fn get_co2_rating(&self) -> u32 {
        if self.root.is_none() {
            return 0;
        }

        let mut node = self.root.as_ref().unwrap();
        while node.values.len() != 1 {
            node = node.least_common.as_ref().unwrap();
        }

        return node.values[0];
    }
}


fn get_nth_bit(number: u32, n: u32) -> u32 {
    1 & (number >> (n-1))
} 


fn get_bit_size(mut number: u32) -> u32 {
    let mut size = 0;
    while number > 0 {
        size += 1;
        number >>= 1;
    }
    return size;
}


fn solve_problem_2(input: &Vec<u32>) -> u32 {
    let tree = Tree::new(&input);
    return tree.get_o2_rating() * tree.get_co2_rating();
}


fn solve_problem_1(input: &Vec<u32>) -> u32 {
    let mut numbers = input.to_vec();
    let mut gamma_rate: u32 = 0;
    let n_bits: u32 = get_bit_size(*input.iter().max().unwrap());

    for pos in 0..n_bits {
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

    let n_spare_bits = u32::BITS - n_bits;
    let epsilon_rate: u32 = (!gamma_rate << n_spare_bits) >> n_spare_bits;

    return gamma_rate * epsilon_rate;
}


pub fn solve(input_path: std::path::PathBuf) {
    let file = File::open(input_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| u32::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect::<Vec<u32>>();

    println!("Solution to part 1: {}\nSolution to part 2: {}", solve_problem_1(&numbers), solve_problem_2(&numbers));
}
