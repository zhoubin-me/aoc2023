
use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs;

use regex::Regex;
use rayon::prelude::*;

fn part_one(content: &String) -> u32 {

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let map = re.captures_iter(content).map(
        |c| c.extract()
    ).map(
        |(_, [name, left, right])| (name, (left, right))
    ).collect::<HashMap<&str, (&str, &str)>>();
    let instruction = content.lines().nth(0).unwrap().trim();
    let mut cur = "AAA";
    let mut itr = instruction.chars();
    let mut count = 0;
    loop {
        match itr.next() {
            Some('R') => {cur = map.get(&cur).unwrap().1; 'R'},
            Some('L') => {cur = map.get(&cur).unwrap().0; 'L'},
            None => {
                // println!("_");
                itr = instruction.chars(); 
                continue;
            },
            _ => panic!("Not possible")
        };

        count += 1;
        println!("{} {}", count, cur);
        if cur == "ZZZ" {
            break;
        }
    }

    count
    
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
}

fn lcm(n: u64, m: u64) -> u64 {
    (n * m) / gcd(n, m)
}

fn part_two(content: &String) -> u64 {
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let map = re.captures_iter(content).map(
        |c| c.extract()
    ).map(
        |(_, [name, left, right])| (name, (left, right))
    ).collect::<HashMap<&str, (&str, &str)>>();
    let instruction = content.lines().nth(0).unwrap().trim();
    let n = instruction.len();
    let itr: Vec<char> = instruction.chars().collect();
    let mut cur: Vec<&str> = map.keys().filter(|k| k.ends_with("A")).map(|&k| k).collect();
    // dbg!(&cur);

    cur.iter_mut().map(
        |e| {
            for step in 0.. {
                match itr[step % n] {
                    'R' => *e = map.get(*e).unwrap().1,
                    'L' => *e = map.get(*e).unwrap().0,
                    _ => panic!("Impossible")
                };
                if e.ends_with("Z") {
                    return (step + 1) as u64;
                }
            }
            0 as u64
        }
    ).reduce(lcm).unwrap()
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
    dbg!(part_two(&content));
}