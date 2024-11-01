use std::{fs, env};

use regex::Regex;

fn part_one(content: &String, rgb: (u32, u32, u32)) -> u32 {
    let game_re = Regex::new(r"(\d+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (\w+)").unwrap();
    content.lines().filter_map(
        |line| {
            let game_id = game_re.find_iter(line).map(
                |c| c.as_str().parse::<u32>()
            ).next().unwrap().unwrap();

            let is_possible = cube_re.captures_iter(line).map(
                |c| {
                    let (_, [num, color]) = c.extract();
                    let x = num.parse::<u32>().unwrap();
                    match color {
                        "red" => x <= rgb.0,
                        "green" => x <= rgb.1,
                        "blue" => x <= rgb.2,
                        _ => unreachable!(),
                    }
                }
            ).all(|e| e);
            match is_possible {
                true => Some(game_id),
                false => None
            }
        }
    ).sum()
}


fn part_two(content: &String) -> u32 {

    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    content.lines().map(
        |line| {
            let mut rgb: (u32, u32, u32) = (0, 0, 0);
            re.captures_iter(line).for_each(
                |c| {
                    let (_, [num, color]) = c.extract();
                    let x = num.parse::<u32>().unwrap();
                    match (color, x) {
                        ("red", x) if x > rgb.0 => rgb.0 = x,
                        ("blue", x) if x > rgb.1 => rgb.1 = x,
                        ("green", x) if x > rgb.2 => rgb.2 = x,
                        _ => ()
                    };
                }
            );
            rgb.0 * rgb.1 * rgb.2
        }
    ).sum()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Need to specify a file name!");
        std::process::exit(1);
    }
    let filename = &args[1];
    let content: String = fs::read_to_string(filename).expect("Cannot open file!");
    dbg!(part_one(&content, (12, 13, 14)));
    dbg!(part_two(&content));
}