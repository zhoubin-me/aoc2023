use std::env;
use std::fmt::Display;
use std::fs;

fn print_matrix<T: Display>(mat: &Vec<Vec<T>>) {
    let mut s = "".to_string();
    for line in mat.iter() {
        for x in line.iter() {
            s += format!("{:5}", x).as_str();
        }
        s.push_str("\n");
    }

    println!("{s}");

}

fn part_one(content: &String) -> u32 {
    let grid: Vec<Vec<char>> = content.lines().map(
        |line| {
            line.chars().collect::<Vec<char>>()
        }
    ).collect();

    let (n, m) = (grid.len(), grid[0].len());
    let idx = content.find('S').unwrap();


    let start = (idx / (m + 1), idx % (m + 1));
    let mut visited = vec![vec![0; m]; n];
    let mut to_visit: Vec<((usize, usize), char, char, u32)> = ['U', 'D', 'L', 'R'].iter().map(
        | &dir | (start, 'S', dir, 0)
    ).collect();

    while to_visit.len() > 0 {
        let ((i, j), cur, dir, count) = to_visit.remove(0);
        visited[i][j] = count + 1;
        match (cur, dir) {
            ('|', 'U') | ('S', 'U') | ('L', 'L') | ('J', 'R') => {
                if i > 0 && visited[i-1][j] == 0 {
                    let next = grid[i-1][j];
                    match next {
                        'F' | '7' | '|' => to_visit.push(((i-1, j), next, 'U', count+1)),
                        _ => {}
                    }
                }
            }
            ('|', 'D') | ('S', 'D') | ('F', 'L') | ('7', 'R') => {
                if i < n - 1 && visited[i+1][j] == 0 {
                    let next = grid[i+1][j];
                    match next {
                        'L' | 'J' | '|' => to_visit.push(((i+1, j), next, 'D', count+1)),
                        _ => {}
                    }
                }
            }
            ('-', 'L') | ('S', 'L') | ('J', 'D') | ('7', 'U') => {
                if j > 0 && visited[i][j-1] == 0 {
                    let next = grid[i][j-1];
                    match next {
                        'L' | 'F' | '-' => to_visit.push(((i, j-1), next, 'L', count+1)),
                        _ => {}
                    }
                }
            }
            ('-', 'R') | ('S', 'R') | ('F', 'U') | ('L', 'D') => {
                if j < m - 1 && visited[i][j+1] == 0 {
                    let next = grid[i][j+1];
                    match next {
                        '7' | 'J' | '-' => to_visit.push(((i, j+1), next, 'R', count+1)),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    print_matrix(&visited);
    *visited.iter().flat_map(|row| row.iter()).max().unwrap_or(&0) - 1
}

fn part_two(content: &String) -> u32 {
    let grid: Vec<Vec<char>> = content.lines().map(
        |line| {
            line.chars().collect::<Vec<char>>()
        }
    ).collect();

    let (n, m) = (grid.len(), grid[0].len());
    let idx = content.find('S').unwrap();


    let start = (idx / (m + 1), idx % (m + 1));
    let mut visited = vec![vec![0; m]; n];
    let mut to_visit: Vec<((usize, usize), char, char, u32)> = ['D', 'L', 'U', 'R'].iter().map(
        | &dir | (start, 'S', dir, 0)
    ).collect();


    while to_visit.len() > 0 {
        let ((i, j), cur, dir, count) = to_visit.pop().unwrap();
        // dbg!(&to_visit);
        visited[i][j] = count + 1;
        let cell = (cur, dir);
        match cell {
            ('|', 'U') | ('S', 'U') | ('L', 'L') | ('J', 'R') => {
                // visited[i][j] = char::from_u32(0x2191).unwrap();
                if i > 0 && visited[i-1][j] == 0 {
                    let next = grid[i-1][j];
                    match next {
                        'F' | '7' | '|' => to_visit.push(((i-1, j), next, 'U', count+1)),
                        'S' => break,
                        _ => {}
                    }
                }
            }
            ('|', 'D') | ('S', 'D') | ('F', 'L') | ('7', 'R') => {
                // visited[i][j] = char::from_u32(0x2193).unwrap();
                if i < n - 1 && visited[i+1][j] == 0 {
                    let next = grid[i+1][j];
                    match next {
                        'L' | 'J' | '|' => to_visit.push(((i+1, j), next, 'D', count+1)),
                        'S' => break,
                        _ => {}
                    }
                }
            }
            ('-', 'L') | ('S', 'L') | ('J', 'D') | ('7', 'U') => {
                // visited[i][j] = char::from_u32(0x2190).unwrap();
                if j > 0 && visited[i][j-1] == 0 {
                    let next = grid[i][j-1];
                    match next {
                        'L' | 'F' | '-' => to_visit.push(((i, j-1), next, 'L', count+1)),
                        'S' => break,
                        _ => {}
                    }
                }
            }
            ('-', 'R') | ('S', 'R') | ('F', 'U') | ('L', 'D') => {
                // visited[i][j] = char::from_u32(0x2192).unwrap();
                if j < m - 1 && visited[i][j+1] == 0 {
                    let next = grid[i][j+1];
                    match next {
                        '7' | 'J' | '-' => to_visit.push(((i, j+1), next, 'R', count+1)),
                        'S' => break,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    // print_matrix(&visited);
    let mut total = 0;
    for (i, row) in visited.iter().enumerate() {
        let mut inside = false; 
        for (j, &cell) in row.iter().enumerate() {
            let c = grid[i][j];
            if cell > 0 && (c == '|' || c == 'J' || c == 'L') {
                inside = !inside;
            } else if cell == 0 && inside {
                print!("({} {}),", i, j);
                total += 1;
            }
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