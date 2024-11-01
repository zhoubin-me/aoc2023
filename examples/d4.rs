
use std::env;
use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn part_one(content: &String) -> u32 {
    let re = Regex::new(r"(\d+)").unwrap();
    content.lines().filter_map(|line|{
        let nums: Vec<u32> = re.find_iter(line).map(|m| 
            m.as_str().parse::<u32>().unwrap()).collect();
        let wins: HashSet<_> = nums.iter().skip(1).take(10).collect();
        let count: usize = nums.iter().skip(11).filter(|&x| wins.contains(x)).count();
        match count {
            0 => None,
            _ => Some(2u32.pow(count as u32 - 1))
        }
    }).sum()
}

fn part_two(content: &String) -> u32 {
    let re = Regex::new(r"(\d+)").unwrap();
    let matches: Vec<usize> = content.lines().map(|line|{
        let nums: Vec<u32> = re.find_iter(line).map(|m| 
            m.as_str().parse::<u32>().unwrap()).collect();
        let wins: HashSet<_> = nums.iter().skip(1).take(10).collect();
        nums.iter().skip(11).filter(|&x| wins.contains(x)).count()
    }).collect();

    let mut count = vec![1; matches.len()];

    for (i, x) in matches.iter().enumerate() {
        let y = count[i];
        for j in count.iter_mut().skip(i+1).take(*x) {
            *j += y;
        }
    }

    count.iter().sum()
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