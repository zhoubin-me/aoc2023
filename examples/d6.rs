
use std::env;
use std::fs;
use regex::Regex;


fn find_ways(data: &Vec<Vec<u64>>) -> u64 {
    let mut result = 1;
    for (&time, &distance) in data[0].iter().zip(data[1].iter()) {
        // (time - n) * n > distance
        // n*n - time * n + distance < 0
        // delta = time * time - 4 * distance > 0
        if time * time < 4 * distance {
            return 0;
        } else if time * time == 4 * distance {
            result *= 1;
        } else {
            let delta: f64 = (time * time) as f64 - 4.0 * (distance as f64);
            if delta > 0.0 {
                let sqrt_delta = delta.sqrt();
                let mut root1 = ((time as f64 - sqrt_delta) / 2.0).ceil() as u64;
                let mut root2 = ((time as f64 + sqrt_delta) / 2.0).floor() as u64;
                // You can use root1 and root2 as needed
                if (time - root1) * root1 == distance {
                    root1 += 1;
                }

                if (time - root2) * root2 == distance {
                    root2 -= 1;
                }

                result *= root2 - root1 + 1
            }
        }
    }
    result
}

fn part_one(content: &String) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut data = vec![];
    for line in content.lines() {
        let nums = re.find_iter(line).filter_map(
            |m| m.as_str().parse::<u64>().ok()
        ).collect::<Vec<u64>>();
        data.push(nums);
    }
    
    find_ways(&data)
}

fn part_two(content: &String) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut data = vec![];
    for line in content.lines() {
        let num = line.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap();
        data.push(vec![num]);
    }
    
    find_ways(&data)
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