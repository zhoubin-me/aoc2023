
use std::env;
use std::fs;

use regex::Regex;

fn part_one(content: &String) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    content.lines().map(
        |line| {
            let mut nums: Vec<i64> = re.find_iter(line).map(
                |m| 
                m.as_str().parse::<i64>().unwrap()
            ).collect();

            let mut result = 0;
            while !nums.iter().all(|&x| x == 0) {
                result += nums.last().unwrap();
                nums = nums.iter_mut().scan(0, |state, x| {
                    let y = *x - *state;
                    *state = *x;
                    Some(y)
                }).skip(1).collect();
            }

            result
        }
    ).sum()
}

fn part_two(content: &String) -> i64 {
    let re = Regex::new(r"-?\d+").unwrap();
    content.lines().map(
        |line| {
            let mut nums: Vec<i64> = re.find_iter(line).map(
                |m| 
                m.as_str().parse::<i64>().unwrap()
            ).collect();

            let mut result = vec![];
            while !nums.iter().all(|&x| x == 0) {
                result.push(nums.first().cloned().unwrap());
                nums = nums.iter_mut().scan(0, |state, x| {
                    let y = *x - *state;
                    *state = *x;
                    Some(y)
                }).skip(1).collect();
            }
            result.iter().rev().fold(0, |state, x| x - state)
        }
    ).sum()
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