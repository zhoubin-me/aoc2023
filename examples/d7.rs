
use std::cmp::Ordering;
use std::env;
use std::fs;
use indexmap::IndexMap;


#[derive(PartialOrd, PartialEq, Debug)]
enum HandOrder {
    FiveKind = 0,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPairs,
    OnePair,
    HighCard
}

fn compare(a: &(HandOrder, String, u32), b: &(HandOrder, String, u32)) -> Ordering {
    match (a, b) {
        (a, b) if a.0 < b.0                 => Ordering::Greater,
        (a, b) if a.0 > b.0                 => Ordering::Less,
        (a, b) if a.0 == b.0 && a.1 < b.1   => Ordering::Greater,
        (a, b) if a.0 == b.0 && a.1 > b.1   => Ordering::Less,
        _ => panic!("{:?}, {:?}", a, b)
    }
}

fn part_one(content: &String) -> u32 {
    let card_order = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'
    ].iter().enumerate().map(|(i, &c)| (c, (i as u8 + 'A' as u8) as char)).collect::<IndexMap<char, char>>();
    let mut results: Vec<(HandOrder, String, u32)> = content.lines().map(|line| {
        let bids = line.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
        let cards = line.split(' ').nth(0).unwrap().chars().filter_map(
            |c| card_order.get(&c).copied()
        ).collect::<String>();

        let mut freqs = cards.chars().fold(
            IndexMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1u8;
                map
            }
        );
        freqs.sort_by(|_, v1, _, v2| v1.cmp(v2));
        freqs.reverse();
        let order = match freqs.values().collect::<Vec<&u8>>() {
            v if v.len() == 1                   => HandOrder::FiveKind,     // five of a kind
            v if v.len() == 2 && *v[0] == 4     => HandOrder::FourKind,     // four of a kind
            v if v.len() == 2 && *v[0] == 3     => HandOrder::FullHouse,    // full house
            v if v.len() == 3 && *v[0] == 3     => HandOrder::ThreeKind,    // three of a kind
            v if v.len() == 3 && *v[0] == 2     => HandOrder::TwoPairs,     // two pairs
            v if v.len() == 4                   => HandOrder::OnePair,      // one pair
            v if v.len() == 5                   => HandOrder::HighCard,     // high card
            v => panic!("Impossible {:?}", v),
        };
        (order, cards, bids)
    }).collect();

    results.sort_by(|a: &(HandOrder, String, u32), b: &(HandOrder, String, u32)| compare(a, b));
    results.iter().enumerate().map(|(i, e)| (i + 1) as u32 * e.2).sum()
}

fn part_two(content: &String) -> u32 {
    let card_order = vec![
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    ].iter().enumerate().map(|(i, &c)| (c, (i as u8 + 'A' as u8) as char)).collect::<IndexMap<char, char>>();
    let mut results: Vec<(HandOrder, String, u32)> = content.lines().map(|line| {
        let bids = line.split(' ').nth(1).unwrap().parse::<u32>().unwrap();
        let cards = line.split(' ').nth(0).unwrap().chars().filter_map(
            |c| card_order.get(&c).copied()
        ).collect::<String>();

        let mut freqs = cards.chars().fold(
            IndexMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1u8;
                map
            }
        );
        freqs.sort_by(|_, v1, _, v2| v1.cmp(v2));
        if let Some(x) = freqs.swap_remove(&'M') {
            freqs.sort_by(|_, v1, _, v2| v1.cmp(v2));
            match  freqs.last_mut() {
                Some((_, v)) => *v += x,
                _ =>  {freqs.insert('M', 5);}
            };
        }
        freqs.reverse();
        let order = match freqs.values().collect::<Vec<&u8>>() {
            v if v.len() == 1                   => HandOrder::FiveKind,     // five of a kind
            v if v.len() == 2 && *v[0] == 4     => HandOrder::FourKind,     // four of a kind
            v if v.len() == 2 && *v[0] == 3     => HandOrder::FullHouse,    // full house
            v if v.len() == 3 && *v[0] == 3     => HandOrder::ThreeKind,    // three of a kind
            v if v.len() == 3 && *v[0] == 2     => HandOrder::TwoPairs,     // two pairs
            v if v.len() == 4                   => HandOrder::OnePair,      // one pair
            v if v.len() == 5                   => HandOrder::HighCard,     // high card
            v => panic!("Impossible {:?}", v),
        };
        (order, cards, bids)
    }).collect();

    results.sort_by(|a: &(HandOrder, String, u32), b: &(HandOrder, String, u32)| compare(a, b));
    results.iter().enumerate().map(|(i, e)| (i + 1) as u32 * e.2).sum()
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