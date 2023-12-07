use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

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
        // get time vector
        let times = match lines.next() {
            Some(Ok(l)) => {
                let (_, times) = l.split_once(":").unwrap();
                let times = times.trim().split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                times
            },
            _ => {
                println!("Parsing error");
                return;
            }
        };

        // get distance vector
        let distances = match lines.next() {
            Some(Ok(l)) => {
                let (_, distances) = l.split_once(":").unwrap();
                let distances = distances.trim().split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                distances
            },
            _ => {
                println!("Parsing error");
                return;
            }
        };

        let mut mul = 1;
        for (t,d) in times.iter().zip(distances.iter()) {
            let mut win_options:usize = 0;
            for c in 1..*t-1 {
                let distance = get_distance(c, *t);
                if distance > *d {
                    win_options = win_options + 1;
                }
            }
            mul = mul * win_options;
        }
        println!("Win options prod: {}", mul);
        println!();

        // part 2 underestimates the speed of my compiled code, just grind it out in a 2 seconds
        // concatenate numbers in times into a string and parse it to usize
        let time = times.iter().map(|n| n.to_string()).collect::<String>().parse::<usize>().unwrap();
        let distance = distances.iter().map(|n| n.to_string()).collect::<String>().parse::<usize>().unwrap();

        println!("Concat Time: {}", time);
        println!("Concat Distance: {}", distance);
        let mut win_options:usize = 0;
        for c in 1..time-1 {
            let d = get_distance(c, time);
            if d > distance {
                win_options = win_options + 1;
            }
        }
        println!("Win options: {}", win_options);
    }
}

fn get_distance(charge_time:usize, total_time: usize) -> usize {
    let runtime = total_time - charge_time;
    let speed = charge_time;

    let distance = speed * runtime;
    distance
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
