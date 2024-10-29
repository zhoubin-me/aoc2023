use std::collections::HashMap;
use std::env;
use std::fs;
use num::{Integer, Num};


pub fn gcd<T: Integer + Copy>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T: Integer + Copy>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

fn get_number_of_steps(
    start: &str,
    end: &str,
    directions: &[u8],
    nodes: &HashMap<&str, [&str; 2]>,
) -> usize {
    let mut current_node: &str = start;
    for n_steps in 0.. {
        if current_node == end || (end.is_empty() && current_node.ends_with('Z')) {
            return n_steps;
        }
        let direction: usize = if directions[n_steps % directions.len()] == b'L' {
            0
        } else {
            1
        };
        current_node = nodes[current_node][direction];
    }
    unreachable!()
}

fn get_number_of_parallel_steps(directions: &[u8], nodes: &HashMap<&str, [&str; 2]>) -> u64 {
    // This implementation assumes that every parallel path hits a Z node at a constant interval
    // (this "Z interval" may be different for every parallel path).
    let start_nodes: Vec<&str> = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect();
    let z_intervals = start_nodes
        .iter()
        .map(|node| get_number_of_steps(node, "", directions, nodes) as u64);
    dbg!(z_intervals.collect::<Vec<u64>>());
    // z_intervals.reduce(lcm).unwrap()
    0
}

pub fn run(input: &str) {
    let mut parts = input.split("\n\n");
    let directions = parts.next().unwrap().as_bytes();
    let nodes: HashMap<_, _> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();
    let n_steps = get_number_of_steps("AAA", "ZZZ", directions, &nodes);
    println!("{}", n_steps);
    let n_steps = get_number_of_parallel_steps(directions, &nodes);
    println!("{}", n_steps)
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Need to specify a file name!");
        std::process::exit(1);
    }
    let filename = &args[1];
    let content: String = fs::read_to_string(filename).expect("Cannot open file!");
    // dbg!(part_one(&content));
    dbg!(run(&content));
}