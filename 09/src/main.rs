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


    if let Ok(lines) = read_lines(path) {
        let mut acc:Vec<(i64,i64)> = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let nums:Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                acc.push(solve(&nums));
            }
        }
        println!();
        println!("Sum part 1: {}", acc.iter().fold(0, |acc, x| acc + x.0));
        println!("Sum part 2: {}", acc.iter().fold(0, |acc, x| acc + x.1));
    }
}

fn solve(nums: &Vec<i64>) -> (i64, i64) {
    if nums.iter().fold(0, |acc, x| acc + x) == 0 {
        return (0,0);
    }
    let diff:Vec<i64> = nums.windows(2).map(|x| x[1] - x[0]).collect();
    let deeper = solve(&diff);
    let res1 = nums.last().unwrap() + deeper.0;
    let res2 = nums.first().unwrap() - deeper.1;
    (res1 , res2)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
