use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use std::cmp::Ordering;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {},
        _ => {
            println!("Usage: main <file-path> <part> (1 or 2, default 1)");
            return;
        },
    }
    let path = &args[1];

    // card power order (strong to weak)
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

    if let Ok(lines) = read_lines(path) {

        let mut hands = lines.map(|l| {
            let l = l.unwrap();
            let mut iter = l.split_whitespace();
            // replace letters with alphabelitcally ordered to easy sorting
            let hand:String = iter.next().unwrap().to_string();
            let hand = hand.replace("A", "Z")
                        .replace("K", "Y")
                        .replace("T", "I");
            let hand_jokers = hand.to_string().replace("J", "1");
            let bid: usize = iter.next().unwrap().parse().unwrap();
            (hand, hand_jokers, bid)
        }).collect::<Vec<(String, String, usize)>>();

        hands.sort_by(|a,b| main_sort(&a.0, &b.0));

        let mut total = 0;
        for (i, (_hand , _ , bid)) in hands.iter().enumerate() {
            // println!("{}: {} -> {}", i+1, _hand, bid);
            total += bid*(i+1);
        }

        hands.sort_by(|a,b| main_sort(&a.1, &b.1));

        let mut total_jokers = 0;
        for (i, (_, _hand , bid)) in hands.iter().enumerate() {
            // println!("{}: {} -> {}", i+1, _hand, bid);
            total_jokers += bid*(i+1);
        }
        println!("Total: {}", total);
        println!("Total with jokers: {}", total_jokers);
    }
}

fn main_sort(a:&str, b:&str) -> Ordering {


    let dict_a:HashMap<char,u8> = gen_sort_map(a);
    let dict_b:HashMap<char,u8> = gen_sort_map(b);
    if dict_a.len() < dict_b.len() {
        return Ordering::Greater;
    } else if dict_a.len() > dict_b.len() {
        return Ordering::Less;
    } else {
        let a_max = dict_a.values().max().unwrap();
        let b_max = dict_b.values().max().unwrap();
        match a_max.cmp(b_max) {
            Ordering::Equal => {
                return a.cmp(b);
            },
            non_equal => {
                return non_equal;
            },
        }
    }
}

fn gen_sort_map(a:&str) -> HashMap<char,u8> {
    let mut map:HashMap<char,u8> = HashMap::new();
    // count chars of each type
    for c in a.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    if let Some(jokers) = map.remove(&'1') {
        for item in map.values_mut() {
            *item += jokers;
        }
        // handle the case where there are only jokers, which
        // would result in an empty map, which would win above
        // all 5-of-a-kind by length
        if map.is_empty() {
            map.insert('Z', jokers);
        }
    }
    map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
