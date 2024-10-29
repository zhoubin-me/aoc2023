
use std::env;
use std::fs;

use regex::Regex;

fn part_one(content: &String) -> u32 {
    0
}

fn part_two(content: &String) -> u32 {
    0
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Need to specify a file name!");
        std::process::exit(1);
    }
    let filename = &args[1];
    let content: String = fs::read_to_string(filename).expect("Cannot open file!");
    dbg!(part_one(&content));
    dbg!(part_two(&content));
}