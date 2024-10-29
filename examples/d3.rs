use std::{fs, env};

use regex::Regex;

fn part_one(content: &String) -> u32 {
    let mut result = 0;

    let digits: Regex = Regex::new(r"(\d+)").unwrap();
    let sym = Regex::new(r"[^0-9\.]+").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let n = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let m: usize = line.trim().len();
        for x in digits.find_iter(line)
        {

            let start = if x.start() == 0 {0} else {x.start()-1};
            let end = if x.end() == m {m} else {x.end() + 1};
            let top = if i == 0 {0} else {i-1};
            let bottom = if i == n-1 {n-1} else {i+1};

            let num = x.as_str().parse::<u32>().unwrap();
            let up = sym.find(&lines[top][start..end]);
            let down = sym.find(&lines[bottom][start..end]);
            let middle = sym.find(&lines[i][start..end]);


            if up.is_some() || down.is_some() || middle.is_some() {
                result += num;
            }
        }
    }
    result
}


fn part_two(content: &String) -> u32 {
    let mut result = 0;

    let digits = Regex::new(r"(\d+)").unwrap();
    let stars = Regex::new(r"(\*)+").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut xs = vec![];
    let mut ys = vec![];
    for &line in lines.iter() {

        let mut x = vec![];
        for m in stars.find_iter(line)
        {
            x.push(m.start());
        }
        xs.push(x);

        let mut y = vec![];
        for m in digits.find_iter(line) {
            y.push(m);
        }
        ys.push(y);
    }

    for (i, x) in xs.iter().enumerate() {
        for &j in x {
            let mut nums = vec![];
            for m in ys[i].iter_mut() {
                if m.start() == j + 1 || m.end() == j {
                    nums.push(m.as_str().parse::<u32>().unwrap());
                }
            }

            if i > 0 {
                for m in ys[i-1].iter_mut() {
                    if m.start() <= j && j < m.end() || m.start() == j + 1 || m.end() == j  {
                        nums.push(m.as_str().parse::<u32>().unwrap());
                    }
                }
            }

            if i < lines.len() - 1 {
                for m in ys[i+1].iter_mut() {
                    if m.start() <= j && j < m.end() || m.start() == j + 1 || m.end() == j  {
                        nums.push(m.as_str().parse::<u32>().unwrap());
                    }
                }
            }
            
            if nums.len() == 2 {
                result += nums[0] * nums[1];
            }
        }
    }

    result
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