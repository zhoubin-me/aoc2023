use std::{fs, env};

use regex::Regex;

fn part_one(content: &String, rgb: (u32, u32, u32)) -> u32 {
    let mut result = 0;

    for line in content.lines() {
        let re: Regex = Regex::new(r"Game (\d+)").unwrap();
        let game_id = re.captures_iter(line).map(
            |c| c.extract()
        ).map(
            |(_, [num])| num.parse::<u32>().unwrap()
        ).next().unwrap();

        let re = Regex::new(r"(\d+) (\w+)").unwrap();

        let mut possible = true;
        for (_, [num, color]) in re.captures_iter(line).map(|c| c.extract()) {
            let x = num.parse::<u32>().expect("Cannot parse number of cubes");
            let is_larger = match color {
                "red" => x > rgb.0,
                "green" => x > rgb.1,
                "blue" => x > rgb.2,
                _ => panic!("Invalid color name")
            };

            if is_larger {
                possible = false;
                break;
            }
        }

        if possible {
            result += game_id;
        }
    }

    result
}


fn part_two(content: &String) -> u32 {

    let mut result = 0;
    
    for line in content.lines() {
        let mut rgb = (0, 0, 0);
        let re = Regex::new(r"(\d+) (\w+)").unwrap();
        for (_, [num, color]) in re.captures_iter(line).map(|c| c.extract()) {
            let x = num.parse::<u32>().expect("Cannot parse number of cubes");
            match color {
                "red" => if rgb.0 < x {rgb.0 = x;},
                "green" => if rgb.1 < x {rgb.1 = x;},
                "blue" => if rgb.2 < x {rgb.2 = x},
                _ => panic!("Invalid color name")
            };
        }
        // dbg!(rgb.0 * rgb.1 * rgb.2);
        result += rgb.0 * rgb.1 * rgb.2;
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
    dbg!(part_one(&content, (12, 13, 14)));
    dbg!(part_two(&content));
}