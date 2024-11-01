use std::{collections::HashMap, env, fs};

use regex::Regex;

fn part_one(content: &String) -> u32 {
    let num_re = Regex::new(r"\d+").unwrap();
    let sym_re = Regex::new(r"[^0-9\.]").unwrap();

    let mut nums = HashMap::new();
    let mut syms = HashMap::new();
    content.lines().enumerate().for_each(|(i, line)| {
        num_re.find_iter(line).for_each(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            nums.insert((i, m.start(), m.end()), num);
        });
        sym_re.find_iter(line).for_each(|m| {
            syms.insert((i, m.start()), m.as_str());
        });
    });


    nums.iter().filter_map(|(&(ni, ns, ne), v)|{
        let is_part = syms.iter().map(|(&(si, ss), _)| {
            ni.abs_diff(si) <= 1 && ns <= ss + 1 && ss <= ne
        }).any(|e| e);
        match is_part {
            true => Some(v),
            false => None
        }
    }).sum()
}


fn part_two(content: &String) -> u32 {
    let num_re = Regex::new(r"\d+").unwrap();
    let sym_re = Regex::new(r"\*").unwrap();

    let mut nums = HashMap::new();
    let mut syms = HashMap::new();
    content.lines().enumerate().for_each(|(i, line)| {
        num_re.find_iter(line).for_each(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            nums.insert((i, m.start(), m.end()), num);
        });
        sym_re.find_iter(line).for_each(|m| {
            syms.insert((i, m.start()), m.as_str());
        });
    });


    syms.iter().filter_map(|((si, ss), _)| {
        let gears = nums.iter().filter_map(|((ni, ns, ne), v)|{
            match ni.abs_diff(*si) <= 1 && *ns <= *ss + 1 && *ss <= *ne {
                true => Some(*v),
                false => None,
            }
        }).collect::<Vec<u32>>();

        match gears.len() {
            2 => Some(gears[0] * gears[1]),
            _ => None
        }
    }).sum()
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