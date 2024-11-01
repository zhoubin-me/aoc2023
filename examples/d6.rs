
use std::env;
use std::fs;
use regex::Regex;

fn find_ways(time: Vec<u64>, distance: Vec<u64>) -> u64 {
    time.iter().zip(distance.iter()).map(|(&t, &d)|{
        match (t*t, 4*d) {
            (x, y) if x < y => 0,
            (x, y) if x == y => 1,
            (x, y) => {
                let sqrt_delta = ((x - y) as f64).sqrt();
                let mut r1 =  ((t as f64 - sqrt_delta) / 2.0).ceil() as u64;
                let mut r2 = ((t as f64 + sqrt_delta) / 2.0).floor() as u64;
                if (t - r1) * r1 == d { r1 += 1; }
                if (t - r2) * r2 == d { r2 -= 1; }
                r2 - r1 + 1
            }
        }
    }).fold(1, |succ, e| succ * e)
}

fn part_one(content: &String) -> u64 {
    let re = Regex::new(r"(\d+)").unwrap();
    let time: Vec<u64> = re.find_iter(content.lines().nth(0).unwrap()).map(
        |m | m.as_str().parse::<u64>().unwrap()
    ).collect();
    let distance: Vec<u64> = re.find_iter(content.lines().nth(1).unwrap()).map(
        |m | m.as_str().parse::<u64>().unwrap()
    ).collect();
    find_ways(time, distance)
}

fn part_two(content: &String) -> u64 {
    let time = content.lines().nth(0).unwrap().chars().filter(
        |c| c.is_ascii_digit()
    ).collect::<String>().parse::<u64>().unwrap();

    let distance = content.lines().nth(1).unwrap().chars().filter(
        |c| c.is_ascii_digit()
    ).collect::<String>().parse::<u64>().unwrap();

    find_ways(vec![time], vec![distance])
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