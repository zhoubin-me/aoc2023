
use std::env;
use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn part_one(content: &String) -> u32 {
    let mut result = 0;
    let digits = Regex::new(r"(\d+)").unwrap();
    for line in content.lines() {
        let mut cache = HashSet::new();
        let mut count = 0;
        let pos = line.find("|").unwrap();
        for (i, m) in digits.find_iter(line).enumerate() {
            if i == 0 {
                continue;
            }
            let x = m.as_str().parse::<u32>().unwrap();
            if m.end() < pos {
                cache.insert(x);
            } else {
                if cache.contains(&x) {
                    count += 1;
                }
            }
        }

        if count > 0 {
            result += 2u32.pow(count - 1);
        }
    }

    result
}

fn part_two(content: &String) -> u32 {
    let digits = Regex::new(r"(\d+)").unwrap();
    let n = content.lines().count();
    let mut counts = vec![];
    counts.resize(n, 1u32);
    for (i, line) in content.lines().enumerate() {
        let mut cache = HashSet::new();
        let mut count = 0;
        let pos = line.find("|").unwrap();
        for (i, m) in digits.find_iter(line).enumerate() {
            if i == 0 {
                continue;
            }
            let x = m.as_str().parse::<u32>().unwrap();
            if m.end() < pos {
                cache.insert(x);
            } else {
                if cache.contains(&x) {
                    count += 1;
                }
            }
        }

        for j in i+1..(i+1+count as usize) {
            if j < n {
                counts[j] += counts[i];
            }
        }
    }

    counts.iter().sum()

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