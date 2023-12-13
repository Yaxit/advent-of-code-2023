use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;
use std::env;

use itertools::Itertools;

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

        let lines:Vec<_> = lines.collect_vec();
        let mut lines_iter = lines.iter();

        // find line that contains seeds
        let seeds = lines_iter.find(|line| line.as_ref()
                .unwrap_or(&String::new())
                .starts_with("seeds")).unwrap();
        let seeds = seeds.as_ref().unwrap().split_once(":").unwrap().1.split_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();



        // find line that contains map
        let mut map_vector:Vec<Vec<(isize, isize, isize)>> = Vec::new();
        while let Some(Ok(line)) = lines_iter.next() {
            if line.contains("map") {
                let map_iter = lines_iter.by_ref().map(|l| l.as_deref().unwrap())
                    .take_while(|l| !l.is_empty() && l.split_whitespace()
                        .take(3)
                        .all(|n| n.parse::<isize>().is_ok()))
                    .map(|l| l.split_whitespace()
                        .take(3)
                        .map(|n| n.parse::<isize>().unwrap())
                        .collect_tuple::<(isize,isize,isize)>().unwrap()
                        );
                map_vector.push(map_iter.collect_vec());
            }
        }
        let map_vector = map_vector;


        // define solution
        let solve = |seeds_ranges: Vec<Range<isize>>, map_vector:&Vec<Vec<(isize, isize, isize)>>| {
            let mut min_dist = 0;
            loop {
                let mut seed = min_dist;
                let map_it = map_vector.iter().rev();

                for map in map_it {
                    for (dst, src, len) in map.iter() {
                        if (*dst..(*dst+*len+1)).contains(&seed) {

                            seed = seed + (src - dst);
                            // inverst src and dst logic
                            break;
                        }
                    }
                }

                if seeds_ranges.iter().any(|r| r.contains(&seed)) {
                    println!("Found seed: {} at location {}", seed, min_dist);
                    break;
                }
                min_dist += 1;
            }
        };

        // funny enought, part 1 takes longer to solve this whay than directly
        let seed_ranges_1 = seeds.iter().map(|c| *c..*c+1).collect::<Vec<Range<isize>>>();
        println!("Part 1");
        solve(seed_ranges_1, &map_vector);
        println!();

        let seed_ranges_2 = seeds.chunks(2).map(|c| c[0]..c[0]+c[1]+1).collect::<Vec<Range<isize>>>();
        println!("Part 2");
        solve(seed_ranges_2, &map_vector);

    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
