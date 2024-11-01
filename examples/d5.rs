use std::{env, fs};
use regex::Regex;

fn map_two(seeds: &mut Vec<(u64, u64)>, map: &mut Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    map.sort_by_key(|&(_, x, _)| x);
    seeds.sort_by_key(|&(x, _)| x);
    let mut new_seeds: Vec<(u64, u64)> = vec![];
    let (mut i, mut j) = (0, 0);

    while i < seeds.len() && j < map.len() {
        let (start, m) = seeds.iter_mut().nth(i).unwrap();
        let (des, src, n) = map.iter().nth(j).unwrap();

        if *start + *m <= *src {
            new_seeds.push((*start, *m));
            i += 1;
        } else if *start + *m <= *src + *n {
            if *start >= *src {
                new_seeds.push((*des + *start - *src, *m));
            } else {
                new_seeds.push((*start, *src - *start));
                new_seeds.push((*des, *m - (*src - *start)));
            }
            i += 1;
        } else {
            if *start >= *src + *n {
                j += 1;
            } else if *start >= *src {
                new_seeds.push((*des + *start - *src, *src + *n - *start));
                *m -= *src + *n - *start;
                *start = *src + *n;
                j += 1;
            } else {
                new_seeds.push((*start, *src - *start));
                new_seeds.push((*des, *n));
                *m -= *src + *n - *start;
                *start = *src + *n;
                j += 1;
            }
        }
    }

    new_seeds.extend(seeds.iter().skip(i));
    new_seeds
}


fn part_two(content: &String) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let seeds = re.find_iter(content.lines().nth(0).unwrap()).map(
        |m| m.as_str().parse::<u64>().unwrap()
    ).collect::<Vec<u64>>();

    let mut seeds = seeds.iter().enumerate().filter_map(
        |(i, _)| match i % 2 == 1 {
            true => Some((seeds[i-1], seeds[i])),
            false => None
        }
    ).collect::<Vec<(u64, u64)>>();

    let mut map = vec![];
    content.lines().skip(2).for_each(|line|{
        match line {
            line if line.len() == 0 => {
                seeds = map_two(&mut seeds, &mut map);
            },
            line if line.contains("map") => {
                map.clear();
            }
            _ => {
                let nums: Vec<u64> = re.find_iter(line).map(
                    |m| m.as_str().parse::<u64>().unwrap()
                ).collect();
                map.push((nums[0], nums[1], nums[2]));
            }
            
        }
    });
    seeds.iter().map(|x| x.0).min().unwrap()
}


fn part_one(content: &String) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let mut seeds: Vec<u64> = re.find_iter(content.lines().nth(0).unwrap()).map(
        |m| m.as_str().parse::<u64>().unwrap()
    ).collect();
    let mut map = vec![];

    content.lines().skip(2).for_each(|line|{
        match line {
            line if line.len() == 0 => {
                seeds.iter_mut().for_each(|x| {
                    for &(des, src, n) in map.iter() {
                        if src <= *x && *x < src + n {
                            *x = des + (*x - src);
                            break;
                        }
                    }
                });
            },
            line if line.contains("map") => {
                map.clear();
            }
            _ => {
                let nums: Vec<u64> = re.find_iter(line).map(
                    |m| m.as_str().parse::<u64>().unwrap()
                ).collect();
                map.push((nums[0], nums[1], nums[2]));
            }
        }
    });

    *seeds.iter().min().unwrap()
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