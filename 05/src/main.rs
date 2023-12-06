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

        let mut seeds = lines.next().unwrap().unwrap()
            .split_once(":").map(|(_, s)| s.trim())
            .unwrap()
            .split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect::<Vec<isize>>();

        let mut seeds_copy = seeds.clone();

        for line in lines {
            if let Ok(l) = line {
                if !l.contains(":") {
                    // parse 3 numbers from line
                    let mut numbers = l.split_whitespace()
                        .map(|n| n.parse::<isize>().unwrap());

                    // separate numbers in tuple of three
                    if let (Some(dst), Some(src), Some(len)) = (numbers.next(), numbers.next(), numbers.next()) {
                        for (seed_src, seed_dst) in seeds.iter().zip(seeds_copy.iter_mut()) {
                            if (src..(src+len)).contains(&seed_src) {
                                *seed_dst = seed_src - (src - dst);
                            }
                        }
                    }
                }else{
                    seeds = seeds_copy;
                    seeds_copy = seeds.clone();
                }
            }
        }
        seeds = seeds_copy;
        // get lowest value in seeds
        let lowest = seeds.iter().min().unwrap();
        println!("Lowest: {}", lowest);

    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
