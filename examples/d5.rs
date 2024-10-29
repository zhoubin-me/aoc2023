use std::{env, fs, vec};

use regex::Regex;

fn map_two(cur: &mut Vec<(u64, u64)>, map: &mut Vec<(u64, u64, u64, u64)>) -> Vec<(u64, u64)> {
    map.sort_by_key(|x| x.2);
    cur.sort_by_key(|x| x.0);

    let mut result = vec![];
    let (mut i, mut j) = (0, 0);
    loop {
        let (start, end) = cur[i];
        let (des_start, des_end, src_start, src_end) = map[j];
        if end < src_start {
            // cur totally on left of src
            // println!("Left: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            result.push((start, end));
            i += 1;
        } else if  start < src_start && end <= src_end {
            // cur overlap with left of src
            // println!("Left Overlap: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            result.push((start, src_start-1));
            result.push((des_start, des_start + end - src_start));
            i += 1;
        } else if start >= src_start && end <= src_end {
            // cur inside src
            // println!("Inside: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            result.push((des_start + start - src_start, des_start + end - src_start));
            i += 1;
        } else if start < src_start && end > src_end {
            // src inside cur
            // println!("Outside: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            result.push((start, start-1));
            result.push((des_start, des_end));
            cur[i].0 = src_end + 1;
            j += 1;
        } else if start >= src_start && start <= src_end && end > src_end {
            // cur overlap with right of src
            // println!("Right Overlap: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            result.push((des_start + start - src_start, des_end));
            cur[i].0 = src_end + 1;
            j += 1;
        } else if start > src_end {
            // println!("Right: {start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            j += 1;
        } else {
            println!("{start}, {end}, {src_start}, {src_end}, {des_start}, {des_end}");
            panic!("Impossible")
        }

        if i == cur.len() && j == map.len() {
            break;
        } else if i < cur.len() && j == map.len() {
            while i < cur.len() {
                result.push((cur[i].0, cur[i].1));
                i += 1;
            }
            break;
        } else if i == cur.len() && j < map.len() {
            break;
        }
    }

    result
    
}


fn part_two(content: &String) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut cur: Vec<(u64, u64)> = vec![];
    let mut map: Vec<(u64, u64, u64, u64)> = vec![];
    for line in content.lines() {
        if line.starts_with("seeds:") {
            let x = re.find_iter(line).map(
                |m| m.as_str().parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();

            for (i, e) in x.iter().enumerate() {
                if i % 2 == 1 {
                    cur.push((x[i-1], x[i-1] + e - 1))
                }
            }


        } else if line.contains("-to-") {
            map.clear();
        } else if line.trim().len() == 0 {
            if map.len() == 0 {
                continue;
            }
            cur = map_two(&mut cur, &mut map);
        } else {
            let x = re.find_iter(line).map(
                |m| m.as_str().parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();
            // (des_start, des_end, src_start, src_end)
            map.push((x[0], x[0] + x[2] - 1, x[1], x[1] + x[2] - 1))
        }
    }

    if map.len() > 0 {
        cur = map_two(&mut cur, &mut map);
    }

    cur.iter().min_by_key(|x| x.0).unwrap().0
}

fn map_one(cur: &mut Vec<u64>, map: &mut Vec<(u64, u64, u64)>) {
    map.sort_by_key(|x| x.1);

    for x in cur.iter_mut() {
        for (des, src, n) in map.iter() {
            if src <= x && *x < *src + *n {
                *x = *des + *x - *src;
                break;
            }
        }
    }
}

fn part_one(content: &String) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut cur: Vec<u64> = vec![];
    let mut map: Vec<(u64, u64, u64)> = vec![];
    for line in content.lines() {
        if line.starts_with("seeds:") {
            cur = re.find_iter(line).map(
                |m| m.as_str().parse::<u64>().unwrap()
            ).collect();
        } else if line.contains("-to-") {
            map.clear();
        } else if line.trim().len() == 0 {
            if map.len() == 0 {
                continue;
            }
            map_one(&mut cur, &mut map);
        } else {
            let x = re.find_iter(line).map(
                |m| m.as_str().parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();
            map.push((x[0], x[1], x[2]))
        }
    }

    if map.len() > 0 {
        map_one(&mut cur, &mut map);
    }

    *cur.iter().min().unwrap()
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