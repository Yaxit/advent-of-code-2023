use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut part = 1;
    match args.len() {
        // path missing
        1 => {
            println!("Usage: main <file-path> <part> (1 or 2, default 1)");
            return;
        },
        // extra argument for the part
        3 => {
            part = match args[2].parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error parsing part number.");
                    return;
                }
            }
        },
        _ => {}
    }


    println!("Running part {}!", part);
    let path = &args[1];

    if let Ok(lines) = read_lines(path) {
        let mut sum = 0;
        for line in lines {
            // replace spelled numbers with digits in line
            let line = match part {
                1 => line,
                _ => line.map(|l|
                    l.replace("zero", "z0o")
                    .replace("one", "o1ne")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9ne"))
            };

            if let Ok(l) = line {
                let mut str = String::new();
                // split line in characters
                let chars = l.chars();
                for c in chars {
                    if c.is_digit(10){
                        str.push(c);
                        break;
                    }
                }
                let chars = l.chars();
                for c in chars.rev() {
                    if c.is_digit(10){
                        str.push(c);
                        break;
                    }
                }
                match str.parse::<i32>() {
                    Ok(num) => sum += num,
                    Err(_) => println!("Error parsing number."),
                }
            } else {
                println!("Error reading line.");
            }

        }
        println!("Sum: {}", sum)
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
