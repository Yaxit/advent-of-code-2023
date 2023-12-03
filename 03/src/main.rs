use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use regex::Regex;

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

        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => {
                println!("Error parsing input file");
                return;
            },
        };
        let mut prev_symbols = find_symbols(&line);
        let mut prev_numbers = find_numbers(&line);

        let mut sum = 0;

        for line in lines {
            if let Ok(l) = line {
                let symbols = find_symbols(&l);
                let numbers = find_numbers(&l);
                for sym_pos in symbols.iter() {
                    for (num_start, num_end, num) in numbers.iter() {
                        if is_adjacent(*sym_pos, (*num_start, *num_end)) {
                            // println!("current: {}", num);
                            sum += num;
                        }
                    }
                    for (num_start, num_end, num) in prev_numbers.iter() {
                        if is_adjacent(*sym_pos, (*num_start, *num_end)) {
                            // println!("previous nums: {}", num);
                            sum += num;
                        }
                    }
                }

                for sym_pos in prev_symbols.iter() {
                    for (num_start, num_end, num) in numbers.iter() {
                        if is_adjacent(*sym_pos, (*num_start, *num_end)) {
                            // println!("previous symb: {}", num);
                            sum += num;
                        }
                    }
                }
                prev_symbols = symbols;
                prev_numbers = numbers;

            }
        }
        println!("Sum: {}", sum);
    }
}

fn find_symbols(line: &str) -> Vec<usize>{
    let re = Regex::new(r"[^\w\s.]").unwrap();
    re.find_iter(line).map(|m| m.start()).collect()

}

fn find_numbers(line: &str) -> Vec<(usize, usize, u32)>{
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line).map(|m| (m.start(), m.end()-1, m.as_str().parse::<u32>().unwrap())).collect()
}

fn is_adjacent(sym_pos: usize, number_range: (usize, usize)) -> bool {
    // check if symbol is in range of number
    if sym_pos+1 >= number_range.0 && sym_pos <= number_range.1+1 {
        return true
    }
    false
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
