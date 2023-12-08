use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use num::Integer;


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


    if let Ok(mut lines) = read_lines(path) {
        let directions = lines.next()
            .unwrap()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let sequence = lines.filter_map(|l| {
            let line = l.unwrap();
            let (node, rest) = line.split_once(" = ")?;
            let (left, right) = rest.split_once(", ").map(|(a, b)| {
                let mut a = a.chars();
                let mut b = b.chars();
                a.next();
                b.next_back();
                (a.as_str(), b.as_str())
            })?;
            Some((node.to_string(), (left.to_string(), right.to_string())))
        }).collect::<HashMap<String, (String, String)>>();

        // main algorithm
        let solve = |starts: Vec<String>, ends: Vec<String>| -> Vec<u64> {
            let mut steps:Vec<u64> = Vec::new();

            for start in starts.iter() {
                let mut cnt:u64 = 0;
                let mut direction_iter = directions.iter().cycle();
                let mut pos = start.clone();
                while !ends.contains(&pos) {
                    let direction = direction_iter.next().unwrap();
                    if direction == &'L' {
                        pos = sequence.get(&pos).unwrap().0.clone();
                    } else {
                        pos = sequence.get(&pos).unwrap().1.clone();
                    }
                    cnt += 1;
                }
                steps.push(cnt);
            }
            steps
        };

        // part 1 inputs
        let starts = vec!["AAA".to_string()];
        let ends = vec!["ZZZ".to_string()];

        let steps = solve(starts, ends);
        let lcm = steps.iter().fold(steps[0], |a, b| {
            a.lcm(b)
        });

        println!("Part 1: {:?} -> {}", steps, lcm);

        // part 2 inputs
        let starts = sequence.keys().filter_map(|k| {
            if k.ends_with("A") {
                Some(k.clone())
            } else {
                None
            }
        }).collect::<Vec<String>>();
        let ends = sequence.keys().filter_map(|k| {
            if k.ends_with("Z") {
                Some(k.clone())
            } else {
                None
            }
        }).collect::<Vec<String>>();

        let steps = solve(starts, ends);
        let lcm = steps.iter().fold(steps[0], |a, b| {
            a.lcm(b)
        });
        println!("Part 2: {:?} -> {}", steps, lcm);
    }

}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
