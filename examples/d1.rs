
use std::env;
use std::fs;

fn part_one(content: &String) -> u32 {
    content.lines().map(
        |line| {
            let first = line.chars().filter_map(
                |c| c.to_digit(10)
            ).nth(0).unwrap();

            let second = line.chars().rev().filter_map(
                |c| c.to_digit(10)
            ).nth(0).unwrap();

            first * 10 + second
        }
    ).sum()
}

fn part_two(content: &String) -> u32 {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    content.lines().map(
        |line| {
            let first = digits.iter().enumerate().filter_map(
                |(i, &x)| {
                    match line.find(x) {
                        Some(idx) => Some((idx, i)),
                        None => None,
                    }
                }
            ).min_by_key(|&(idx, _)| idx).unwrap();

            let second = digits.iter().enumerate().filter_map(
                |(i, &x)| {
                    match line.rfind(x) {
                        Some(idx) => Some((idx + x.len(), i)),
                        None => None
                    }
                }
            ).max_by_key(|&(idx, _)| idx).unwrap();

            10 * (first.1 as u32 % 9 + 1) + second.1 as u32 % 9 + 1
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