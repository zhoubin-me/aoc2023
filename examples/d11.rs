
use std::env;
use std::fs;
use regex::Regex;

fn part_one(content: &String) -> i32 {

    let n = content.lines().count();
    let m = content.lines().nth(0).unwrap().len();

    let re = Regex::new(r"\#").unwrap();
    let mut pos: Vec<_> = re.find_iter(&content).map(
        |mc| {let idx = mc.start(); (idx / (m + 1), idx % (m + 1))}    
    ).collect();

    let rows: Vec<_> = (0..n).filter(|i|{
        pos.iter().all(|(e, _)| *e != *i)
    }).collect();
    let cols: Vec<_> = (0..m).filter(|i|{
        pos.iter().all(|(_, e)| *e != *i)
    }).collect();

    pos.iter_mut().map(
        |(x, y)| {
            *x += rows.iter().filter(|&e| *x > *e).count();
            *y += cols.iter().filter(|&e| *y > *e).count();
        }
    ).for_each(drop);

    let mut total = 0;
    for (i, &x) in pos.iter().enumerate() {
        for &y in pos.iter().skip(i+1) {
            total += (x.0 as i32 - y.0 as i32).abs() + (x.1 as i32 - y.1 as i32).abs();
        }
    }

    total
}

fn part_two(content: &String) -> u64 {
    let n = content.lines().count();
    let m = content.lines().nth(0).unwrap().len();

    let re = Regex::new(r"\#").unwrap();
    let mut pos: Vec<_> = re.find_iter(&content).map(
        |mc| {let idx = mc.start(); (idx / (m + 1), idx % (m + 1))}    
    ).collect();

    let rows: Vec<_> = (0..n).filter(|i|{
        pos.iter().all(|(e, _)| *e != *i)
    }).collect();
    let cols: Vec<_> = (0..m).filter(|i|{
        pos.iter().all(|(_, e)| *e != *i)
    }).collect();

    pos.iter_mut().map(
        |(x, y)| {
            *x += rows.iter().filter(|&e| *x > *e).count() * (1000000 - 1);
            *y += cols.iter().filter(|&e| *y > *e).count() * (1000000 - 1);
        }
    ).for_each(drop);

    let mut total = 0;
    for (i, &x) in pos.iter().enumerate() {
        for &y in pos.iter().skip(i+1) {
            total += (x.0 as i32 - y.0 as i32).abs() as u64 + (x.1 as i32 - y.1 as i32).abs() as u64;
        }
    }

    total
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