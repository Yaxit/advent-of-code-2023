use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

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

    let mut map:Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines{
            if let Ok(line) = line {
                let row = line.chars().collect::<Vec<char>>();
                map.push(row);
            }
        }
    }

    let galaxies = make_galaxy(&map, 2);
    println!("Part 1: {}", sum_distances(&galaxies));

    let galaxies = make_galaxy(&map, 1000000);
    println!("Part 2: {}", sum_distances(&galaxies));

}

fn make_galaxy(map:&Vec<Vec<char>>, growth_factor:usize) -> Vec<Point> {
    // store expansion points
    let mut empty_rows:Vec<usize> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        let empty = row.iter().all(|c| c == &'.');
        if empty {
            empty_rows.push(i);
        }
    }

    let mut empty_cols:Vec<usize> = Vec::new();
    for c in 0..map[0].len() {
        let empty = map.iter().all(|row| row[c] == '.');
        if empty {
            empty_cols.push(c);
        }
    }

    // make a galaxy map
    let mut y_factor = 0;
    let mut x_factor = 0;
    let mut galaxies:Vec<Point> = Vec::new();
    for (y, row) in map.iter().enumerate() {
        if empty_rows.contains(&y) {
            y_factor += growth_factor-1;
        }
        for (x, char) in row.iter().enumerate(){
            if empty_cols.contains(&x) {
                x_factor += growth_factor-1;
            }
            if char == &'#' {
                let p = Point {
                    x:x + x_factor,
                    y:y + y_factor,
                };
                galaxies.push(p);
            }
        }
        x_factor = 0;
    }
    galaxies
}

fn sum_distances(galaxies:&Vec<Point>) -> u64 {
    galaxies.iter().tuple_combinations().map(|(a,b)| {
        let distance  = (a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs();
        distance as u64
    }).sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
