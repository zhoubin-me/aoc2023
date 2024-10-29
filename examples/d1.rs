
use std::env;
use std::fs;

fn part_one(content: &String) -> u32 {
    let mut result = 0;
    for line in content.lines() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                match c.to_digit(10) {
                    Some(x) => {result += 10 * x; break;},
                    None => {continue;}
                }
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                match c.to_digit(10) {
                    Some(x) => {result += x; break},
                    None => {continue;}
                }
            }
        }
    }

    result
}

fn part_two(content: &String) -> u32 {
    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut result = 0;

    for line in content.lines() {
        'first: for start in 0..line.len() {
            for (i, &digit) in digits.iter().enumerate() {
                if line[start..].starts_with(digit) {
                    result += 10 * (i % 9 + 1);
                    break 'first;
                }
            }
        }

        'last: for end in (0..line.len()).rev() {
            for (i, &digit) in digits.iter().enumerate() {
                if line[..=end].ends_with(digit) {
                    result += i % 9 + 1;
                    break 'last;
                }
            }
        }
    }

    result as u32

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