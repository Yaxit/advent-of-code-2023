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

        let line_t2 = unpack_line(lines.next());
        let line_t1 = unpack_line(lines.next());

        let mut line_window = (line_t1, line_t2, String::new());

        let mut sum_components = 0;
        let mut sum_gear = 0;

        for line in lines {
            if let Ok(l) = line {
                // shift lines
                line_window.2 = line_window.1;
                line_window.1 = line_window.0;
                line_window.0 = l;

                let symbols_t1 = find_symbols(&line_window.1);

                let numbers_t2 = find_numbers(&line_window.2);
                let mut numbers_t1 = find_numbers(&line_window.1);
                let mut numbers_t0 = find_numbers(&line_window.0);

                let mut numbers = numbers_t2;

                numbers.append(&mut numbers_t1);
                // find adjacent numbers
                for sym_pos in symbols_t1.iter() {
                    for (num_start, num_end, num) in numbers.iter() {
                        if is_adjacent(sym_pos.0, (*num_start, *num_end)) {
                            sum_components += num;
                        }
                    }
                }

                // find adjacent symbols
                for sym_pos in symbols_t1.iter() {
                    for (num_start, num_end, num) in numbers_t0.iter() {
                        if is_adjacent(sym_pos.0, (*num_start, *num_end)) {
                            sum_components += num;
                        }
                    }
                }

                // squash all numbers
                numbers.append(&mut numbers_t0);
                for sym_pos in symbols_t1.iter() {
                    if sym_pos.1 != '*' {
                        continue;
                    }
                    let mut adj_cnt = 0;
                    let mut sym_power = 1;
                    for (num_start, num_end, num) in numbers.iter() {
                        if is_adjacent(sym_pos.0, (*num_start, *num_end)) {
                            adj_cnt += 1;
                            sym_power *= num;
                        }
                    }
                    if adj_cnt ==  2 {
                        sum_gear += sym_power;
                    }
                }
            }
        }
        println!("Sum Part Numbers: {}", sum_components);
        println!("Sum Gear Ratios: {}", sum_gear);
    }
}


fn unpack_line(line_option: Option<Result<String,io::Error>>) -> String {
    match line_option {
        Some(Ok(l)) => l,
        _ => {
            println!("Error parsing input file");
            return String::new();
        },
    }
}

fn find_symbols(line: &str) -> Vec<(usize, char)>{
    let re = Regex::new(r"[^\w\s.]").unwrap();
    re.find_iter(line).map(|m| (m.start(), m.as_str().chars().next().unwrap())).collect()

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
